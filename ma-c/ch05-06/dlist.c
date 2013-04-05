/* dlist.c */
#include <stdlib.h>
#include <string.h>

#include "dlist.h"

/* dlist_init */
void dlist_init(DList *list, void (*destroy)(void *data)){
	/* Initialize the list */

	list->size = 0;
	list->destroy = destroy;
	list->head = NULL;
	list->tail = NULL;

	return;
}

/* dlist_destroy */
void dlist_destroy(DList *list){
	void *data;

	/* Remove each element. */
	while (dlist_size(list) > 0) {
		if (dlist_remove(list, dlist_tail(list), (void **)&data) == 0
		    && list->destroy != NULL) {
			list->destroy(data);
		}
	}

	/* No operation are allowed now, but clear the structure 
	 * as a precaution. */
	memset(list, 0, sizeof(DList));

	return;
}

/* dlist_ins_next */
int dlist_ins_next(DList *list, DListElmt *element, const void *data){
	DListElmt *new_elelment;

	/* Don't allow a NULL element unless the list is empty. */
	if (element == NULL && dlist_size(list) != 0) {
		return -1;
	}

	/* Allocate storage for the element. */
	if ((new_elelment = (DListElmt *)malloc(sizeof(DListElmt))) == NULL) {
		return -1;
	}

	/*  Insert the new element into the list. */
	new_elelment->data = (void *) data;

	if (dlist_size(list) == 0) {
		/* Handle insertion when the list is empty. */
		list->head = new_elelment;
		list->head->prev = NULL;
		list->head->next = NULL;
		list->tail = new_elelment;
	}
	else {
		/* Handle insertion when the list is not empty. */
		new_elelment->next = element->next;
		new_elelment->prev = element;

		if (element->next == NULL) {
			list->tail = new_elelment;
		} else {
			element->next->prev = new_elelment;
		}
	element->next = new_elelment;
	}

	/* Adjust the size of the list to the account for the inserted element. */
	list->size++;

	return 0;
}

/* dlist_ins_prev */
int dlist_ins_prev(DList *list, DListElmt *element, const void *data){
	DListElmt *new_elelment;

	/* Don't allow a NULL element unless the list is empty. */
	if (element == NULL && dlist_size(list) != 0) {
		return -1;
	}

	/* Allocate storage to be managed by the abstract datetype. */
	if ((new_elelment = (DListElmt *)malloc(sizeof(DListElmt))) == NULL) {
		return -1;
	}

	/* Insert the new element into the list. */
	new_elelment->data = (void *)data;
	if (dlist_size(list) == 0) {
		/* Handle insertion when the list is empty */
		list->head = new_elelment;
		list->head->prev = NULL;
		list->head->next = NULL;
		list->tail = new_elelment;
	} else {
		/* Handle insertion when the list is not empty. */
		new_elelment->next = element;
		new_elelment->prev = element->prev;

		if (element->prev == NULL) {
			list->head = new_elelment;
		} else {
			element->prev->next = new_elelment;
		}
		element->prev = new_elelment;
	}

	/* Adjust the size of list to account for the new element. */
	list->size++;

	return 0;
}

/* dlist_remove */
int dlist_remove(DList *list, DListElmt *element, void **data){
	/* Don't allow a NULL element or remoral from an empty list. */
	if (element == NULL && dlist_size(list) == 0) {
		return -1;
	}

	/* Remove the element from the list. */
	*data = element->data;

	if (element == list->head) {
		/* Handle remoral from the head of list. */
		list->head = element->next;

		if (list->head == NULL) {
			list->tail = NULL;
		} else {
			element->next->prev = NULL;
		}
	} else {
		/* Handle remoral from other than the head of the list. */
		element->prev->next = element->next;
		if (element->next == NULL) {
			list->tail = element->prev;
		} else {
			element->next->prev = element->prev;
		}
	}

	/* Free the storage allocated by the abstract datetype. */
	free(element);

	/* Adjust the size of list to account for the removed element. */
	list->size--;

	return 0;
}
