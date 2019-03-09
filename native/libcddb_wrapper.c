/**
 A slim wrapper API around libcddb, that guarantees that
 there will be no memory leaks.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "libcddb_wrapper.h"

#define FREE_NOT_NULL(p) if (p) { free(p); p = NULL; }


/**
  Logs a debug message to stdout.
*/
void log_debug(char * msg) {
    printf("[ libcddb_wrapper.c    ]  %s\n", msg);
}


/**
  Allocates heap memory for a CDDB disc struct.
*/
cddb_disc_t * cddb_wrapper_new_disc_t(void) {
    // TODO: ...
}
