/*
 * =====================================================================================
 *
 *       Filename:  thread-create2.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月17日 10时29分39秒
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

/* Parameters to print_function.  */

struct char_print_parms
{
	/* The character to print.  */
	char character;
	/* The number of times to print it.  */
	int count;
};

/* Prints a number of characters to stderr, as given by Parameters,
 * which is a pointer to a struct char_print_parms.  */

void * char_print (void * parameters)
{
	/* Cast the cookie pointer to the right type.  */
	struct char_print_parms * p = (struct char_print_parms *) parameters;
	int i;

	for(i = 0; i < p->count; ++i) {
		fputc (p->character, stderr);
	}
	return NULL;
}

/* The main program.  */

int main(int argc, const char **argv)
{
	pthread_t thread1_id;
	pthread_t thread2_id;
	struct char_print_parms thread1_args;
	struct char_print_parms thread2_args;

	/* Create a new thread to print 300 'x's.  */
	thread1_args.character = 'x';
	thread1_args.count = 300;
	pthread_create (&thread1_id, NULL, &char_print, &thread1_args);

	/* Create a new thread to print 200 'o's.  */
	thread2_args.character = 'o';
	thread2_args.count = 200;
	pthread_create (&thread2_id, NULL, &char_print, &thread2_args);

	/* Make sure the first thread has finished.  */
	pthread_join (thread1_id, NULL);
	/* Make sure the second thread has finished.  */
	pthread_join (thread2_id, NULL);

	/* Now we can safely return.  */
	return 0;
}

/* 之前不存在66-69行,导致无法正常安全退出.
 * pthread_join()的第二个参数是void*型 */
