#ifndef ch_memory_h
#define ch_memory_h
/* learn from redis/zmalloc */

#include "ch_types.h"			/* size_t */

void *malloc2(size_t size);
void *calloc2(size_t nmemb, size_t size);
void *calloc1(size_t size);
void *realloc2(void *ptr, size_t size);
void free2(void *ptr);
char *strdup2(const char *s);

#endif
