#ifndef SERVER_H
#define SERVER_H

#include <netinet/in.h>
#include <sys/types.h>

/* Symbols defined in common.c ****************************************/

/* The name of this program. */
extern const char* program_name;

/* If nonzero, print verbose messages. */
extern int verbose;

/* Like malloc, realloc, malloc-strcpy,
 * except aborts the program if allocation fails. */
extern void* xmalloc (size_t size);
extern void* xrealloc (void* ptr, size_t size);
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


/* Symbols defined in module.c ****************************************/

/* An instance of a loaded server module */
struct server_module {
	/* The shared library handle corresponding to the loaded module. */
	void* handle;
	/* A name descriptive the module */
	const char* name;
	/* The function that generates the HTML results for the this module */
	void (* generate_function) (int);
};

/* The directory from which modules are loaded */
extern char* module_dir;

/* Attempt to load a server module with the name MODULE_PATH. If a
 * server module exists with this path, loads the module and returns a
 * server_module structure representing it. Otherwise, return NULL. */
extern struct server_module* module_open (const char* module_open);

/* Close a server module and deallocate the MODULE object. */
extern void module_close (struct server_module* module);



/* Symbols defined in module.c ****************************************/

/* Run the server on LOCAL_ADDRESS and PORT. */
extern void server_run (struct in_addr local_address, uint16_t port);


#endif  /* SERVER_H */
