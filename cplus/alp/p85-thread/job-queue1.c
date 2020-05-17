/*
 * =====================================================================================
 *
 *       Filename:  job-queue1.c
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  2013年02月18日 12时45分07秒
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  YOUR NAME (), 
 *   Organization:  
 *
 * =====================================================================================
 */

#include <malloc.h>

struct job {
	/* Link field for linked list.  */
	struct job* next;

	/* other fields describing work to be done...  */
};

/* A linked list of pending jobs.  */
struct job* job_queue;

/* process queued jobs until the queue is empty.  */

void * thread_function (void * arg)
{
	while (job_queue != NULL) {
		/* Get the next availbel job.  */
		struct job* next_job = job_queue;

		/* Remove this job from the list.  */
		job_queue = job_queue->next;

		/* Carry out the work.  */
		process_job (next_job);

		/* clean up.  */
		free (next_job);
	}

	return NULL;
}
