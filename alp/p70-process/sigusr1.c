/*
 * =====================================================================================
 *
 *       Filename:  sigusr1.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月10日 14时07分43秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

sig_atomic_t sigusr1_count = 0;

void handler (int signal_number)
{
	++sigusr1_count;
}

int main(int argc, const char **argv)
{
	struct sigaction sa;
	memset (&sa, 0, sizeof (sa));
	sa.sa_handler = &handler;
	sigaction (SIGUSR1, &sa, NULL);

	/* Do some lengthy stuff here. */
	/* ... */

	printf ("SIGUSR1 was raised %d times\n", sigusr1_count);

	return 0;
}
