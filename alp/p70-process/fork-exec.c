/*
 * =====================================================================================
 *
 *       Filename:  fork-exec.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月09日 19时46分41秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>


/* Spawn a child process running a new program. PROGRAM is the name
 * of the program to run; the path will be searched for this program.
 * ARG_LIST is a NULL-terminated list of character strings to be passes
 * as the program's argument list. Returns the peocess ID of the
 * spawned process. */

int spawn (char* program, char** arg_list)
{
	pid_t child_pid;
	/* Duplicate this process. */
	child_pid = fork ();
	if (child_pid != 0)
		/* This is the parent process. */
		return child_pid;
	else {
		/* Now execute PROGRAM, searching for it in the path. */
		execvp (program, arg_list);
		/* The execvp function returns only if an error occurs. */
		fprintf(stderr, "an error occured in execvp\n");
		abort ();
	}
}

int main(int argc, const char **argv)
{
	/* The argument list to pass to the "ls" command.  */
	char * arg_list[]= {
		"ls", 	/* argv[0], the name of the program.*/
		"-l",
		"/",
		NULL 	/* The argument list must end with a NULL. */
	};

	/* Spawn a child process running the "ls" command. Ingnore the
	 * returned child process ID.  */
	spawn ("ls", arg_list);

	printf ("done with main program\n");

	return 0;
}
