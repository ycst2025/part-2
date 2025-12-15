#![no_std]
#![allow(non_camel_case_types)]

extern crate alloc;
#[allow(unused_imports)]
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec};

#[macro_use]
mod utils;
#[allow(unused_imports)]
use utils::spawn;

#[unsafe(no_mangle)]
extern "C" fn rust_task(_: usize) {
    // Initialize the heap used by `alloc`
    utils::init_heap();

    println!("rust_task is running");

    // See if heap-allocated data structures work
    let mut map = BTreeMap::new();
    map.insert(56, "fifty six");
    map.insert(5, "five");
    map.insert(100, "a hundred");
    dbg!(&map);

    // Create a task
    spawn(move || {
        println!("We're inside another task!");
        dbg!(&map[&56]);
    });
}

unsafe extern "C" {
    fn acre_flg(pk_cflg: *const T_CFLG) -> ER_ID;
}

#[repr(C)]
struct T_CFLG {
    flgatr: ATR,
    iflgptn: FLGPTN,
}

type int_t = core::ffi::c_int;
type uint_t = core::ffi::c_uint;
type ATR = uint_t;
type ER_ID = int_t;
type FLGPTN = uint_t;

const TA_WMUL: ATR = 0x02;
const TA_CLR: ATR = 0x04;
