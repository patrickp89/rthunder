/**
 A slim wrapper API around libcddb, that guarantees that
 there will be no memory leaks.
*/

#include <sys/types.h>
#include <cddb/cddb_disc.h>


/**
  Allocates heap memory for a CDDB disc struct.
*/
cddb_disc_t * cddb_wrapper_new_disc_t(void);
