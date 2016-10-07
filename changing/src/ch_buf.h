#ifndef ch_buf_h
#define ch_buf_h

#include "ch_types.h"			/* size_t, va_list */

/* from zlog/src/buf.h */
typedef struct {
	char *start;
	char *tail;
	char *end;
	char *end_plus_1;

	size_t min;
	size_t max;
	size_t size_real;
} ch_buf_t;

ch_buf_t *ch_buf_new(size_t buf_size_min, size_t buf_size_max);
void ch_buf_del(ch_buf_t * a_buf);

int ch_buf_vprintf(ch_buf_t * a_buf, const char *format, va_list args);
int ch_buf_append(ch_buf_t * a_buf, const char *str, size_t str_len);

#define ch_buf_restart(a_buf) do { \
	a_buf->tail = a_buf->start; \
} while(0)

#define ch_buf_len(a_buf) (a_buf->tail - a_buf->start)
#define ch_buf_start(a_buf) (a_buf->start)

#endif
