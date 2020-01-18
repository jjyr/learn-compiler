#include <stdarg.h>
#include <stdlib.h>
#include <stdio.h>

void __attribute__((noreturn)) error(char * strfmt, ...)  {
    va_list argptr;
    va_start(argptr,strfmt);
    vprintf(strfmt, argptr);
    va_end(argptr);
    exit(-1);
}
