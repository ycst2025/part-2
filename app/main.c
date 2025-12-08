#include <kernel.h>
#include "syssvc/serial.h"
#include "main.h"

void
main_task(EXINF exinf)
{
  serial_wri_dat(1, "main_task is running\n", 21);
}

