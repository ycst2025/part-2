#pragma once

#include <kernel.h>

#define STACK_SIZE 16384

#ifndef TOPPERS_MACRO_ONLY

extern void	main_task(EXINF exinf);

extern void rust_task(EXINF exinf);

#endif
