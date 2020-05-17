#ifndef ch_list_h
#define ch_list_h

/* circular list element and circular lists */
typedef struct clist_node_t_ {
	void *data;
	struct clist_node_t_ *next;
} clist_node_t;

typedef struct clist_t_ {
	int size;
	int (*match) (const void *key1, const void *key2);	/* unused */
	void (*destroy) (void *data);
	clist_node_t *head;
} clist_t;

void clist_init(clist_t * list, void (*destroy) (void *data));
void clist_destroy(clist_t * list);
int clist_ins_next(clist_t * list, clist_node_t * element, const void *data);
int clist_rem_next(clist_t * list, clist_node_t * element, void **data);
#define clist_size(list) ((list)->size)
#define clist_head(list) ((list)->head)
#define clist_data(element) ((element)->data)
#define clist_next(element) ((element)->next)

#endif
