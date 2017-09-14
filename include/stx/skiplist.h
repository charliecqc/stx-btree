#ifndef STX_SKIPLIST_HEADER
#define STX_SKIPLIST_HEADER
#include <algorithm>
#include <functional>
#include <istream>
#include <ostream>
#include <memory>
#include <stdlib.h>
#include <immintrin.h>
#include <bitset>
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

#define MAX_LEVELS 24
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


					///set variable to initial values
					inline leaf()
					{
						bs.reset(); // to set each bit to 0
					}

					///True if the node's slots are full
					inline bool isfull() const
					{
						//	return (slotused == leafslotmax - 1);
						return bs.count() == bs.size();
					}
					///set the indexth bit to 1 
					inline int set_bitmap(unsigned short index)
					{
						bs[index] = 1;
						return 0;
					}

					///set the index th bit to 0 
					inline int clear_bitmap(unsigned short index) 
					{
						bs[index] = 0;
						return 0;
					}
					///return index th bit's value
					inline int get_bit(unsigned short index)
					{
						return bs[index];
					}
					///return the first bit that was set to 1, from left to right
					inline int get_first_non_zero_bit()
					{
						for(int index = 0; index < leafslotmax; index++) {
							if(bs.test(index))
								return index;
						}
					}

					///return the first bit that was 0. from left to right
					inline int get_first_free_bit()
					{
						for(int index = 0; index < leafslotmax; index++) {
							if(!bs.test(index))
								return index;
						}
					}

				}leaf_t;

				typedef struct node {
					key_type key;  //max key in its data node
					key_type min; //minimum key in its data node
					value_type val;
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
					unsigned int r = rand();
					//		int z = __buildin_ctz(r);
					int levels = ffs(r);
					//	int levels = (int)(z / 1.5);
					//		int levels = r % MAX_LEVELS;
					//	levels = levels / 1.5;
					if(levels == 0)
						return 1;
					if(levels > sl->high_water) {
						levels = SYNC_ADD(&sl->high_water, 1); // in case of unusual large level
#ifdef DEBUG
						cout << " s2 random_levels: increased high water mark to " << sl->high_water << endl;
#endif
					}
					if(levels > MAX_LEVELS)
					{levels = MAX_LEVELS;}
					return levels;
				}

				// Allocate a new leaf node
				leaf_t *leaf_alloc() {
					size_t sz = sizeof(leaf_t);		
					leaf_t *item = reinterpret_cast<leaf_t *>(malloc(sz));
					memset(item, 0, sz);
					return item;
				}

				node_t *node_alloc(int num_levels, key_type key, value_type val) {
					assert(num_levels >= 0 && num_levels <= MAX_LEVELS);
					size_t sz = sizeof(node_t) + (num_levels - 1) * sizeof(node_t *);
					node_t *item = static_cast<node_t *>(malloc(sz)); //todo use new memory allocator later
					memset(item, 0, sz);
					item->key = key;
					item->val = val;
					item->num_levels = num_levels;
					item->leaf_ptr = NULL;
#ifdef DEBUG 
					cout << "s2 node_alloc : new node " << item << " "<< num_levels << " levels" << endl;
#endif
					return item;
				}

				skiplist_t *sl_alloc (const datatype_t *type) {
					skiplist_t *sl = static_cast<skiplist_t *>(malloc(sizeof(skiplist_t)));
					sl->type = type;
					sl->high_water = 1;
					sl->head = node_alloc(MAX_LEVELS, 0, 0);
					memset(reinterpret_cast<void *>(sl->head->next), 0, MAX_LEVELS * sizeof(skiplist_t *));
					return sl;
				}

				///return a pointer to the splited new data node
				//key will be used to make a new index node
				//@min_key: minimum key stored in the data node
				//@max_key: maximum key stored in the data node
				inline leaf_t * split_date_node(key_type *max_key, key_type *min_key,const key_type target_key, leaf_t *old_leaf)	{
					struct leaf *new_leaf = leaf_alloc();
					memcpy(new_leaf, old_leaf, sizeof(leaf_t));
					for(int index = 0; index < new_leaf->bs.size(); index++) {

						///invalidate the keys which are larger than target_key in the new data node
						if(new_leaf->slotkey[index] > target_key)
							new_leaf->clear_bitmap(index);	
						///invalidate the keys which are smaller than target_key in the old data node
						if(old_leaf->slotkey[index] <= target_key)
							old_leaf->clear_bitmap(index);

						if(!new_leaf->bs.test(index))	
							continue;
						///calculate the largest key in the data node
						if(new_leaf->slotkey[index] > *max_key)
							*max_key = new_leaf->slotkey[index];
						///calculate the smallest key in the dat node
						if(new_leaf->slotkey[index] < *min_key)
							*min_key = new_leaf->slotkey[index];
					}
					return new_leaf;
				}

				void sl_free (skiplist_t *sl) {
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
				//find the preds and succss of new node. n is the random level of the new node
				node_t *find_preds(node_t **preds, node_t **succs, int n, skiplist_t *sl, key_type key, enum unlink unlink) {
					node_t *pred = sl->head;
					node_t *item = NULL;
					int d = 0;
#ifdef DEBUG
					cout << "s2 find_preds: searching for key " << key << " in skiplist (head is " << pred << " )"<<endl;
#endif
					for(int level = sl->high_water - 1; level >= 0; --level) {
						//		cout << "start at level: " << level << endl;
						markable_t next = pred->next[level];
						if(next == 0 && level >= n) {
							continue;	
						}
#ifdef DEBUG
						cout << "s3 find_preds: traversing level " << level << " starting at " << pred <<endl;
#endif
						if(EXPECT_FALSE(HAS_MARK(next))) {
#ifdef DEBUG
							cout << "s2 find_preds: pred " << pred << " is marked for removal (next  " << next << " ), retry "<<endl;
#endif
							cout << " hit a return " << endl;
							return find_preds(preds, succs, n, sl, key, unlink);
						}

						item = GET_NODE(next);
						while (item != NULL) {
							next = item->next[level];

							//A tag means an item is logically removed but not phusically unlinked yet
							while(EXPECT_FALSE(HAS_MARK(next))) {
#ifdef DEBUG
								cout << "s3 find_preds: found marked item " <<item << " next is  " << next <<endl;
#endif
								if (unlink == DONT_UNLINK) {

									//skip over logically removed items
									item = STRIP_MARK(next);
									if(EXPECT_FALSE(item == NULL)) {
										cout << " hit a break " << endl;
										break;
									}
									next = item->next[level];
								}else {
									// Unlink logically removed items
									markable_t other = SYNC_CAS(&pred->next[level], reinterpret_cast<markable_t>(item), reinterpret_cast<markable_t>STRIP_MARK(next));
									if(other == reinterpret_cast<markable_t>(item)) {
#ifdef DEBUG
										cout << "s3 find_preds: unlinked item from pred " << pred << endl;
#endif
										item = STRIP_MARK(next);

									}else {
#ifdef DEBUG
										cout << "s3 find_preds: lost race to unlink item pred " << pred << " 's link changed to " << other << endl;
#endif
										if(HAS_MARK(other)) {
											cout << " hit a return " << endl;
											return find_preds(preds, succs, n, sl, key, unlink); //retry
										}
										item = GET_NODE(other);
									}
									next = (item != NULL) ? item->next[level] : 0;
								}
							}

							if (EXPECT_FALSE(item == NULL)) {
#ifdef DEBUG
								cout << "s3 find_preds: past the last item in the skiplist " << endl;
								cout << " hit a break " << endl;
#endif
								break;	
							}
#ifdef DEBUG
							cout << "s4 find_preds: visiting item " << item << " next is  " << (node_t *)next << endl;
							cout << "s4 find_preds: key " << STRIP_MARK(item->key) << " val " << item->val << endl;
#endif
							if(EXPECT_TRUE(sl->type) == 0) {
								d = item->key - key;
							}else {
								d = sl->type->cmp(reinterpret_cast<void *>(item->key), reinterpret_cast<void *>(key));
							}

							if(d > 0) {
								//	cout << " hit a break due to d > 0 " << endl; 
								break;
							}
							if (d == 0 && unlink != FORCE_UNLINK) {
								//	cout << " hit a break due to d == 0 " << endl; 
								break;
							}

							pred = item;
							item = GET_NODE(next);
						}
#ifdef DEBUG
						cout << "s3 find_preds: found pred " << pred << " next " <<(node_t *)next << " level " << level << endl;
#endif
						//charliecqc changed
						//	if(level < n) {
						if(level <= n) {
							if(preds != NULL) {
								preds[level] = pred;
							}
							if(succs != NULL) {
								succs[level] = item;
							}
						}
#ifdef DEBUG
						//						cout << "  end with level: " << level << endl;
#endif
					}

#ifdef DEBUG
					//		cout << "out of the loop " << endl;
#endif
					if (d == 0) {
#ifdef DEBUG
						cout << "s2 find_preds: found matching item " << item << " in skiplist, pred is " << pred << endl;
#endif
						return item;
					}
#ifdef DEBUG
					cout << "s2 find_preds: found proper place for key " << key << " in skiplist, pred is " << pred << " returning null " << endl;
#endif
					return NULL;

					}

					value_type sl_lookup(skiplist_t *sl, key_type key) {
#ifdef DEBUG
						cout << "s1 sl_lookup: searching for key " << key << " in skiplist " << sl << endl;
#endif
						node_t *nexts[MAX_LEVELS];
						node_t *item = (node_t *)find_preds(NULL, nexts, 0, sl, key, DONT_UNLINK);
						leaf_t *leaf = NULL;
						if(item) 
							leaf = item->leaf_ptr;
						else if(nexts[0]) 
							leaf = nexts[0]->leaf_ptr;		
						else
							goto not_found;

						if(leaf) {
							for(int i = leafslotmax-1; i>=0; i--) {
								if(leaf->slotkey[i] == key) {
#ifdef DEBUG
									cout << " s1 sl_lookup: found value " << leaf->slotvalue[i] << " for key " << key << " in skiplist "<< endl;
#endif
									return leaf->slotvalue[i];
								}
							}
						}else
							goto not_found;
not_found:
#ifdef DEBUG
						cout << " s1 sl_lookup: not found key " << key << " in skiplist "<< endl;
#endif
						return 0;
					}

					value_type sl_lookup_old(skiplist_t *sl, key_type key) {
#ifdef DEBUG
						cout << "s1 sl_lookup: searching for key " << key << " in skiplist " << sl << endl;
#endif
						node_t *nexts[MAX_LEVELS];
						node_t *item = find_preds(NULL, NULL, 0, sl, key, DONT_UNLINK);

						// if we found an <item> matching the <key> return its value
						if(item != NULL) {
							value_type val = item->val;
							if(val != NULL) {
#ifdef DEBUG
								cout << "s1 sl_lookup: found item " << item << " val " <<item->val <<endl;
#endif
								return val;
							}
						}
#ifdef DEBUG
						cout << "s1 sl_lookup: no item in the skiplist matched the key " << endl; 
#endif
						return NULL;
					}

					key_type sl_min_key (skiplist_t *sl) {
						node_t *item = GET_NODE(sl->head->next[0]);
						while(item != NULL) {
							markable_t next = item->next[0];
							if(!HAS_MARK(next))
								return item->key;
							item = STRIP_MARK(next);
						}
						return NULL;
					}

					value_type update_item(node_t *item, value_type expectation, value_type new_val) {
						value_type old_val = item->val;
						// if the item's value is NULL it means another thread removed the node out from under us
						if(EXPECT_FALSE(old_val == 0)) {
#ifdef DEBUG
							cout << "s2 update_item: lost a race to another thread removing the item, retry " << endl;
#endif
							return 0; //retry
						}

						if(EXPECT_FALSE(expectation == CAS_EXPECT_DOES_NOT_EXIST)) {
#ifdef DEBUG
							cout << "s1 update_item: the expectation was not met; the skiplist was not changed "<<endl;
#endif
							return old_val;
						}
						// Use a CAS and not a SWAP. If the CAS fails it means another thread removed the node or updated its 
						// value. If another thread removed the node but it is not unlinked yet and we used a SWAP, we could 
						// replace DOES_NOT_EXIST with our value. Then another thread that is updating the value could think 
						// it succeeded and return return out value even though it should return DOES_NOT_EXIST
						if(old_val == SYNC_CAS(&item->val, old_val, new_val)) {
#ifdef DEBUG
							cout << "s1 update_item: the CAS succeeded.updated the value of the item "<<endl;
#endif
							return old_val; //success
						}
#ifdef DEBUG
						cout << "s2 update_item: lost a race. the CAS failed. another thread chhhhanged the item's value'"<<endl;
#endif
						return update_item(item, expectation, new_val); //tail call
					}
					//new version of insert. nodes in skiplist are used as indexing node, data will be stored separtely in data nodes. 
					value_type sl_insert(skiplist_t *sl, key_type key, value_type expectation, value_type new_val,leaf_t *leaf = NULL) {

						node_t *preds[MAX_LEVELS];
						node_t *nexts[MAX_LEVELS];
						node_t *new_item = NULL;
						int n = random_levels(sl);
#ifdef DEBUG
						cout << " s1 sl_insert: key   " << key << " skiplist " << sl << endl;
						cout << " s1 sl_insert:expectation " << expectation << " new value " << new_val << " with "<< n << " levels "<< endl;
#endif
						node_t *old_item = find_preds(preds, nexts, n, sl, key, ASSIST_UNLINK);

						//If there is already an item in the skiplist that matched the key just update its value
						if(old_item != NULL) {
#ifdef DEBUG
							cout << " same keys are already existed in the skiplist " << endl;
#endif
							value_type ret_val = update_item(old_item, expectation, new_val);
							if(ret_val != 0)
								return ret_val;

							//If we lose a race with a thread removing the item we tried to update then we haveto retty
							return sl_insert(sl, key, expectation, new_val); //tail call
						}
						//it old_item == NULL, check whether nexts[0] exists. next[0] is the lower_bound of key
						if(EXPECT_FALSE(expectation != CAS_EXPECT_DOES_NOT_EXIST && expectation != CAS_EXPECT_WHATEVER)) {
#ifdef DEBUG
							cout << " s1 sl_insert: the expectation was not met, the skiplist was not changed " << endl;
#endif
							return 0;
						}

						if(nexts[0]) {
#ifdef DEBUG
							cout << " s3 sl_insert: index node exists" << nexts[0]  << " key is " << nexts[0]->key << endl;
#endif
							leaf = nexts[0]->leaf_ptr;
#ifdef DEBUG
							cout << " s3 sl_insert: get leaf node: " << leaf  << " of  " << nexts[0] << endl;
#endif
							if(!leaf->isfull()) { // if the leaf node is not full
								markable_t index = leaf->get_first_free_bit();
								leaf->slotkey[index] = key;
								leaf->slotvalue[index] = new_val;
								leaf->set_bitmap(index);
							}else {
								//split
								key_type min = 9999999999999999;
								key_type max = 0;
								leaf_t *new_leaf = split_date_node(&max, &min, (nexts[0]->key - nexts[0]->min)/2,leaf);
								if(key <= max) {
									markable_t index = new_leaf->get_first_free_bit();
									new_leaf->slotkey[index] = key;
									new_leaf->slotvalue[index] = new_val;
									new_leaf->set_bitmap(index);
								}else {
									leaf_t *old_leaf = nexts[0]->leaf_ptr;
									markable_t index = old_leaf->get_first_free_bit();
									old_leaf->slotkey[index] = key;
									old_leaf->slotvalue[index] = new_val;
									old_leaf->set_bitmap(index);
								}
								
								///@max is the max key value of new index node
								return sl_insert(sl, max, expectation, max, new_leaf);

							}
						} else {
							//Create a new index node and insert it into the skiplist
#ifdef DEBUG
							cout << " s3 sl_insert: attempting to insert a new index node between " << preds[0] << " and " << nexts[0]  << endl;
#endif
							//nexts is null and need allocate new leaf node and index node

							//allocate a new skiplist node
							key_type new_key = sl->type == NULL ? key : reinterpret_cast<key_type>(sl->type->clone(reinterpret_cast<void *>(key)));
							new_item = node_alloc(n, new_key, new_val);
#ifdef DEBUG
							cout << " allocate new index node  " << new_item << endl;
#endif
							leaf = leaf_alloc();
#ifdef DEBUG
							cout << "allocate new leaf " << leaf << endl;
#endif
							//set the leaf into new_item
							new_item->leaf_ptr = leaf;

							//Set <new_item> into <sl> from the bottom level up. After <>;
							markable_t next = new_item->next[0] = reinterpret_cast<markable_t>(nexts[0]);
#ifdef DEBUG
							cout << "next[0] of " << new_item << " is " << new_item->next[0] << endl;
#endif
							for(int level = 1; level < new_item->num_levels;  ++level) {
								new_item->next[level] = reinterpret_cast<markable_t>(nexts[level]);	
#ifdef DEBUG
								cout << "next["<<level <<"] of " << new_item << " is " << new_item->next[level] << endl;
#endif
							}
							//Allocate a new leaf node. Right now i just allocate the leaf when is is necessary. 
							//todo: leaf pool

							if(!leaf->isfull()) {
								markable_t index = leaf->get_first_free_bit();
								leaf->slotkey[index] = key;
								leaf->slotvalue[index] = new_val;
								leaf->set_bitmap(index);
#ifdef DEBUG
								cout << "insert key " << key  << " into leaf " << leaf << endl;
#endif
							}else { //not possible?

							}


							//Link <new）item> into <sl> from the bottom level up. After <new_item> is insert into the bottom level			   //it is officially part of the skiplist 
							node_t *pred = preds[0];
#ifdef DEBUG
							cout << " pred of new_item: " << new_item << " is " << pred<< " pred->next is " <<  (node_t *)pred->next[0] <<" head is "<< sl->head << " leaf of new_item is "<< new_item->leaf_ptr<< endl;
#endif
							markable_t other = SYNC_CAS(&pred->next[0], next, reinterpret_cast<markable_t>(new_item));
#ifdef DEBUG 
							cout << " after cas pred of new_item: " << new_item << " is " << pred<< " pred->next is " <<  (node_t *)pred->next[0] << " leaf of new item is " << new_item->leaf_ptr << " sl head is "<< sl->head << endl;
#endif
							if(other != next) {
#ifdef DEBUG
								cout <<"s3 sl_insert: failed to change pred's link: expected " << next << " found " << other << endl;
#endif
								//Lost a race to another thread modifying the skiplist. Free the new item we allocated and retry
								if(sl->type != NULL) {
									free(reinterpret_cast<void *>(new_key));
								}
								free(new_item);
								return sl_insert(sl, key, expectation, new_val); //tail call
							}
#ifdef DEBUG
							cout << "s3 sl_insert: successfully inserted a new item " << new_item << " at the bottom level " << endl;
#endif


							for(int level = 1; level < new_item->num_levels; ++level) {
#ifdef DEBUG
								cout << "s3 sl_insert: inserting the new item " << new_item << " at level " << level << endl;
#endif
								do {
									node_t * pred = preds[level];
#ifdef DEBUG
									cout << " pred" << level << " of new_item: " << new_item << " is " << pred<< " pred->next is " <<  (node_t *)pred->next[level] <<" head is "<< sl->head << endl;
#endif
									markable_t other = SYNC_CAS(&pred->next[level], (markable_t)nexts[level], (markable_t)new_item);
#ifdef DEBUG 
									cout << " after cas pred"<<level << "of new_item: " << new_item << " is " << pred<< " pred->next is " <<  (node_t *)pred->next[level] << " leaf of new item is " << new_item->leaf_ptr << " sl head is "<< sl->head << endl;
#endif
									if(other == (markable_t)nexts[level])
										break;
#ifdef DEBUG
									cout << "s3 sl_insert: lost a race, failed to change pred's link. expected " << nexts[level] << " found " << other << endl;
#endif
									find_preds(preds, nexts, new_item->num_levels, sl, key, ASSIST_UNLINK);

									for(int i = level; i < new_item->num_levels; ++i) {
										markable_t old_next = new_item->next[i];
										if(reinterpret_cast<markable_t>(nexts[i]) == old_next)
											continue;

										///update <new_items>'s inconsistent next pointer before trying again. Use a CAS so if 
										//another thread is trying to remove the new item concurrently we do not stop on the mark
										//it places on the item 
#ifdef DEBUG
										cout << "s3 sl_insert: attempting to update the new item's link from " << old_next << " to " << nexts[i] << endl;
#endif
										other = SYNC_CAS(&new_item->next[i], old_next, reinterpret_cast<markable_t>(nexts[i]));
										//If another thread is removing this item we can stop linking it into to skiplist
										if(HAS_MARK(other)) {
											find_preds(NULL, NULL, 0, sl, key, FORCE_UNLINK);
											return 0;
										}
									}
								}while(1);
							}

							// INn case another thread was in the process of remoiiving the <new_item> while we were added it. we 
							// have to make sure it is completely unlinked before we return. We might have lost a race and inserte
							// the new item at some level after the other thrad thought it was fully removed. That is a problem 
							// because oince a thread thinks it completely unlinks a node it queues it to be freed
							if(HAS_MARK(new_item->next[new_item->num_levels-1])) {
								find_preds(NULL, NULL, 0 , sl, key, FORCE_UNLINK);
							}
						} //else of nexts[0] does not exist
						return 0;
					}

					value_type sl_cas(skiplist_t *sl, key_type key, value_type expectation, value_type new_val) {
#ifdef DEBUG
						//	cout << " s1 sl_cas: key   " << key << " skiplist " << sl << endl;
						//		cout << " s1 sl_cas:expectation " << expectation << " new value " << new_val << endl;
#endif
						node_t *preds[MAX_LEVELS];
						node_t *nexts[MAX_LEVELS];
						node_t *new_item = NULL;
						int n = random_levels(sl);
						node_t *old_item = find_preds(preds, nexts, n, sl, key, ASSIST_UNLINK);

						//If there is already an item in the skiplist that matched the key just update its value
						if(old_item != NULL) {
							value_type ret_val = update_item(old_item, expectation, new_val);
							if(ret_val != 0)
								return ret_val;

							//If we lose a race with a thread removing the item we tried to update then we haveto retty
							return sl_cas(sl, key, expectation, new_val); //tail call
						}
						//it old_item == NULL, preds[0]
						if(EXPECT_FALSE(expectation != CAS_EXPECT_DOES_NOT_EXIST && expectation != CAS_EXPECT_WHATEVER)) {
#ifdef DEBUG
							//						cout << " s1 sl_cas: the expectation was not met, the skiplist was not changed " << endl;
#endif
							return 0;
						}
						//Create a new node and insert it into the skiplist
#ifdef DEBUG
						//					cout << " s3 sl_cas: attempting to insert a new item between " << preds[0] << " and " << nexts[0]  << " with  " << n << " levels "<<endl;
#endif
						//allocate a new skiplist node
						key_type new_key = sl->type == NULL ? key : reinterpret_cast<key_type>(sl->type->clone(reinterpret_cast<void *>(key)));
						new_item = node_alloc(n, new_key, new_val);

						//Set <new_item> into <sl> from the bottom level up. After <>;
						markable_t next = new_item->next[0] = reinterpret_cast<markable_t>(nexts[0]);
						for(int level = 1; level < new_item->num_levels;  ++level) {
							new_item->next[level] = reinterpret_cast<markable_t>(nexts[level]);	
#ifdef DEBUG
							cout << "new_item: " <<new_item << " next[" << level <<"] is " <<new_item->next[level]<<endl; 
#endif
						}
						//Link <new）item> into <sl> from the bottom level up. After <new_item> is insert into the bottom level			   //it is officially part of the skiplist 
						node_t *pred = preds[0];
						markable_t other = SYNC_CAS(&pred->next[0], next, reinterpret_cast<markable_t>(new_item));
						if(other != next) {
#ifdef DEBUG
							//						cout <<"s3 sl_cas: failed to change pred's link: expected " << next << " found " << other << endl;
#endif
							//Lost a race to another thread modifying the skiplist. Free the new item we allocated and retry
							if(sl->type != NULL) {
								free(reinterpret_cast<void *>(new_key));
							}
							free(new_item);
							return sl_cas(sl, key, expectation, new_val); //tail call
						}
#ifdef DEBUG
						//					cout << "s3 sl_cas: successfully inserted a new item " << new_item << " at the bottom level " << endl;
#endif
						for(int level = 1; level < new_item->num_levels; ++level) {
#ifdef DEBUG
							//						cout << "s3 sl_cas: inserting the new item " << new_item << " at level " << level << endl;
#endif
							do {
								node_t * pred = preds[level];

								markable_t other = SYNC_CAS(&pred->next[level], (markable_t)nexts[level], (markable_t)new_item);
								if(other == reinterpret_cast<markable_t>(nexts[level]))
									break;
#ifdef DEBUG
								//							cout << "s3 sl_cas: lost a race, failed to change pred's link. expected " << nexts[level] << " found " << other << endl;
#endif
								find_preds(preds, nexts, new_item->num_levels, sl, key, ASSIST_UNLINK);

								for(int i = level; i < new_item->num_levels; ++i) {
									markable_t old_next = new_item->next[i];
									if(reinterpret_cast<markable_t>(nexts[i]) == old_next)
										continue;

									///update <new_items>'s inconsistent next pointer before trying again. Use a CAS so if 
									//another thread is trying to remove the new item concurrently we do not stop on the mark
									//it places on the item 
#ifdef DEBUG
									//								cout << "s3 sl_cas: attempting to update the new item's link from " << old_next << " to " << nexts[i] << endl;
#endif
									other = SYNC_CAS(&new_item->next[i], old_next, reinterpret_cast<markable_t>(nexts[i]));
									//If another thread is removing this item we can stop linking it into to skiplist
									if(HAS_MARK(other)) {
										find_preds(NULL, NULL, 0, sl, key, FORCE_UNLINK);
										return 0;
									}
								}
							}while(1);
						}

						// In case another thread was in the process of remoiiving the <new_item> while we were added it. we 
						// have to make sure it is completely unlinked before we return. We might have lost a race and inserte
						// the new item at some level after the other thrad thought it was fully removed. That is a problem 
						// because oince a thread thinks it completely unlinks a node it queues it to be freed
						if(HAS_MARK(new_item->next[new_item->num_levels-1])) {
							find_preds(NULL, NULL, 0 , sl, key, FORCE_UNLINK);
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
						node_t *item = find_preds(preds, NULL, sl->high_water, sl, key, ASSIST_UNLINK);
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
						value_type val = SYNC_SWAP(&item->val, NULL);
#ifdef DEBUG
						cout << " sw sl_remove: replaced item " << item << " 's value whit NULL " << endl;
#endif
						//unlink the item
						find_preds(NULL, NULL, 0, sl, key, FORCE_UNLINK);

						//free the node
						if(sl->type !=NULL) {
							free(static_cast<void *>(item->key));	
						}
						free(item);

						return val;
					}

					sl_iter * sl_iter_begin(skiplist_t *sl, key_type key) {
						sl_iter *iter = static_cast<sl_iter *>(malloc(sizeof(sl_iter)));
						if(key != NULL) {
							find_preds(NULL, &iter->next, 1, sl, key ,DONT_UNLINK);
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
							*key_ptr = item->key;
						}
						return item->val;
					}

					void sl_iter_free ( sl_iter *iter) {
						free(iter);
					}































				};
		}
#endif

