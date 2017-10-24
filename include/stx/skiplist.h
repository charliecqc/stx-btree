#ifndef STX_SKIPLIST_HEADER
#define STX_SKIPLIST_HEADER
#include <algorithm>
#include <functional>
#include <istream>
#include <ostream>
#include <memory>
#include <stdlib.h>
#include <assert.h>
#include <immintrin.h>
#include <bitset>
#include <climits>
//#define DEBUG
//#define DEBUG_CLONE


#define EXPECT_TRUE(x)	__builtin_expect(!!(x), 1)
#define EXPECT_FALSE(x)	__builtin_expect(!!(x), 0)

#define CAS_EXPECT_DOES_NOT_EXIST (0)
#define CAS_EXPECT_EXISTS		(-1)
#define	CAS_EXPECT_WHATEVER		(-2)
#define SYNC_SWAP(addr, x)		__sync_lock_test_and_set(addr, x)
#define SYNC_CAS(addr, old, x)	__sync_val_compare_and_swap(addr, old, x)
#define SYNC_ADD(addr, n)		__sync_add_and_fetch(addr, n)
#define SYNC_FETCH_AND_OR		__sync_fetch_and_or(addr, x)

#define MAX_LEVELS 32 
#define TAG_VALUE(v, tag) ((v) | tag)
#define IS_TAGGED(v, tag) ((v) & tag)
#define STRIP_TAG(v, tag) ((v) & ~tag)

typedef size_t markable_t;

//Marking the <next> field of a node logically removes it from the list
#define MARK_NODE(x) TAG_VALUE(reinterpret_cast<markable_t>(x), 0x1)
#define HAS_MARK(x) (IS_TAGGED((x), 0x1) == 0x1)
#define GET_NODE(x) (reinterpret_cast<node_t *>(x))
#define STRIP_MARK(x) (reinterpret_cast<node_t *>(STRIP_TAG((x), 0x1)))

enum unlink {
	FORCE_UNLINK,
	ASSIST_UNLINK,
	DONT_UNLINK
};

namespace stx {
	template <typename _Key>
		class skiplist_default_set_traits
		{
			static const bool selfverify = false;
			static const bool debug = false;
			static const int leafslots = 256;
		};

	template <typename _Key,  typename _Value, typename _Traits = skiplist_default_set_traits<_Key> >	
		class skiplist
		{
			public:

				typedef _Key key_type;

				typedef _Value value_type;

				typedef _Traits traits;

				typedef int (*cmp_func_t) (void *, void *);

				typedef void * (*clone_fun_t) (void *);

				typedef int64_t (*hash_fun_t) (void *);

				typedef struct datatype {
					cmp_func_t cmp;
					hash_fun_t hash;
					clone_fun_t clone;
				}datatype_t;

			public:
				/// Typedef of our own type
				typedef skiplist<key_type, value_type, traits> self_type;

				/// Size type usedjkj to count keys
				typedef size_t size_type;

				typedef std::pair<key_type, value_type> pair_type;
			public:

				///Base Skiplist parameter: The numener of key/value slots in each leaf
				static const unsigned short leafslotmax = traits::leafslots;

			private:
				static const int rs = 12345678;

			public:
				typedef struct leaf {
					///Double linked list pointers to traverse the leaves
					//					struct leaf *prevleaf;	
					///Double linked list pointers to traverse the leaves
					//					struct leaf *nextleaf;
					std::bitset<leafslotmax> bs;
					///array of key
					key_type slotkey[leafslotmax];
					///array of data
					value_type slotvalue[leafslotmax];
					///current count of used key
					//
					public:
					int slotused;


					///set variable to initial values
					leaf()
					{
						bs.reset(); // to set each bit to 0
						slotused = 0;
#ifdef DEBUG
						cout << " new leaf is created " << this << " with slotused "<< slotused << endl;
#endif
					}

					///True if the node's slots are full
					inline bool isfull() const
					{
						//	return (slotused == leafslotmax - 1);
						//	return bs.count() == bs.size();
						return slotused == leafslotmax;
					}
					///set the indexth bit to 1 
					int set_bitmap(unsigned short index)
					{
						bs[index] = 1;
						return 0;
					}

					///set the index th bit to 0 
					int clear_bitmap(unsigned short index) 
					{
						bs[index] = 0;
						return 0;
					}
					///return index th bit's value
					int get_bit(unsigned short index)
					{
						return bs[index];
					}

					///return the first bit that was set to 1, from left to right
					int get_first_non_zero_bit()
					{
						for(int index = 0; index < leafslotmax; index++) {
							if(bs.test(index))
								return index;
						}
					}

					///return the first bit that was 0. from left to right
					int get_first_free_bit()
					{
						for(int index = 0; index < leafslotmax; index++) {
							if(!bs.test(index))
								return index;
						}
						return -1;
					}

#if 0
					inline int set(key_type key, value_type value) {
						markable_t index = get_first_free_bit();
						slotkey[index] = key;
						slotvalue[index] = value;
						set_bitmap(index);
					}
#endif

					int hash(key_type key, int k) {
						return (key + k * (1 + (((key >> 5) + 1) % (leafslotmax - 1)))) % leafslotmax;	
					}

					int set_hash(key_type key, value_type value)
					{
						for(int i = 0; i < leafslotmax; i++) {
							int index = hash(key, i);
							if(!bs.test(index)){
#ifdef DEBUG
								cout << "insert key at " << index << " i is " << i << " "<<this<< endl;
#endif
								slotkey[index] = key;
								slotvalue[index] = value;
								set_bitmap(index);
								return index;
							}else if(slotkey[index] == key) {
								slotvalue[index] = value;
								return index;
							}
						}
						return -1;
					}

					inline int set(key_type key, value_type value)
					{
#ifdef DEBUG
						cout <<this << " 's slotused is " << slotused <<  endl;
#endif
						//		if(slotkey[slotused] == key) {
						//			slotvalue[slotused] = value;
						//			return 0;
						//		cout <<this << " 's slotused keep unchanged due to same key" << endl;
						//		}

						int index =	slotused++;
#ifdef DEBUG
						cout <<this << " 's slotused is set to be " << slotused <<  endl;
#endif
						slotkey[index] = key;
						slotvalue[index] = value;
						return 0;
					}

					value_type get(key_type key) {
						for(int i = leafslotmax - 1; i >= 0; i--) {
							int index = hash(key, i);
							if(!bs.test(index)) {
								if(slotkey[index] == key){
									return slotvalue[index];
								}
							}
						}
						return -1;
					}
				}leaf_t;

				typedef struct node {
					key_type max;  //max key in its data node
					key_type min; //minimum key in its data node
					key_type sum; //sum of all the key in the leaf node
					unsigned num_levels;
					markable_t next[MAX_LEVELS];
					leaf_t *leaf_ptr;
				} node_t;

				struct sl_iter {
					node_t *next;
				};

				typedef struct sl {
					node_t *head;
					const datatype_t *type;
					int high_water;	//max historic number of levels
				} skiplist_t ;

			public:
				int random_levels (skiplist_t *sl) {
					double r = rand()/(RAND_MAX + 1.0);
					int levels = 1;
					while(r < 0.15 && levels < MAX_LEVELS) {
						levels++;
						r = rand()/(RAND_MAX + 1.0);
					}
					if(levels > sl->high_water){
						levels = sl->high_water;
						sl->high_water++;
					}
					if(sl->high_water >= MAX_LEVELS)
						sl->high_water = MAX_LEVELS;

#ifdef DEBUG
					cout << " s2 random_levels: increased high water mark to " << sl->high_water << endl;
#endif
					return levels;

				}

				// Allocate a new leaf node
				leaf_t *leaf_alloc() {
					size_t sz = sizeof(leaf_t);		
					//	leaf_t *item = reinterpret_cast<leaf_t *>(malloc(sz));
					leaf_t *item = new leaf_t(); 
					memset(item, 0, sz);
					return item;
				}

				node_t *node_alloc(skiplist_t *sl, int num_levels, key_type max, key_type min = ULLONG_MAX, bool is_head = false, key_type sum = 0) {
					assert(num_levels >= 0 && num_levels <= MAX_LEVELS);
					size_t sz = sizeof(node_t) + (num_levels - 1) * sizeof(node_t *);
					node_t *item = reinterpret_cast<node_t *>(malloc(sz)); //todo use new memory allocator later
					memset(item, 0, sz);
					item->max = max;
					if(min == ULLONG_MAX)
						item->min = max;
					else
						item->min = min;
					item->num_levels = num_levels;
					item->leaf_ptr = NULL;
					item->sum = sum;
#if 0
					if(!is_head){
						if(sl->high_water < num_levels)
							sl->high_water++;
						if(sl->high_water > MAX_LEVELS)
							sl->high_water = MAX_LEVELS;
					}
#endif
#ifdef DEBUG 
					cout << "s2 node_alloc : new node " << item << " "<< num_levels << " levels" << endl;
#endif
					return item;
				}

				skiplist_t *sl_alloc (const datatype_t *type) {
					skiplist_t *sl = static_cast<skiplist_t *>(malloc(sizeof(skiplist_t)));
					sl->type = type;
					sl->high_water = 1;
					sl->head = node_alloc(sl, MAX_LEVELS, 0, 0, true);
					memset(reinterpret_cast<void *>(sl->head->next), 0, MAX_LEVELS * sizeof(skiplist_t *));
					return sl;
				}

				///return a pointer to the splited new data node
				//key will be used to make a new index node
				//@min_key: minimum key stored in the old_leaf 
				//@max_key: maximum key stored in the new_leaf 
				leaf_t * split_leaf_node_bitmap(key_type *max_key, key_type *min_key,const key_type target_key, leaf_t *old_leaf)	{
					struct leaf *new_leaf = new leaf_t();

					memcpy(new_leaf, old_leaf, sizeof(struct leaf));
					for(int index = 0; index < new_leaf->bs.size(); index++) {
						///invalidate the keys which are larger than target_key in the new data node
						if(new_leaf->slotkey[index] >= target_key)
							new_leaf->clear_bitmap(index);	
						///calculate the largest key in the data node
						if(new_leaf->bs.test(index) && new_leaf->slotkey[index] >= *max_key)
							*max_key = new_leaf->slotkey[index];
						///invalidate the keys which are smaller than target_key in the old data node
						if(old_leaf->slotkey[index] < target_key)
							old_leaf->clear_bitmap(index);
#if 0
						if(!new_leaf->bs.test(index) && !old_leaf->bs.test(index))	
							continue;
#endif
						///calculate the smallest key in the dat node
						if(old_leaf->bs.test(index) && old_leaf->slotkey[index] <= *min_key)
							*min_key = old_leaf->slotkey[index];
					}
					return new_leaf;
				}

				inline leaf_t *split_leaf_node(key_type *max_key, key_type *min_key, const key_type target_key, leaf_t *old_leaf, leaf_t *orig_leaf, key_type *new_sum, key_type *orig_sum) {
					struct leaf *new_leaf = new leaf_t();
					for(int index = 0; index < old_leaf->slotused; index++){
						key_type temp_key = old_leaf->slotkey[index];
						key_type temp_value = old_leaf->slotvalue[index];
#ifdef DEBUG
						cout << "target key "<< target_key << " temp_key " << temp_key << endl;
#endif	
						if(temp_key <= target_key ) {
							new_leaf->slotkey[new_leaf->slotused] = temp_key;
							new_leaf->slotvalue[new_leaf->slotused] = temp_value;
							if(temp_key >= *max_key){
								*max_key = temp_key;
							}
							*new_sum += temp_key;
							new_leaf->slotused++;
						}else{
							orig_leaf->slotkey[orig_leaf->slotused] = temp_key;
							orig_leaf->slotvalue[orig_leaf->slotused] = temp_value;
							if(temp_key <= *min_key){
								*min_key = temp_key;
							}
							*orig_sum += temp_key;
							orig_leaf->slotused++;
						}
					}
					return new_leaf;
				}

				inline key_type get_split_key(node_t *index_node)
				{
					key_type target;
#ifdef DEBUG
					cout << " sum of index_node: "<<index_node << " is " << index_node->sum <<endl;
#endif
					target = index_node->sum / leafslotmax;
					return target;
				}

				int sl_free (skiplist_t *sl) {
					size_t count = 0;
					node_t *item = GET_NODE(sl->head->next[0]);
					while(item) {
						if(!HAS_MARK(item->next[0])) {
							count++;
						}
						item = STRIP_MARK(item->next[0]);
					}
					return count;
				}


				//find the index node of certain key. n is the random level of the new node
				node_t *find_index_node(node_t **preds, node_t **succs, int n, skiplist_t *sl, key_type key, enum unlink unlink ) {
					node_t *pred = sl->head;
					node_t * item = NULL; //item is the pointer point to target index node.
					int d_max = 0;
					int d_min = 0;
#ifdef DEBUG
					cout << " s2 find_index_node: searching for min key that >= key " << key << " in skiplist head is " << pred << " sl->high_water is " << sl->high_water<< " n is "<< n <<endl;
#endif
					for(int level = sl->high_water - 1; level >= 0; --level) {
						markable_t next = pred->next[level]; // from top to bottom
						if(next == 0 && level > n) {// in the case of next = 0 && level > n, next[level] directly link to the nil;
							continue;
						}
#ifdef DEBUG
						cout << " s3 find_index_node: traverling level  " << level << " starting at " << pred << endl;
#endif

						item = GET_NODE(next);
#ifdef DEBUG
						cout << "item is " << item << " next is " << (node_t *)next << " level is " << level <<endl;
#endif
						while (item != NULL) {
							next = item->next[level];	
							if (EXPECT_FALSE(item == NULL)) {
#ifdef DEBUG
								cout << " s3 find_index_node: past the last item in the skiplist " << endl;
#endif
								break;
							}

#ifdef DEBUG
							cout << " s4 find_index_node: visiting item " << item << " max " << item->max << " min " << item->min << " sum is "<< item->sum << " next is " << reinterpret_cast<node_t *>(next) << endl;
							cout << " s4 find_index_node: key " << item->max << endl;
#endif

							if(EXPECT_TRUE(sl->type) == 0) {
								d_max = item->max - key;
#ifdef DEBUG
								cout << "item->max: " << item->max << " item->min " << item->min <<endl;
#endif
							}else {
								d_max = sl->type->cmp(reinterpret_cast<void *>(item->max), reinterpret_cast<void *>(key));
							}

							if(d_max >= 0) {
								d_min = key - item->min;
								if(d_min >= 0) {  // key is belong to this index node
#ifdef DEBUG
									cout << "s3 find_index_node: " << item << " with min " << item->min << " max " << item->max << " sum " << item->sum << endl;
#endif
									if(!item->leaf_ptr->isfull())
										return item;	
								}else
									break;
							}
							if (d_max == 0 && unlink != FORCE_UNLINK) {
								break;
							}

							pred = item;
							item = GET_NODE(next);
						} // end of while;
#ifdef DEBUG
						cout << " s3 find_index_node: foud pred " << pred << " next " << reinterpret_cast <node_t *>(next) << " at level " << level << endl;
#endif
						if (level <= n) {
							if(preds != NULL) {
								preds[level] = pred;
							}
							if(succs != NULL) {
								succs[level] = item;	
							}
						}

					} //end of for 

#ifdef DEBUG
					cout << " find_index_node:  foud proper place for key " << key << " in skiplist. pred is " << " returning null " << endl;
#endif
					return NULL;
				}


				value_type sl_lookup(skiplist_t *sl, key_type key) {
#ifdef DEBUG
					cout << "s1 sl_lookup: searching for key " << key << " in skiplist " << sl << endl;
#endif
					node_t *nexts[MAX_LEVELS];
					node_t *index_node = (node_t *)find_index_node(NULL, nexts, 0, sl, key, DONT_UNLINK);
					leaf_t *leaf = NULL;
					if(index_node) 
						leaf = index_node->leaf_ptr;
					else if(nexts[0]) 
						leaf = nexts[0]->leaf_ptr;		
					else
						goto not_found;

					if(leaf) {
						return leaf->get(key);
					}else
						goto not_found;
not_found:
					cout << " s1 sl_lookup: not found key " << key << " in skiplist "<< endl;
					return -1;
				}

				key_type sl_min_key (skiplist_t *sl) {
					node_t *item = GET_NODE(sl->head->next[0]);
					while(item != NULL) {
						markable_t next = item->next[0];
						if(!HAS_MARK(next))
							return item->max;
						item = STRIP_MARK(next);
					}
					return NULL;
				}

				void find_preds_simple(node_t **preds, node_t **succs, int n, skiplist_t *sl, key_type key) {
					node_t *pred = sl->head;
					int diff = 0;
					for(int level = sl->high_water - 1; level >= 0; --level) {
						markable_t next = pred->next[level];
						if(next == 0 && level >= n)
							continue;

						node_t *item = GET_NODE(next);
						while(item != NULL) {
							next = item->next[level];
							if(EXPECT_FALSE(item == NULL)) {
								break;
							}
							if (EXPECT_TRUE(sl->type) == 0 ){
								diff = item->max - key;
							}else {
								diff = sl->type->cmp(reinterpret_cast<void *>(item->max), reinterpret_cast<void *>(key));
							}
							if(diff > 0)
								break;
							pred = item;
							item = GET_NODE(next);
						} // end of while
						if(level <= n) {
							if(preds != NULL)
								preds[level] = pred;
							if(succs != NULL) 
								succs[level] = item;
						}
					}//end of for
				}

				value_type sl_insert_new(skiplist_t *sl, key_type key, value_type new_val, leaf_t *leaf = NULL)
				{
#ifdef DEBUG
					cout << " s1 sl_insert_new: going to insert key " << key << " into sl " << sl << endl;
#endif
					node_t *preds[MAX_LEVELS];
					node_t *nexts[MAX_LEVELS];

					int n = random_levels(sl);

					node_t * index_node = find_index_node(preds, nexts, n, sl, key, ASSIST_UNLINK);
					if (index_node || nexts[0]) { // index_nodes exists: key belongs to (min, max); nexts[0] exists: index nodes exists, however, key < min; anyway, nexts[0] == index_node; insert the k,v pair to its leaf node
						//	assert(index_node == nexts[0]);
						if(!index_node)
							index_node = nexts[0];
#ifdef DEBUG
						cout << " s3 sl_insert_new: going to insert key " << key << " index node exists " << index_node << " max: " << index_node->max << " min " << index_node->min << " sum " << index_node->sum<< endl; 
#endif
						leaf = index_node->leaf_ptr;
						if(!leaf->isfull()) {
							leaf->set(key, new_val);
							if(key <= index_node->min) {
								index_node->min = key;
							}
							index_node->sum += key;
						}else { //need to split
#ifdef DEBUG 
							cout << " leaf " << leaf << " is full, need to be split " << endl;
#endif
							key_type min_key = ULLONG_MAX; // new min key of original leaf node  
							key_type max_key = 0;// new max key of new leaf node
							key_type orig_sum = 0; //sum of each key in original's leaf after split.
							key_type new_sum = 0; //sum of each key in new;s leaf after split.

							key_type target_key = get_split_key(index_node); //todo: new algorithm to find the seperation key
							leaf_t *orig_leaf = new leaf_t();
							leaf_t *new_leaf = split_leaf_node(&max_key, &min_key, target_key, leaf, orig_leaf, &new_sum, &orig_sum);
							index_node->leaf_ptr = orig_leaf;
							index_node->sum = orig_sum;
							delete leaf;
#ifdef DEBUG
							cout << " orig_leaf: " << orig_leaf << " with slot " << orig_leaf->slotused << " new_leaf " << new_leaf << " with slot " << new_leaf->slotused << endl;
#endif
							//put the key value value into leaf node; it depends on the key
							if(key <= min_key) {
								new_leaf->set(key, new_val);
								if(key >= max_key)
									max_key = key;
								new_sum += key;
							}else {
								orig_leaf->set(key, new_val);
								orig_sum += key;
							} //put fini.

							// link a new index node that contains new_leaf to the skiplist

#ifdef DEBUG
							cout << " create new_leaf " << new_leaf << " with max " << max_key << " min " << index_node->min << " orig_leaf " << orig_leaf << " with max " <<index_node->max << " min " << min_key << endl;
#endif

							//allocate new index node and link it into the skiplist
							if(nexts[0]){ //allocate node, link directly
#ifdef DEBUG
								cout <<" sl_insert_new: attempting to insert an new index node between " << preds[0] << " and " << nexts[0] << " with new_sum " << new_sum <<endl;
#endif
								node_t *new_index = node_alloc(sl, n, max_key, min(index_node->min,key), false, new_sum);
								new_index->leaf_ptr = new_leaf;

								index_node->min = min_key; //update new min value of original leaf node
								index_node->sum = orig_sum; //update new sum value of original index_node
#ifdef DEBUG
								cout << new_index << " sum is " <<new_index->sum << " "<<index_node << " sum is "<< index_node->sum<<endl; 
#endif
								new_index->next[0] = reinterpret_cast<markable_t>(nexts[0]);
								for(int level = 1; level < new_index->num_levels; level++) {
									assert(nexts[level]);
									new_index->next[level] = reinterpret_cast<markable_t>(nexts[level]);
								}

								node_t *pred = preds[0];
#ifdef DEBUG
								cout << " pred of new_index is " << pred << " sl->head is " << sl->head << endl;
#endif
								pred->next[0] = reinterpret_cast<markable_t>(new_index);
								for(int level = 1; level < new_index->num_levels; ++level) {
									assert(preds[level] != 0);
									node_t *pred = preds[level];	
									pred->next[level] = reinterpret_cast<markable_t>(new_index);
#ifdef DEBUG
									cout << "preds level " << level << " is " << preds[level] << " next is " << (node_t *)pred->next[level] << endl;
#endif
								}
							}else{ //key belongs to original's [min, max], 
							//	find_preds_simple(preds, nexts, n, sl, max_key);
								node_t *new_index = node_alloc(sl, n, max_key, index_node->min, false, new_sum);
								new_index->leaf_ptr = new_leaf;
								index_node->min = min_key;
								index_node->sum = orig_sum;;
								for(int level = 0; level < new_index->num_levels; level++) {
									assert(nexts[level]);
									new_index->next[level] = reinterpret_cast<markable_t>(nexts[level]);
								}

								for(int level = 0; level < new_index->num_levels; ++level) {
									assert(preds[level]);
									node_t *pred = preds[level];	
									pred->next[level] = reinterpret_cast<markable_t>(new_index);
								}
							}

						}

					}else { // naither index_node nor nexts[0] exists. add a total new index node with new leaf_node
						assert(preds[0]);
#ifdef DEBUG
						cout << " s3 sl_insert_new with preds[0]: attempting to insert an new index node between " << preds[0] << " and " << nexts[0] << " 0?" <<endl;
#endif
						node_t *new_index = node_alloc(sl, n, key);
						leaf_t *leaf = new leaf_t();
						if(!leaf->isfull()) {
							leaf->set(key, new_val);
							new_index->sum += key;
						}
						new_index->leaf_ptr = leaf;
						new_index->next[0] = reinterpret_cast<markable_t>(nexts[0]);
						for(int level = 1; level < new_index->num_levels; ++level) {
							new_index->next[level] = reinterpret_cast<markable_t>(nexts[level]);
						}
						node_t *pred = preds[0];
						pred->next[0] = reinterpret_cast<markable_t>(new_index);
						for(int level = 1; level < new_index->num_levels; ++level) {
							assert(preds[level]);
							node_t *pred = preds[level];	
							pred->next[level] = reinterpret_cast<markable_t>(new_index);
						}
					}
					return 0;
				}


				// Mark <item > at each level of <sl> from the top down. If multiple threads try to concurrently remove 
				// the same item only one of them should succeed. Marking the bottom level established which of them succe
				// ed.
				value_type sl_remove (skiplist_t *sl, key_type key) {
#ifdef DEBUG
					cout << "s1 sl_remove: removing item with key " << key << " from skiplist " << sl << endl;
#endif
					node_t *preds[MAX_LEVELS];
					node_t *item = find_preds_index(preds, NULL, sl->high_water, sl, key, ASSIST_UNLINK);
					if(item == NULL) {
#ifdef DEBUG
						cout << "s3 sl_remove: remove failed, an item with a matching key does not exist in the skiplist" << endl;
#endif
						return NULL;
					}

					// Mark <item> at each level of <sl> from the top down. if multiple threads try to concurrently remove
					// the same item only one of them should succeed. Marking the bottom level establishes which of 
					// them succeeds.
					markable_t old_next = 0;
					for (int level = item->num_levels - 1; level >= 0; --level) {
						markable_t next;
						old_next = item->next[level];
						do {
#ifdef DEBUG
							cout << "s3 sl_remove: marking item at level " << level << "next " << old_next << endl;
#endif
							next = old_next;
							old_next = SYNC_CAS(&item->next[level], next, MARK_NODE(static_cast<node_t *>(next)));
							if(HAS_MARK(old_next)) {
#ifdef DEBUG
								cout << " s2 sl_remove: " << item << " is already marked for remove by another thread next " << old_next << endl;
#endif
								if(level == 0)
									return NULL;
								break;
							}
						}while(next != old_next);
					}

					//Atomically swap out the item's value in case another thread is updating the item while we are 
					//removing it. This establishes which operation occurs first logically, the update or the remove.
					value_type val = SYNC_SWAP(&item->min, NULL);
#ifdef DEBUG
					cout << " sw sl_remove: replaced item " << item << " 's value whit NULL " << endl;
#endif
					//unlink the item
					find_preds_index(NULL, NULL, 0, sl, key, FORCE_UNLINK);

					//free the node
					if(sl->type !=NULL) {
						free(static_cast<void *>(item->max));	
					}
					free(item);

					return val;
				}

				sl_iter * sl_iter_begin(skiplist_t *sl, key_type key) {
					sl_iter *iter = static_cast<sl_iter *>(malloc(sizeof(sl_iter)));
					if(key != NULL) {
						find_preds_index(NULL, &iter->next, 1, sl, key ,DONT_UNLINK);
					}else {
						iter->next = GET_NODE(sl->head->next[0]);
					}
					return iter;
				}

				value_type sl_iter_next(sl_iter *iter, key_type * key_ptr) {
					node_t *item = iter->next;
					while ( item != NULL && HAS_MARK(item->next[0])) {
						item = STRIP_MARK(item->next[0]);
					}
					if(item == NULL) {
						iter->next = NULL;
						return NULL;
					}

					iter->next = STRIP_MARK(item->next[0]);
					if ( key_ptr != NULL) {
						*key_ptr = item->max;
					}
					return item->min;
				}

				void sl_iter_free ( sl_iter *iter) {
					free(iter);
				}

		};
}
#endif

