/*
 * =====================================================================================
 *
 *       Filename:  cleanup.c
 *
 *    Description:  p99/368
 *
 *        Version:  1.0
 *        Created:  2013年02月17日 23时03分38秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <malloc.h>
#include <pthread.h>

/* Allocate a temporary buffer.  */
void * allocate_buffer (size_t size)
{
	return malloc (size);
}

/* Deallocate a temporary buffer.  */
void deallocate_buffer (void * buffer)
{
	free (buffer);
}

void do_some_work ()
{
	/* Allocate a temporary buffer.  */
	void * temp_buffer = allocate_buffer (1024);

	/* Register a cleanup handler for this buffer, to deallocate it in
	 * case the thread exits or is cancelled.  */
	pthread_cleanup_push (deallocate_buffer, temp_buffer);

	/* Do some work here that might call pthread_exit or might be cancelled. */

	/* Unregister the cleanup handler. Because we pass a nonzero value,
	 * this actually performs the cleanup by calling deallocate_buffer.  */
	pthread_cleanup_pop (1);
}
