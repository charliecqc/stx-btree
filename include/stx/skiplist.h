#ifndef STX_SKIPLIST_HEADER
#define STX_SKIPLIST_HEADER
#include <algorithm>
#include <functional>
#include <istream>
#include <ostream>
#include <memory>
#include <stdlib.h>



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
#define MARK_NODE(x) TAG_VALUE((markable_t)(x), 0x1)
#define HAS_MARK(x) (IS_TAGGED((x), 0x1) == 0x1)
#define GET_NODE(x) ((node_t *)(x))
#define STRIP_MARK(x) ((node_t *)STRIP_TAG((x), 0x1))

enum unlink {
	FORCE_UNLINK,
	ASSIST_UNLINK,
	DONT_UNLINK
};

namespace stx {
	template <typename _Key,  typename _Value>	
		class skiplist
		{
			public:

				typedef _Key key_type;

				typedef _Value value_type;

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
				typedef skiplist<key_type, value_type> self_type;

				/// Size type usedjkj to count keys
				typedef size_t size_type;

				typedef std::pair<key_type, value_type> pair_type;

			private:
				static const int randseed = 34234235;



			private:
				typedef struct node {
					key_type key;
					value_type val;
					unsigned num_levels;
					markable_t next[1];
				} node_t;

				struct sl_iter {
					node_t *next;
				};

				typedef struct sl {
					node_t *head;
					const datatype_t *type;
					int high_water;	//max historic number of levels
				} skiplist_t;

			private:
				int random_levels (skiplist_t *sl) {
					srand(randseed);	
					unsigned long long r = rand();
					int z = __buildin_ctz(r);
					int levels = (int)(z / 1.5);
					if(levels == 0)
						return 1;
					if(levels > sl->high_water) {
						levels = SYNC_ADD(&sl->high_water, 1); // in case of unusual large level
					}
				}

				node_t *node_alloc(int num_levels, key_type key, value_type val) {
					assert(num_levels >= 0 && num_levels <= MAX_LEVELS)
						size_t sz = sizeof(node_t) + (num_levels - 1) * sizeof(node_t *);
					node_t *item = (node_t *)malloc(sz); //use new memory allocator later
					memset(item, 0, size);
					item->key = key;
					item->val = val;
					item->num_levels = num_levels;
#ifdef DEBUG 
					cout << "s2 node_alloc : new node " << item << num_levels << "levels" << endl;
#endif
					return item;
				}

				skiplist_t *sl_alloc (const datatype_t *type) {
					skiplist_t *sl = (skiplist_t *)malloc(sizeof(skiplist_t));
					sl->type = type;
					sl->high_water = 1;
					sl->head = node_alloc(MAX_LEVELS, 0, 0);
					memset(sl->head->next, 0, MAX_LEVELS * sizeof(skiplist_t *));
					return sl;
				}

				void sl_free (skiplist_t *sl) {
					node_t *item = GET_NODE(sl->head->next[0]);
					while(item) {
						if(!HAS_MARK(item->next[0])) {
							count++;
						}
						item = STRIP_MARK(item->next[0]);
					}
					return count;
				}

				node_t *find_preds(node_t **preds, node_t **succs, int n, skiplist_t *sl, key_type key, enum unlink unlink) {
					node_t *pred = sl->head;
					node_t *item = NULL;
#ifdef DEBUG
					cout << "s2 find_preds: searchingfor key " << key << " in skiplist (head is " << pred << " )"<<endl;
#endif
					for(int level = sl->high_water - 1; level >= 0; --level) {
						markable_t next = pred->next[level];
						if(next == NULL && level >= n) 
							continue;	
#ifdef DEBUG
						cout << "s3 find_preds: traversing level " << level << " starting at " << pred <<endl;
#endif
						if(EXPECT_FALSE(HAS_MARK(next))) {
#ifdef DEBUG
							cout << "s2 find_preds: pred " << pred << " is marked for removal (next  " << next << " ), retry "<<endl;
#endif
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
									if(EXPECT_FALSE(item == NULL))
										break;
									next = item->next[level];
								}else {
									// Unlink logically removed items
									markable_t other = SYNC_CAS(&pred->next[level], (markable_t)item, (markable_t)STRIP_MARK(next));
									if(other == (markable_t)item) {
#ifdef DEBUG
										cout << "s3 find_preds: unlinked item from pred " << pred << endl;
#endif
										item = STRIP_MARK(next);

									}else {
#ifdef DEBUG
										cout << "s3 find_preds: lost race to unlink item pred " << pred << " 's link changed to " << other << endl;
#endif
										if(HAS_MARK(other))
											return find_preds(preds, succs, n, sl, key, unlink); //retry
										item = GET_NODE(other);
									}
									next = (item != NULL) ? item->next[level] : NULL;
								}
							}

						if (EXPECT_FALSE(item == NULL)) {
#ifdef DEBUG
							cout << "s3 find_preds: past the last item in the skiplist " << endl;
#endif
							break;	
						}
#ifdef DEBUG
						cout << "s4 find_preds: visiting item " << item << " next is  " << next << endl;
						cout << "s4 find_preds: key " << STRIP_MARK(item->key) << " val " << item->val << endl;
#endif
						if(EXPECT_TRUE(sl->key_type) == NULL) {
							d = item->key - key;
						}else {
							d = sl->key_type->cmp((void *)item->key, (void *)key);
						}

						if(d > 0)
							break;
						if (d == 0 && unlink != FORCE_UNLINK_)
							break;

						ored = item;
						item = GET_NODE(next);
					}
#ifdef DEBUG
					cout << "s3 find_preds: found pred " << pred << " next " << next << endl;
#endif
					if(level < n) {
						if(preds != NULL) {
							preds[level] = preds;
						}
						if(succs != NULL) {
							succs[level] = item;
						}
					}
				}
				if (d == 0) {
#ifdef DEBUG
					cout << "s2 find_preds: found matching item " << item << " in skiplist, pred is " << pred << endl;
#endif
					return item;
				}
#ifdef DEBUG
				cout << "s2 find_preds: found proper place for key " << key << " in skiplist, pred is " << pred << " returning null " << endl;
				return NULL;
#endif

		}

		value_type sl_lookup(skiplist_t *sl, key_type key) {
#ifdef DEBUG
			cout << "s1 sl_lookup: searching for key " << key << " in skiplist " << sl << endl;
#endif
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
			if(EXPECT_FALSE(old_val == NULL)) {
#ifdef DEBUG
				cout << "s2 update_item: lost a race to another thread removing the item, retry " << endl;
#endif
				return NULL; //retry
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
				return old_val //success
			}
#ifdef DEBUG
			cout << "s2 update_item: lost a race. the CAS failed. another thread chhhhanged the item's value'"<<endl;
#endif
			return update_item(item, expectation, new_val) //tail call
		}

		value_type sl_cas(skiplist_t *sl, key_type key, value_type expectation, value_type new_val) {
#ifdef DEBUG
			cout << " s1 sl_cas: key " << key << " skiplist " << sl << endl;
			cout << " s1 sl_cas:expectation " << expectation << " new value " << new_val << endl;
#endif
			node_t *preds[MAX_LEVELS];
			node_t *nexts[MAX_LEVELS];
			node_t *new_item = NULL;
			int n = random_levels(sl);
			node_t *old_item = find_preds(preds, nexts, n, sl, key, ASSIST_UNLINK);

			//If there is already an item in the skuolist that matched the key just update its value
			if(old_item != NULL) {
				value_type ret_val = update_item(old_item, expectation, new_val);
				if(ret_val != NULL)
					return ret_val;

				//If we lose a race with a thread removing the item we tried to update then we haveto retty
				return sl_cas(sl, key, expectation, new_val); //tail call
			}

			if(EXPECT_FALSE(expectation != CAS_EXPECT_DOES_NOT_EXIST && expectation != CAS_EXPECT_WHATEVER)) {
#ifdef DEBUG
				cout << " s1 sl_cas: the expectation was not met, the skiplist was not changed " << endl;
#endif
				return NULL;
			}
			//Create a new node and insert it into the skiplist
#ifdef DEBUG
			cout << " s3 sl_cas: attempting to insert a new item between " << preds[0] << " and " << nexts[0] << endl;
#endif
			key_type new_key = sl->key_type == NULL ? key : (key_type)sl->key_type->clone((void *)key);
			new_item = node_alloc(n, new_key, new_val);

			//Set <new_item> into <sl> from the bottom level up. After <>
			markable_t next = new_item->next[0] = (markable_t)nexts[0];
			for(int level = 1; level < new_item->num_levels;  ++level) {
				new_item->next[level] = (markable_t)nexts[level];	
			}
			//Link <new）item> into <sl> from the bottom level up. After <new_item> is insert into the bottom level			   //it is officially part of the skiplist 
			node_t *pred = preds[0];
			markable_t other = SYNC_CAS(&pred->next[0], next, (markable_t)new_item);
			if(other != next) {
#ifdef DEBUG
				cout <<"s3 sl_cas: failed to change pred's link: expected " << next << " found " << other << endl;
#endif
				//Lost a race to another thread modifying the skiplist. Free the new item we allocated and retry
				if(sl->key_type != NULL) {
					free((void *)new_key);
				}
				free(new_item);
				return sl_cas(sl, key, expectation, new_val) //tail call
			}
#ifdef DEBUG
			cout << "s3 sl_cas: successfully inserted a new item" << new_item << " at the bottom level" << endl;
#endif
			for(int level = 1; level < new_item->num_levels; ++level) {
#ifdef DEBUG
				cout << "s3 sl_cas: inserting the new item " << new_item << " at level " << level << endl;
#endif
				do {
					node_t * pred = preds[level];
#ifdef DEBUG
					markable_t other = SYNC_CAS(&pred->next[level], (markable_t)nexts[level], (markable_t)new_item);
					if(other == (markable_t)nexts[level])
						break;
					cout << "s3 sl_cas: lost a race, failed to change pred's link. expected " << nexts[level] << " found " << other << endl;
#endif
					find_preds(preds, nexts, new_item->num_levels, sl, key, ASSIST_UNLINK);

					for(int i = level; i < new_item->num_levels; ++i) {
						markable_t old_next = new_item->next[i];
						if((markable_t)nexts[i] == old_next)
							continue;
						
						///update <new_items>'s inconsistent next pointer before trying again. Use a CAS so if 
						//another thread is trying to remove the new item concurrently we do not stop on the mark
						//it places on the item 
#ifdef DEBUG
						cout << "s3 sl_cas: attempting to update the new item's link from " << old_next << " to " << nexts[i] << endl;
#endif
						other = SYNC_CAS(&new_item->next[i], old_next, (markable_t)nexts[i]);
						//If another thread is removing this item we can stop linking it into to skiplist
						if(HAS_MARK(other)) {
							find_preds(NULL, NULL, 0, sl, key, FORCE_UNLINK);
							return NULL;
						}
					}
				}while(1);
			}

			// INn case another thread was in the process of remoiiving the <new_item> while we were added it. we 
			// have to make sure it is completely unlinked before we return. We might have lost a race and inserte
			// the new item at some level after the other thrad thought it was fully removed. That is a problem 
			// because oince a thread thinks it completely unlinks a node it queues it to be freed
			if(HAS_MARK(new_iteme->next[new_item->num_levels-1])) {
				find_preds(NULL, NULL, 0 , sl, key, FORCE_UNLINK);
			}
			return NULL;
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
			if(item == NULL {
#ifdef DEBUG
				cout << "s3 sl_remove: remove failed, an item with a matching key does not exist in the skiplist" << endl;
#endif
				return NULL
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
					old_next = SYNC_CAS(&item->next[level], next, MARK_NODE((node_t *)next));
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
			if(sl->key_type !=NULL) {
				free((void *)item->key);	
			}
			free(item);

			return val;
		}

		sl_iter * sl_iter_begin(skiplist_t *sl, key_type key) {
			sl_iter *iter = (sl_iter *)malloc(sizeof(sl_ite));
			if(key != NULL) {
				find_preds(NULL, &iter->next, 1, sl, key DONT_UNLINK);
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

