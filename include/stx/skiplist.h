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
#include <list>
#include <climits>
#include <set>
#include "nv_backend.h"
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

#define MAX_LEVELS 16 
#define HOT_THRESHOULD 15 
#define TAG_VALUE(v, tag) ((v) | tag)
#define IS_TAGGED(v, tag) ((v) & tag)
#define STRIP_TAG(v, tag) ((v) & ~tag)

#define PEM_LEN 4096000000

typedef size_t markable_t;

//Marking the <next> field of a node logically removes it from the list
#define MARK_NODE(x) TAG_VALUE(reinterpret_cast<markable_t>(x), 0x1)
#define HAS_MARK(x) (IS_TAGGED((x), 0x1) == 0x1)
#define GET_NODE(x) (reinterpret_cast<dnode_t *>(x))
#define STRIP_MARK(x) (reinterpret_cast<dnode_t *>(STRIP_TAG((x), 0x1)))

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
				typedef struct kv_entry
				{
					key_type key;
					value_type value;
				}entry_t;

				typedef struct nvram_node {
					std::bitset<leafslotmax> bs;
					struct nvram_node *next;
					entry_t slotentry[leafslotmax];
					uint64_t x;

					nvram_node()
					{
						bs.reset(); //to set each bit to 0;
					}

					///True if the node's slots are full
					inline bool isfull() const
					{
						return bs.count() == bs.size();
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
#if 0
						for(int index = 0; index < bs.size(); index++) {
							if(!bs.test(index))
								return index;
						}
						return -1;
#endif
						return bs.count();
					}

					int hash(key_type key, int k) {
						return (key + k * (1 + (((key >> 5) + 1) % (leafslotmax - 1)))) % leafslotmax;	
					}
#if 0
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
#endif	
					inline int set(entry_t *entry) {
						int index = get_first_free_bit(); 			
#ifdef DEBUG
						cout << this << " set key " << entry->key << " at " <<index << endl; 
#endif
						nv_memcpy(&slotentry[index],entry,sizeof(entry_t));
					//	nv_flush(&slotentry[index]);
				//		set_bitmap(index);
				//		nv_flush(&bs);
						return index;
					}

					inline value_type get(key_type key) {
						for(int i = leafslotmax - 1; i >= 0; i--) {
							if(slotentry[i].key == key){
								return slotentry[index].value;
							}
						}
						return -1;
					}

				}nvnode_t;

				typedef struct dram_node {
					key_type max;  //max key in its data node
					key_type min; //minimum key in its data node
					key_type sum; //sum of all the key in the leaf node
					int split_count;
					unsigned num_levels;
					nvnode_t *nv_node;
					markable_t next[0];
				} dnode_t;

				struct sl_iter {
					dnode_t *next;
				};

				typedef struct sl {
					dnode_t *head;
					const datatype_t *type;
					int high_water;	//max historic number of levels
					list<nvnode_t *> *shadow_list;
				} skiplist_t ;

			public:

				nvnode_t *get_shadow_node(skiplist_t *sl) //since we use append style to insert new pair into leaf, logging for leaf is not necessary. shadow node is only for nvnode_t
				{
					nvnode_t *nv_node = sl->shadow_list->front();
					sl->shadow_list->pop_front();
					return nv_node;
				}

				void put_shadow_node(skiplist_t *sl, nvnode_t *nv_node) {
					sl->shadow_list->push_back(nv_node);	
				}

				int random_levels (skiplist_t *sl) {
#if 1
					double r = rand()/(RAND_MAX + 1.0);
					int levels = 1;
					while(r < 0.25 && levels < MAX_LEVELS) {
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
#else
					int level = 1;
					while(rand() % 2)
						level++;
					level = (MAX_LEVELS > level)? level : MAX_LEVELS;
					if(level >= sl->high_water)
						sl->high_water = level;
					return level;
#endif
				}


				nvnode_t *nvnode_alloc(){
					nvnode_t *item = reinterpret_cast<nvnode_t *>(nv_malloc(sizeof(nvnode_t)));
					item->bs.reset();
					item->next = NULL;
					return item;
				}

				int nvnode_flush(nvnode_t *nv_node)
				{
					size_t sz = sizeof(*nv_node);
					for(markable_t i = 0; i <= sz; i += 64) {
						nv_flush((void *)nv_node + i);
					}
					return 0;
				}

				dnode_t *dnode_alloc(int num_levels, key_type max, key_type min = ULLONG_MAX, key_type sum = 0){

					assert(num_levels >= 0 && num_levels <= MAX_LEVELS);
			//		size_t sz = sizeof(dnode_t) + (num_levels - 1) * sizeof(dnode_t *);
					size_t sz = sizeof(dnode_t) + num_levels * sizeof(dnode_t *);
					dnode_t *item = reinterpret_cast<dnode_t *>(malloc(sz)); //todo use new memory allocator later
					memset(item, 0x0, sz);
					item->max = max;
					if(min == ULLONG_MAX)
						item->min = max;
					else
						item->min = min;
					item->num_levels = num_levels;
					item->sum = sum;
					item->split_count = 0;
#ifdef DEBUG 
					cout << "s2 node_alloc : new node " << item << " "<< num_levels << " levels" << endl;
#endif
					return item;
				}

				skiplist_t *sl_alloc (const datatype_t *type) {
					skiplist_t *sl = static_cast<skiplist_t *>(malloc(sizeof(skiplist_t)));
					sl->type = type;
					sl->high_water = 1;
					sl->shadow_list = new list <nvnode_t *>();
					sl->head = dnode_alloc(MAX_LEVELS, 0, 0);
					memset(reinterpret_cast<void *>(sl->head->next), 0x0, MAX_LEVELS * sizeof(skiplist_t *));
					//
					nvnode_t *shadow_node = nvnode_alloc();
					put_shadow_node(sl, shadow_node);
					//

					sl->head->nv_node = nvnode_alloc();
					//todo: logging
					//		nv_flush(sl->head->nv_node, sizeof(nvnode_t));	
					nvnode_flush(sl->head->nv_node);	
					return sl;
				}

				inline nvnode_t *split_leaf_node(key_type *max_key, key_type *min_key, const key_type target_key, nvnode_t *old_nvnode, nvnode_t *orig_nvnode, key_type *new_sum, key_type *orig_sum) {

					nvnode_t *new_nvnode = nvnode_alloc();
					for(int index = old_nvnode->bs.count() - 1; index >= 0; index--){
						if(old_nvnode->bs.test(index) != 1)
							continue;

						key_type temp_key = old_nvnode->slotentry[index].key;
						key_type temp_value = old_nvnode->slotentry[index].value;
#ifdef DEBUG
						cout << "target key "<< target_key << " temp_key " << temp_key << endl;
#endif	
						entry_t temp_entry;
						if(temp_key <= target_key ) {
							temp_entry.key = temp_key;
							temp_entry.value = temp_value;
							int ix = new_nvnode->set(&temp_entry);
							new_nvnode->set_bitmap(ix);
							if(temp_key >= *max_key){
								*max_key = temp_key;
							}
							*new_sum += temp_key;
						}else{
							temp_entry.key = temp_key;
							temp_entry.value = temp_value;
							int ix = orig_nvnode->set(&temp_entry);
							orig_nvnode->set_bitmap(ix);
							if(temp_key <= *min_key){
								*min_key = temp_key;
							}
							*orig_sum += temp_key;
						}
					}
					return new_nvnode;
				}

				inline int journaling_ahead(skiplist_t *sl, nvnode_t *nv_node)
				{
					nvnode_t *shadow_node = get_shadow_node(sl); 
					nv_memcpy(shadow_node,nv_node,sizeof(nvnode_t));
					if(nvnode_flush(shadow_node) == 0){
						put_shadow_node(sl,shadow_node);
						return 0;
					}
					else
						return -1;
				}

				inline key_type get_split_key(dnode_t *index_node)
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
					dnode_t *item = GET_NODE(sl->head->next[0]);
					while(item) {
						if(!HAS_MARK(item->next[0])) {
							count++;
						}
						item = STRIP_MARK(item->next[0]);
					}
					return count;
				}


				//find the index node of certain key. n is the random level of the new node
				dnode_t *find_index_node(dnode_t **preds, dnode_t **succs, int n, skiplist_t *sl, key_type key, enum unlink unlink ) {
					dnode_t *pred = sl->head;
					dnode_t * item = NULL; //item is the pointer point to target index node.
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
						cout << "item is " << item << " next is " << (dnode_t *)next << " level is " << level <<endl;
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
							cout << " s4 find_index_node: visiting item " << item << " max " << item->max << " min " << item->min << " sum is "<< item->sum << " next is " << reinterpret_cast<dnode_t *>(next) << endl;
							cout << " s4 find_index_node: key " << item->max << endl;
#endif

							d_max = item->max - key;

							if(d_max >= 0) {
								d_min = key - item->min;
								if(d_min >= 0) {  // key is belong to this index node
#ifdef DEBUG
									cout << "s3 find_index_node: " << item << " with min " << item->min << " max " << item->max << " sum " << item->sum << endl;
#endif
									if(!item->nv_node->isfull())
										return item;	
									else
										break;
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
						cout << " s3 find_index_node: foud pred " << pred << " next " << reinterpret_cast <dnode_t *>(next) << " at level " << level << endl;
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
					dnode_t *nexts[MAX_LEVELS];
					dnode_t *index_node = (dnode_t *)find_index_node(NULL, nexts, 0, sl, key, DONT_UNLINK);
					nvnode_t *nv_node =NULL;
					if(index_node) 
						nv_node = index_node->nv_node;
					else if(nexts[0]) 
						nv_node = nexts[0]->nv_node;		
					else
						goto not_found;

					if(nv_node) {
						return nv_node->get(key);
					}else
						goto not_found;
not_found:
					cout << " s1 sl_lookup: not found key " << key << " in skiplist "<< endl;
					return -1;
				}

				key_type sl_min_key (skiplist_t *sl) {
					dnode_t *item = GET_NODE(sl->head->next[0]);
					while(item != NULL) {
						markable_t next = item->next[0];
						if(!HAS_MARK(next))
							return item->max;
						item = STRIP_MARK(next);
					}
					return NULL;
				}

				void find_preds_simple(dnode_t **preds, dnode_t **succs, int n, skiplist_t *sl, key_type key) {
					dnode_t *pred = sl->head;
					int diff = 0;
					for(int level = sl->high_water - 1; level >= 0; --level) {
						markable_t next = pred->next[level];
						if(next == 0 && level >= n)
							continue;

						dnode_t *item = GET_NODE(next);
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

				void print_height(skiplist_t *sl)
				{
					dnode_t *pred = sl->head;
					dnode_t *item = NULL;
					markable_t next = pred->next[0];
					item = GET_NODE(next);
					while(item != NULL)
					{
						cout << item->num_levels << endl;
						next = item->next[0];
						item = GET_NODE(next);
					}
				}

				value_type sl_insert_new(skiplist_t *sl, key_type key, value_type new_val, nvnode_t *nv_node = NULL)
				{
#ifdef DEBUG
					cout << " s1 sl_insert_new: going to insert key " << key << " into sl " << sl << endl;
#endif
					dnode_t *preds[MAX_LEVELS];
					dnode_t *nexts[MAX_LEVELS];
					memset(preds, 0x0, sizeof(dnode_t *)*MAX_LEVELS);
					memset(nexts, 0x0, sizeof(dnode_t *)*MAX_LEVELS);

					int n = random_levels(sl);
					//find the indexing dram node
					dnode_t * index_node = find_index_node(preds, nexts, n, sl, key, ASSIST_UNLINK);
					if (index_node || nexts[0]) { // index_nodes exists: key belongs to (min, max); nexts[0] exists: index nodes exists, however, key < min; anyway, nexts[0] == index_node; insert the k,v pair to its leaf node
						if(!index_node)
							index_node = nexts[0];
#ifdef DEBUG
						cout << " s3 sl_insert_new: going to insert key " << key << " index node exists " << index_node << " max: " << index_node->max << " min " << index_node->min << " sum " << index_node->sum << " slot " << index_node->nv_node->bs.count()<< endl; 
#endif
						nv_node = index_node->nv_node; // read from the nvram 
						if(!nv_node->isfull()) {
							entry_t new_entry;
							new_entry.key = key;
							new_entry.value = new_val;
							int ix = nv_node->set(&new_entry); 
							nv_flush(&nv_node->slotentry[ix]);
							nv_node->set_bitmap(ix);
							nv_flush(&nv_node->bs);

							//update index_node
							if(key <= index_node->min) {
								index_node->min = key;
							}
							index_node->sum += key;
						}else { //need to split
#ifdef DEBUG 
							cout << " nvnode " << nv_node << " is full, need to be split " << endl;
#endif
							key_type min_key = ULLONG_MAX; // new min key of original leaf node  
							key_type max_key = 0;// new max key of new leaf node
							key_type orig_sum = 0; //sum of each key in original's leaf after split.
							key_type new_sum = 0; //sum of each key in new;s leaf after split.
							

							key_type target_key = get_split_key(index_node); //todo: new algorithm to find the seperation key
							nvnode_t *orig_nvnode = nvnode_alloc(); 
							nvnode_t *new_nvnode = split_leaf_node(&max_key, &min_key, target_key, nv_node, orig_nvnode, &new_sum, &orig_sum); // get two new leafs, each contains half of entries of nv_node;
							//link two new nv_nodes
							orig_nvnode->next = nv_node->next;
							new_nvnode->next = orig_nvnode;
							//link completed
							//
							//insert the new key,value into either orignal node or new node
							if(key <= min_key) {
								if(key >= max_key)
									max_key = key;
								entry_t new_entry;
								new_entry.key = key;
								new_entry.value = new_val;
								int ix = new_nvnode->set(&new_entry); 
								new_nvnode->set_bitmap(ix);
								new_sum += key;
							}else {
								entry_t new_entry;
								new_entry.key = key;
								new_entry.value = new_val;
								int ix = orig_nvnode->set(&new_entry);
								orig_nvnode->set_bitmap(ix);
								orig_sum += key;
							} 
							nvnode_flush(orig_nvnode);
							nvnode_flush(new_nvnode);
							SYNC_CAS(&preds[0]->nv_node->next, reinterpret_cast<markable_t>(orig_nvnode), reinterpret_cast<markable_t>(new_nvnode));
							//insert completed
							//update dram indexing node
							index_node->min = min_key;
							index_node->sum = orig_sum;
							index_node->split_count++;
					#if 0
							if(index_node->split_count >= HOT_THRESHOULj) {
								n = MAX_LEVELS;
								index_node->split_count = 0;
							}
					#endif
							dnode_t * new_index = dnode_alloc(n, max_key, min(index_node->min, key), new_sum);
							for(int level = 0; level < new_index->num_levels; level++){
								new_index->next[level] = reinterpret_cast<markable_t>(nexts[level]);	
								dnode_t *pred = preds[level];
								if(pred == NULL)
									pred = sl->head;
								pred->next[level] = reinterpret_cast<markable_t>(new_index);
							}
							index_node->nv_node = orig_nvnode;
							new_index->nv_node = new_nvnode;

							//update completed
						}

					}else { // naither index_node nor nexts[0] exists. add a total new index node with new leaf_node
						assert(preds[0]);
#ifdef DEBUG
						cout << " s3 sl_insert_new with preds[0]: attempting to insert an new index node between " << preds[0] << " and " << nexts[0] << " 0?" <<endl;
#endif
						dnode_t *new_index = dnode_alloc(n, key);
						nvnode_t *nv_node = nvnode_alloc();


						if(!nv_node->isfull()) {
							entry_t new_entry;
							new_entry.key = key;
							new_entry.value = new_val;
							int ix = nv_node->set(&new_entry); 
							nv_flush(&nv_node->slotentry[ix]);
							nv_node->set_bitmap(ix);
							nv_flush(&nv_node->bs);
							new_index->sum += key;
						}

						dnode_t *pred = preds[0];
						SYNC_CAS(&pred->nv_node->next, 0, reinterpret_cast<markable_t>(nv_node));

						for(int level = 0; level < new_index->num_levels; ++level) {
							new_index->next[level] = reinterpret_cast<markable_t>(nexts[level]);
							dnode_t *pred = preds[level];
							pred->next[level] = reinterpret_cast<markable_t>(new_index);
						}
						new_index->nv_node = nv_node;
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
					dnode_t *preds[MAX_LEVELS];
					dnode_t *item = find_preds_index(preds, NULL, sl->high_water, sl, key, ASSIST_UNLINK);
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
							old_next = SYNC_CAS(&item->next[level], next, MARK_NODE(static_cast<dnode_t *>(next)));
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
					dnode_t *item = iter->next;
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

