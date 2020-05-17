/*
 * =====================================================================================
 *
 *       Filename:  cxx-exit.cpp
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月18日 12时32分38秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <pthread.h>

class ThreadExitException
{
	public:
		/* Create an exception-signaling thread exit with RETURN_VALUE. */
		ThreadExitException (void * return_value)
			: thread_return_value_ (return_value)
			{
			}

		/* actually exit the thread, using the return value provided in
		 * the constructor.  */
		void * DoThreadExit ()
		{
			pthread_exit (thread_return_value_);
		}

	private:
		/* The return value that will be used when exiting the thread.  */
		void * thread_return_value_;
};

void do_some_work ()
{
	while (1) {
		/* Do some useful things here...  */

		if (should_exit_thread_immediately ())
			throw ThreadExitException (
					/* thread's return value = */NULL);
	}
}

void * thread_function (void *)
{
	try {
		do_some_work ();
	}
	catch (ThreadExitException ex) {
		/* Some function indicated that we should exit the thread. */
		ex.DoThreadExit ();
	}
	return NULL;
}
