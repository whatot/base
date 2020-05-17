#ifndef ch_status_h
#define ch_status_h

#include "changing.h"
#include "ch_vector.h"			/* vec3 */

typedef struct ch_status_ {
	vec3 p;						/* position */
} ch_status;

int ch_status_init();

#endif
