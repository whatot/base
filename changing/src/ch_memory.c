#include "ch_memory.h"

#include <assert.h>				/* assert */
#include <stdlib.h>				/* malloc,calloc,realloc,free */
#include <string.h>				/* strlen,memcpy */

void *malloc2(size_t size)
{
	void *ptr = malloc(size);
	assert(ptr);

	return ptr;
}

void *calloc2(size_t nmemb, size_t size)
{
	void *ptr = calloc(nmemb, size);
	assert(ptr);

	return ptr;
}

void *calloc1(size_t size)
{
	void *ptr = calloc(1, size);
	assert(ptr);

	return ptr;
}

void *realloc2(void *ptr, size_t size)
{
	void *newptr = NULL;

	if (!ptr) {
		newptr = malloc2(size);
	} else {
		newptr = realloc(ptr, size);
		assert(newptr);
	}

	return newptr;
}

void free2(void *ptr)
{
	if (!ptr) {
		return;
	} else {
		free(ptr);
		ptr = NULL;
	}
}

char *strdup2(const char *s)
{
	size_t l = strlen(s) + 1;
	char *p = malloc2(l);

	memcpy(p, s, l);
	return p;
}
