#ifndef STX_STX_TTREE_MULTIMAP_H_HEADER
#define STX_STX_TTREE_MULTIMAP_H_HEADER

/* ttree_multimap.h
 * Contains the specialized T tree template class tree_set
 * */

#include <stx/ttree.h>
namespace stx {
template <typename _Key,
		  typename _Compare = std::less<_Key>,
		  typename _Traits = ttree_default_set_traits<_Key>,
		  typename _Alloc = std::allocator<_Key> >	
class ttree_multimap
{
public:
	typedef _Key key_type;
	typedef _Compare key_compare;
	typedef _Traits traits;
	typedef _Alloc allocator_type;
private:
/// \internal The empty struct used as a placeholder for the data_type
	struct empty_struct 
	{};
public:
/// *** Constructed Types

	typedef struct empty_struct data_type;

	typedef std::pair<int, int> value_type;

	typedef ttree_multimap<key_type, key_compare, traits, allocator_type> self_type;

	typedef stx::CTtree ttree_impl;

	typedef ttree_impl::iterator iterator;

private:
	ttree_impl tree;
public:
	inline void insert(int key, int value)
	{
		return tree.Insert(key, value);
	}

	inline void insert(const value_type& x)
	{
		return insert(x.first,x.second);
	}

	inline int find(int x){
		return tree.Find(x);
	}

	inline void erase(int x){
		tree.Delete(x);
	}
};
}
#endif
