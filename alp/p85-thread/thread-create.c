/*
 * =====================================================================================
 *
 *       Filename:  thread-create.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月16日 22时17分24秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <pthread.h>
#include <stdio.h>

/* Prints x's to stderr. The parameter is unused. Does not return.  */

void * print_xs (void * unused)
{
	while (1)
		fputc ('x', stderr);
	return NULL;
}

/* The main program.  */

int main(int argc, const char **argv)
{
	pthread_t thread_id;
	/* Create a new thread. The new thread will run the print_xs function. */
	pthread_create (&thread_id, NULL, &print_xs, NULL);
	/* Print o's continuously to stderr.  */
	while (1)
		fputc ('o', stderr);

	return 0;
}

// $ cc -g -Wall -o thread-create thread-create.c -lpthread
// use pthread_exit(exit_code) function can exit.
