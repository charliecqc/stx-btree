#ifndef STX_STX_SKIPLIST_MULTIMAP_H_HEADER
#define STX_STX_SKIPLIST_MULTIMAP_H_HEADER

#include <stx/skiplist.h>

namespace stx{
template <typename _Key, typename _Value,
		 typename _Compare = std::less<_Key>,
		 typename _Traits = skiplist_default_set_traits<_Key>,
		 typename _Alloc = std::allocator<_Key> >
class skiplist_multimap
{
public:
	typedef _Key key_type;
	typedef _Value value_type;
	typedef _Compare key_compare;
	typedef _Traits traits;
	typedef _Alloc allocator_type;
private:
	struct empty_struct
	{};
public:
	typedef struct empty_struct empty_type;

	typedef std::pair<key_type, value_type> data_type;

	typedef skiplist_multimap<key_type, value_type, key_compare, traits, allocator_type> self_type;

	typedef stx::skiplist<key_type, value_type, traits> skiplist_impl;

	typedef typename skiplist_impl::sl_iter iterator;

	typedef typename skiplist_impl::skiplist_t sl;
	sl * slist;

private:
		skiplist_impl list;
public:
		skiplist_multimap()
	{
		slist = list.sl_alloc(NULL);
	}
public:
		inline value_type insert(const key_type &key, const value_type &value)
		{
			return list.sl_insert(slist, key, -2, value);
		}

		inline value_type insert(const data_type& x)
		{
			return insert(x.first, x.second);
		}

		inline value_type find(const key_type &key)
		{
			return list.sl_lookup(slist, key);
		}

};

}

#endif
