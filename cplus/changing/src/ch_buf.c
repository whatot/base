#include "ch_buf.h"

#include <stdlib.h>
#include "ch_memory.h"

ch_buf_t *ch_buf_new(size_t buf_size_min, size_t buf_size_max)
{
	ch_buf_t *a_buf;

	if (buf_size_min == 0) {
		abort();
	}

	/* buf_size_max == 0 means infinity */
	if (buf_size_max != 0 && buf_size_max < buf_size_min) {
		abort();
	}

	a_buf = calloc2(1, sizeof(*a_buf));
	a_buf->min = buf_size_min;
	a_buf->max = buf_size_max;
	a_buf->size_real = buf_size_min;

	a_buf->start = calloc2(1, a_buf->size_real);

	a_buf->tail = a_buf->start;
	a_buf->end_plus_1 = a_buf->start + a_buf->size_real;
	a_buf->end = a_buf->end_plus_1 - 1;

	return a_buf;
}

void ch_buf_del(ch_buf_t * a_buf)
{
	if (a_buf->start) {
		free2(a_buf->start);
	}
	free2(a_buf);
	return;
}

int ch_buf_vprintf(ch_buf_t * a_buf, const char *format, va_list args);
int ch_buf_append(ch_buf_t * a_buf, const char *str, size_t str_len);
