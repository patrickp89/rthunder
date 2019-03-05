/**
 A slim wrapper API around libcdio, that guarantees that
 there will be no memory leaks.
*/

#include <sys/types.h>
#include <cdio/cdio.h>


/**
  Finds the system's default cdrom device.
*/
char * cdio_wrapper_get_default_device();


/**
  Frees a C string.
*/
void cdio_wrapper_free_string(char * p);


/**
  Opens a given CD device for reading.
*/
CdIo_t * cdio_wrapper_open_device(char * device);


/**
  Destroys the underlying CDIO environment.
*/
void cdio_wrapper_destroy_cdio_env(CdIo_t * cdio);
