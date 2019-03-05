/**
 A slim wrapper API around libcdio, that guarantees that
 there will be no memory leaks.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "libcdio_wrapper.h"

#define FREE_NOT_NULL(p) if (p) { free(p); p = NULL; }


/**
  Logs a debug message to stdout.
*/
void log_debug(char * msg) {
    printf("[ libcdio_wrapper.c    ]  %s\n", msg);
}


/**
  Finds the system's default cdrom device.
*/
char * cdio_wrapper_get_default_device() {
    log_debug("cdio_get_default_device()...");
    char *device = cdio_get_default_device(NULL);
    printf("[ libcdio_wrapper.c    ]  device: %s\n", device);
    return device;
}


/**
  Frees a C string.
*/
void cdio_wrapper_free_string(char * p) {
    log_debug("Freeing pointer...");
    FREE_NOT_NULL(p);
    log_debug("Done!");
}


/**
  Opens a given CD device for reading.
*/
CdIo_t * cdio_wrapper_open_device(char * device) {
    log_debug("Opening device...");
    CdIo_t *cdio = cdio_open(device, DRIVER_UNKNOWN);
    return cdio;
}


/**
  Destroys the underlying CDIO environment.
*/
void cdio_wrapper_destroy_cdio_env(CdIo_t * cdio) {
    log_debug("Destroying CDIO environment...");
    cdio_destroy(cdio);
    log_debug("Done!");
}
