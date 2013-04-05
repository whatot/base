/* stack.c */
#include <stdlib.h>

#include "list.h"
#include "stack.h"

/* stack_push */
int stack_push(Stack *stack, const void *data){
	/* push the data onto the stack. */
	return list_ins_next(stack, NULL, data);
}

/* stack_pop */
int stack_pop(Stack *stack, void **data){
	/* pop the data off the stack. */
	return list_rem_next(stack, NULL, data);
}
