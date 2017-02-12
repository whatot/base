/* ch_list.c for clist and more */
#include "ch_list.h"

#include <string.h>
#include "ch_memory.h"

void clist_init(clist_t * list, void (*destroy) (void *data))
{
	list->size = 0;
	list->destroy = destroy;
	list->head = NULL;

	return;
}

void clist_destroy(clist_t * list)
{
	void *data;

	/* Remove each element. */
	while (clist_size(list) > 0) {
		if (clist_rem_next(list, list->head, (void **)&data) == 0 &&
			list->destroy != NULL) {
			list->destroy(data);
		}
	}

	memset(list, 0, sizeof(clist_t));

	return;
}

/* function malloc memory, data will be copyed. */
int clist_ins_next(clist_t * list, clist_node_t * element, const void *data)
{
	clist_node_t *new_element;

	new_element = (clist_node_t *) malloc2(sizeof(clist_node_t));

	new_element->data = (void *)data;
	if (clist_size(list) == 0) {	/* empty list */
		new_element->next = new_element;
		list->head = new_element;
	} else {
		new_element->next = element->next;
		element->next = new_element;
	}

	list->size++;

	return 0;
}

/* function free unused node, data contain the removed node's data. */
int clist_rem_next(clist_t * list, clist_node_t * element, void **data)
{
	clist_node_t *old_element;

	if (clist_size(list) == 0) {
		return -1;
	}

	*data = element->next->data;
	if (element->next == element) {	/* the last element */
		old_element = element->next;
		list->head = NULL;
	} else {
		old_element = element->next;
		element->next = element->next->next;
		if (old_element == clist_head(list)) {
			list->head = old_element->next;
		}
	}

	free2(old_element);

	list->size--;

	return 0;
}
