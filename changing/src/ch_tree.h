/*
 * two implenments of tree
 * leaf-tree and node-tree
 *
 * node trees need like this
 * struct ntree_node_t {
 *		key_h key;
 *		struct ntree_node_t *left;
 *		struct ntree_node_t *right;
 *		// more possible information
 * };
 *
 * */

#ifndef ch_tree_h
#define ch_tree_h

#include "ch_types.h"

typedef void *nkey_t;

typedef struct ntree_node_t_ {
	nkey_t key;
	struct ntree_node_t_ *left;
	struct ntree_node_t_ *right;
	void *data;
	void *flags;
} ntree_node_t;
/*
 * flags may contain hidden,factor,...
 * using macros to access the inside of flags.
 * */

typedef struct ntree_t_ {
	size_t size;
	int (*compare) (const void *key1, const void *key2);
	void (*destroy) (void *data);
	ntree_node_t *root;
} ntree_t;
/*
 * compare function need behave like below:
 * key1 > key2 : return 1
 * key1 = key2 : return 0
 * key1 < key2 : return -1
 * So, maybe int is too large */

void ntree_init(ntree_t * tree,
				void (*compare) (const void *key1, const void *key2),
				void (*destroy) (void *data));
void ntree_destroy(ntree_t * tree);
void ntree_find(ntree_t * ntree, nkey_t key);
void ntree_insert(ntree_t * ntree, nkey_t key, void *data);
void ntree_delete(ntree_t * ntree, nkey_t key);

#define ntree_size(tree) ((tree)->size)
#define ntree_root(tree) ((tree)->root)
#define ntree_is_eob(node) ((node) == NULL)
#define ntree_is_leaf(node) ((node)->left == NULL && (node)->right == NULL)
#define ntree_data(node) ((node)->data)
#define ntree_left(node) ((node)->left);
#define ntree_right(node) ((node)->right);

#endif
