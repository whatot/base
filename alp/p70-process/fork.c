/*
 * =====================================================================================
 *
 *       Filename:  fork.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月09日 19时24分21秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdio.h>
#include <sys/types.h>
#include <unistd.h>

int main(int argc, const char **argv)
{
	pid_t child_pid;
	printf ("the main program process ID is %d\n", (int) getpid ());

	child_pid = fork ();
	if (child_pid != 0) {
		printf ("this is the parent process, with id %d\n", (int) getpid());
		printf ("the child's process ID is %d\n", (int) getpid());
	} else {
		printf ("this is the child process, with id %d\n", (int) getpid());
	}

	return 0;
}
