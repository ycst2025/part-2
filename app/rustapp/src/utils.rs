//! Utility functions and macros to ease debugging
#![allow(dead_code)]
#![allow(unused_macros)]
use alloc::boxed::Box;
use core::{
    alloc::Layout,
    fmt::Write,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};
use embedded_alloc::TlsfHeap;

unsafe extern "C" {
    fn serial_wri_dat(portid: i32, buf: *const u8, len: u32) -> i32;

    #[cfg(not(test))]
    fn ext_ker() -> !;

    fn loc_cpu() -> i32;
    fn unl_cpu() -> i32;
    fn sns_loc() -> i32;
}

pub(crate) struct SerialWrite;

impl Write for SerialWrite {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { serial_wri_dat(1, s.as_ptr(), s.len() as u32) };
        Ok(())
    }
}

/// Print to the serial port.
macro_rules! print {
    ($($tt:tt)*) => {
        ::core::fmt::Write::write_fmt(
            &mut crate::utils::SerialWrite,
            ::core::format_args!($($tt)*),
        )
        .unwrap()
    };
}

/// Print to the serial port, with a newline.
macro_rules! println {
    ($($tt:tt)*) => {
        ::core::fmt::Write::write_fmt(
            &mut crate::utils::SerialWrite,
            ::core::format_args!(
                "{}\n",
                ::core::format_args!($($tt)*),
            )
        )
        .unwrap()
    };
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging.
///
/// This is based on `std::dbg!`.
macro_rules! dbg {
    () => {
        println!("[{}:{}:{}]", ::core::file!(), ::core::line!(), ::core::column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                println!("[{}:{}:{}] {} = {:#?}",
                    ::core::file!(),
                    ::core::line!(),
                    ::core::column!(),
                    ::core::stringify!($val),
                    &&tmp as &dyn ::core::fmt::Debug,
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}\n");
    unsafe { ext_ker() };
}

// Heap
// ------------------------------------------------------------------------

#[global_allocator]
static HEAP: TlsfHeap = TlsfHeap::empty();

pub(crate) fn init_heap() {
    static INITED: AtomicBool = AtomicBool::new(false);
    if INITED.swap(true, Relaxed) {
        return;
    }

    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 64 * 1024 * 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
}

// `critical_section` implementation
// ------------------------------------------------------------------------

struct MyCriticalSection;
critical_section::set_impl!(MyCriticalSection);

unsafe impl critical_section::Impl for MyCriticalSection {
    #[inline]
    unsafe fn acquire() -> critical_section::RawRestoreState {
        if unsafe { sns_loc() } != 0 {
            true
        } else {
            unsafe { loc_cpu() };
            false
        }
    }

    #[inline]
    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        if !restore_state {
            unsafe { unl_cpu() };
        }
    }
}

// Spawn task
// ------------------------------------------------------------------------

/// Create and activate a task, using `f` as the entry point.
///
/// Warning: The created task is never deleted, even after the provided closure
/// returns. If you call
#[inline]
pub fn spawn<F>(f: F) -> i32
where
    F: FnOnce() + Send + 'static,
{
    unsafe extern "C" {
        fn acre_tsk(pk_ctsk: *const T_CTSK) -> i32;
    }

    #[repr(C)]
    struct T_CTSK {
        tskatr: u32,
        exinf: *mut (),
        task: unsafe extern "C" fn(*mut ()),
        pri: i32,
        stksz: usize,
        stk: *mut u8,
    }

    unsafe extern "C" fn trampoline<F>(exinf: *mut ())
    where
        F: FnOnce() + Send + 'static,
    {
        let f_store: &mut Option<F> = unsafe { &mut *exinf.cast() };
        let f = f_store
            .take()
            .expect("spawned task's entry point was called twice");
        f();
    }

    unsafe {
        // Move `f` into heap
        let f_store = Box::into_raw(Box::new(Some(f)));

        // Allocate stack from heap
        let stksz = 16384;
        let stk_layout = Layout::from_size_align(stksz, 16).unwrap();
        let stk = alloc::alloc::alloc(stk_layout);

        let id = acre_tsk(&T_CTSK {
            tskatr: 1,
            exinf: f_store.cast(),
            task: trampoline::<F>,
            pri: 5,
            stksz,
            stk,
        });

        if id < 0 {
            alloc::alloc::dealloc(stk, stk_layout);
            drop(Box::from_raw(f_store));
            panic!("acre_tsk failed: {id}")
        } else {
            id
        }
    }
}
