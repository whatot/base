/*
 * =====================================================================================
 *
 *       Filename:  zombie.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月10日 17时13分42秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>
#include <stdio.h>

int main(int argc, const char **argv)
{
	pid_t child_pid;

	/* Create a child process.  */
	child_pid = fork ();
	if (child_pid > 0) {
		printf ("This is parent process,pid: %d\n", (int) getpid());
		sleep (20);
		printf ("This is parent process,sleep over.\n");
	} else {
		printf ("This is child process, pid: %d\n", (int) getpid());
		printf ("Exit immediately.\n");
		exit (0);
	}

	return 0;
}

// $ gcc -g -Wall -o make-zombie zombie.c
// $ ps -e -o pid,ppid,stat,cmd
