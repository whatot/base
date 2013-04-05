/*
 * =====================================================================================
 *
 *       Filename:  sigchld.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月16日 21时01分05秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <signal.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>

sig_atomic_t child_exit_status;

void clean_up_child_process (int signal_number)
{
	/* Clean up the child process.  */
	int status;
	wait (&status);
	/* Store its exit status in a golbal variable.  */
	child_exit_status = status;
}

int main(int argc, const char **argv)
{
	/* Handle SIGCHLD by calling clean_up_child_process.  */
	struct sigaction sigchld_action;
	memset (&sigchld_action, 0, sizeof (sigchld_action));
	sigchld_action.sa_handler = &clean_up_child_process;
	sigaction (SIGCHLD, &sigchld_action, NULL);

	/* Now do things, including forking a child process.  */
	/* ... */

	return 0;
}

// $ gcc -g -Wall -o sigchld sigchld.c
