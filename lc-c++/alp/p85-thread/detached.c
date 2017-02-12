/*
 * =====================================================================================
 *
 *       Filename:  detached.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月17日 15时21分12秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <pthread.h>

void * thread_function (void * thread_arg)
{
	/* Do work here...  */
}

int main(int argc, const char **argv)
{
	pthread_attr_t attr;
	pthread_t thread;
	
	pthread_attr_init (&attr);
	pthread_attr_setdetachstate (&attr, PTHREAD_CREATE_DETACHED);
	pthread_create (&thread, &attr, &thread_function, NULL);
	pthread_attr_destroy (&attr);

	/* Do work here...  */

	/* No need to join the second thread.  */
	return 0;
}
