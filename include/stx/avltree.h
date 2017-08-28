#ifndef _AVLTREE_H_
#define _AVLTREE_H_

#include <string.h>
#include <stdio.h>
#include <algorithm>
#include <functional>
#include <istream>
#include <ostream>
#include <memory>
#include <cstddef>
#include <cassert>

#define AVL_MAX(a, b)	((a) < (b)? (b) : (a))

using namespace std;
namespace stx{
/*
 * Generates default traits for a AVL tree used as a set/
 * */
template <typename _Key>
class avltree_default_set_traits
{
public:
}

/*
 * Basic implementation of a memory AVL tree data structure.
 * */

template <typename _Key, typename _Value>
class avltree
{
public:
	/*Template Parameter Types*/
	/*First template parameter: the key type of the AVL-tree this is stored in the inner nodes and leaves*/
	typedef _Key key_type;

	///Second template parametse:
	typedef _Value value_type;


public:
	typedef avltree<key_type, value_type> self_type;

	typedef size_t size_type;

	typedef std::pair<key_type, value_type> pair_type;
private:
	/// Node classes for in-Memory nodes
	struct node
	{
		
	}
	
}





}
#endif
