/* queue.c */
#include <stdlib.h>

#include "list.h"
#include "queue.h"

/* queue_enqueue */
int queue_enqueue(Queue *queue, const void *data){
	/* enqueue the data. */
	return list_ins_next(queue, list_tail(queue), data);
}

/* queue_dequeue */
int queue_dequeue(Queue *queue, void **data){
	/* dequeue the data. */
	return list_rem_next(queue, NULL, data);
}
