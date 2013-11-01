#ifndef SERVER_H
#define SERVER_H

#include <netinet/in.h>
#include <sys/types.h>

/* Symbols defined in common.c */

/* The name of this program. */
extern const char* program_name;

/* If nonzero, print verbose messages. */
extern int verbose;

/* Like malloc, realloc, strdup,
 * except aborts the program if allocation fails. */
extern void* xmalloc (size_t size);
extern void xrealloc (void* ptr, size_t size);
extern char* xstrdup (const char* s);

/* Print an error message for a failed call OPERATION, using the value
 * of errno, and end the program. */
extern void system_error (const char* operation);

/* Print an error message for failure invalving CAUSE, including a
 * descriptive MESSAGE, and end the program. */
extern void error (const char* cause, const char* message);

/* Return the directory containing the running program's executable.
 * The return value is a memory buffer that the caller must deallocate
 * using free. This function calls abort on failure. */
extern char* get_self_executable_directory ();


#endif
