#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "server.h"

const char *program_name;

int verbose;

#define CHECK_PTR_NULL_ABORT(x) \
	if (x == NULL) { abort (); }

void *xmalloc(size_t size)
{
	void *ptr = malloc(size);

	CHECK_PTR_NULL_ABORT(ptr);
	return ptr;
}

void *xrealloc(void *ptr, size_t size)
{
	ptr = realloc(ptr, size);

	CHECK_PTR_NULL_ABORT(ptr);
	return ptr;
}

/* the copy need be free by hand. */
char *xstrdup(const char *s)
{
	char *copy = NULL;
	if (s && (copy = (char *)malloc((strlen(s) + 1)))) {
		strcpy(copy, s);
	}

	CHECK_PTR_NULL_ABORT(copy);
	return copy;
}

void error(const char *cause, const char *message)
{
	/* Print an error message to strerr */
	fprintf(stderr, "%s: error: (%s) %s\n", program_name, cause, message);
	exit(1);
}

void system_error(const char *operation)
{
	/* Generate an error message for errno */
	error(operation, strerror(errno));
}

char *get_self_executable_directory()
{
	int rval = 0;
	char link_target[1024];
	char *last_slash = NULL;
	size_t result_length;
	char *result = NULL;

	/* Read the target of the symbolic link /proc/self/exe */
	rval = readlink("/proc/self/exe", link_target, sizeof(link_target) - 1);
	if (rval == -1) {
		abort();
	} else {
		/* NUL-terminate the target */
		link_target[rval] = '\0';
	}

	/* We want to trim the name of the executable file, to obtain the
	 * directory that contains it. Find the rightmost slash. */
	last_slash = strrchr(link_target, '/');
	if (last_slash == NULL || last_slash == link_target) {
		abort();
	}

	/* Allocate a buffer to hold the resulting path. */
	result_length = last_slash - link_target;
	result = (char *)xmalloc(result_length + 1);
	strncpy(result, link_target, result_length);
	result[result_length] = '\0';
	return result;
}
