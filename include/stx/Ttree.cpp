//
// Ttree.cpp: implementation file
//
// Copyright (C) liuxuezong.  All rights reserved
//
// This source is free to use as you like.  If you make
// any changes please keep me in the loop.  Email them to
// liuxuezong@126.com.
//
// PURPOSE:
//
//  To implement mutexes as a C++ object
//
// REVISIONS
// =======================================================
// Date: 2010.12.03
// Name: liuxuezong
// Description: File creation
//
// Date:
// Name:
// Description:
//
//
//////////////////////////////////////////////////////////////////////
#include "Ttree.h"
#include <math.h>
#include <algorithm>
#include <assert.h>
#include "LinkedQueue.h"
#include <stdio.h>
#include <iostream>
#include <cstring>
using namespace std;

TTREENODE* FindMin(TTREENODE *pNode)
{
    if (pNode != NULL)
    {
        if (pNode->left == NULL)
        {
            return pNode;
        }
        else
        {
            return FindMin(pNode->left);
        }
    }
    return NULL;
}

TTREENODE* FindMax(TTREENODE *pNode)
{
    if (pNode != NULL)
    {
        if (pNode->right == NULL)
        {
            return pNode;
        }
        else
        {
            return FindMax(pNode->right);
        }
    }
    return NULL;
}

CTtree::CTtree()
{
    root        = NULL;
    m_nSize     = 0;
}

CTtree::~CTtree()
{
    Clear();
    root        = NULL;
    m_nSize     = 0;
}

int CTtree::GetNodeSize()
{
    return m_nSize;
}

ElementData CTtree::Find(ElementType key)
{
    TTREENODE *pNode = root;
    while (pNode != NULL)
    {
        int n = pNode->nItems;
        ElementType keymin = pNode->item[0];
        ElementType keymax = pNode->item[n > 0 ? n - 1 : 0];
        int nDiff1 = keycompare(key, keymin);
        int nDiff2 = keycompare(key, keymax);
        if (nDiff1 >= 0 && nDiff2 <= 0)
        {
            int l = 0, r = n-1;
            // Binary search.
            while (l <= r)
            {
                int i = (l + r) >> 1;
                ElementType itemkey = pNode->item[i];
                int nDiff = keycompare(key, itemkey);
                if (nDiff == 0)
                {
                    return pNode->data[i];
                }
                else if (nDiff > 0)
                {
                    l = i + 1;
                }
                else
                {
                    r = i - 1;
                }
            }
            break;
        }
        else if (nDiff1 < 0)
        {
            pNode = pNode->left;
        }
        else if (nDiff2 > 0)
        {
            pNode = pNode->right;
        }
    }
    return -1;
}

int CTtree::BalanceFactor(TTREENODE *pNode) const
{
    int l, r;
    TTREENODE *p1, *p2;
    l = r = 0;
    p1 = p2 = pNode;
    if (p1 != NULL)
    {
        while (p1->left != NULL)
        {
            p1 = p1->left;
            l++;
        }
    }
    if (p2 != NULL)
    {
        while (p2->right != NULL)
        {
            p2 = p2->right;
            r++;
        }
    }
    return (r - l);
}

int CTtree::Depth()
{
    int l, r;
    TTREENODE *p1, *p2;
    l = r = 0;
    p1 = p2 = root;
    if (p1 != NULL)
    {
        while (p1->left != NULL)
        {
            p1 = p1->left;
            l++;
        }
    }
    if (p2 != NULL)
    {
        while (p2->right != NULL)
        {
            p2 = p2->right;
            r++;
        }
    }
    return max(l, r);
}

const TTREENODE *CTtree::GetMinNode()
{
    return FindMin(root);
}

const TTREENODE *CTtree::GetMaxNode()
{
    return FindMax(root);
}

int CTtree::Max( int a, int b ) const
{
    return (a > b ? a : b);
}

/**
* Rotate T-tree node with left child.this is a single rotation for case LL.
* Update balance factor, then return new root.
*/
TTREENODE *CTtree::SingleRotateLeft(TTREENODE *pNode)
{
    TTREENODE *K = pNode->left;
    pNode->left = K->right;
    K->right = pNode;

    // Adjust the balance factor.
    pNode->bf = BalanceFactor(pNode);
    K->bf = BalanceFactor(K);

    return K;  // new root
}

/**
* Rotate T-tree node with right child.this is a single rotation for case RR.
* Update balance factor, then return new root.
*/
TTREENODE *CTtree::SingleRotateRight(TTREENODE *pNode)
{
    TTREENODE *K = pNode->right;
    pNode->right = K->left;
    K->left = pNode;

    // Adjust the balance factor.
    pNode->bf = BalanceFactor(pNode);
    K->bf = BalanceFactor(K);

    return K;  // new root
}

/**
* Rotate T-tree node with left child.this is a double rotation for case LR.
* Update balance factor, then return new root.
*/
TTREENODE *CTtree::DoubleRotateLeft(TTREENODE *pNode)
{
    pNode->left = SingleRotateRight(pNode->left);

    // Adjust the balance factor.
    pNode->bf = BalanceFactor(pNode);

    return SingleRotateLeft(pNode);
}

/**
* Rotate T-tree node with right child.this is a double rotation for case RL.
* Update balance factor, then return new root.
*/
TTREENODE *CTtree::DoubleRotateRight(TTREENODE *pNode)
{
    pNode->right = SingleRotateLeft(pNode->right);

    // Adjust the balance factor.
    pNode->bf = BalanceFactor(pNode);

    return SingleRotateRight(pNode);
}

void CTtree::Insert(ElementType key, ElementData data)
{
    if (root == NULL)
    {
        root = MallocNode();
        root->item[0] = key;
        root->data[0] = data;
        root->nItems = 1;
        root->left = NULL;
        root->right = NULL;
    }
    else
    {
        TTREENODE *pNode = root;
        bool bRet = _insert(pNode, key, data);
        if (pNode != root)
        {
            root = pNode;
        }
    }
}

void CTtree::FreeNode(TTREENODE *pNode)
{
    if (pNode)
    {
        delete pNode;
        pNode = NULL;
    }
}

TTREENODE *CTtree::MallocNode()
{
    TTREENODE *pNode = new TTREENODE;
    memset(pNode, 0, sizeof(TTREENODE));
    m_nSize++;

    return (pNode);
}

bool CTtree::_insert(TTREENODE *&pNode, ElementType key, ElementData data)
{
    int n = pNode->nItems;
    ElementType keymin = pNode->item[0];
    ElementType keymax = pNode->item[n > 0 ? n - 1 : 0];
    int nDiff = keycompare(key, keymin);
    if (nDiff <= 0)
    {
        TTREENODE *pLeftId = pNode->left;
        if ((pLeftId == 0 || nDiff == 0 ) && pNode->nItems != pageSize)
        {
            for (int i = n; i > 0; i--)
            {
                pNode->item[i] = pNode->item[i-1];
                pNode->data[i] = pNode->data[i-1];
            }
            pNode->item[0] = key;
            pNode->data[0] = data;
            pNode->nItems += 1;
            return false;
        }
        if (pLeftId == 0)
        {
            pLeftId = MallocNode();
            pLeftId->item[0] = key;
            pLeftId->data[0] = data;
            pLeftId->nItems += 1;
            pNode->left = pLeftId;
        }
        else
        {
            TTREENODE *pChildId = pLeftId;
            bool bGrow = _insert(pChildId, key, data);
            if (pChildId != pLeftId)
            {
                pNode->left = pLeftId = pChildId;
            }
            if (!bGrow)
            {
                return false;
            }
        }
        if (pNode->bf > 0)
        {
            pNode->bf = 0;
            return false;
        }
        else if (pNode->bf == 0)
        {
            pNode->bf = -1;
            return true;
        }
        else
        {
            if (pLeftId->bf < 0)
            {
                pNode = SingleRotateLeft(pNode); // single LL turn
            }
            else
            {
                pNode = DoubleRotateLeft(pNode); // double LR turn
            }
            return false;
        }

    }
    nDiff = keycompare(key, keymax);
    if (nDiff >= 0)
    {
        TTREENODE *pRightId = pNode->right;
        if ((pRightId == 0 || nDiff == 0 ) && pNode->nItems != pageSize)
        {
            pNode->item[n] = key;
            pNode->data[n] = data;
            pNode->nItems += 1;
            return false;
        }
        if (pRightId == 0)
        {
            pRightId = MallocNode();
            pRightId->item[0] = key;
            pRightId->data[0] = data;
            pRightId->nItems += 1;
            pNode->right = pRightId;
        }
        else
        {
            TTREENODE *pChildId = pRightId;
            bool bGrow = _insert(pChildId, key, data);
            if (pChildId != pRightId)
            {
                pNode->right = pRightId = pChildId;
            }
            if (!bGrow)
            {
                return false;
            }
        }
        if (pNode->bf < 0)
        {
            pNode->bf = 0;
            return false;
        }
        else if (pNode->bf == 0)
        {
            pNode->bf = 1;
            return true;
        }
        else
        {
            if (pRightId->bf > 0)
            {
                pNode = SingleRotateRight(pNode); // single RR turn
            }
            else
            {
                pNode = DoubleRotateRight(pNode); // double RL turn
            }
            return false;
        }
    }

    // Node appears in the middle of the primary key points.
    int l = 1, r = n-1;
    while (l < r)
    {
        int i = (l + r) >> 1;
        ElementType itemkey = pNode->item[i];
        nDiff = keycompare(key, itemkey);
        if (nDiff > 0)
        {
            l = i + 1;
        }
        else
        {
            r = i;
            if (nDiff == 0)
            {
                break;
            }
        }
    }

    // Insert before item[r]
    if (n != pageSize)
    {
        for (int i = n; i > r; i--)
        {
            pNode->item[i] = pNode->item[i-1];
        }
        pNode->item[r] = key;
        pNode->nItems += 1;
        return false;
    }
    else
    {
        ElementType reinsertId;
        ElementData reinsertData;
        // The right than the left subtree subtree weight, into the left equilibrium.
        if (pNode->bf >= 0)
        {
            // Node in the value of the most left out,
            // key inserted into its position in the r-1.
            // Value will be inserted into the left of its left subtree.
            reinsertId = pNode->item[0];
            reinsertData = pNode->data[0];
            for (int i = 1; i < r; i++)
            {
                pNode->item[i-1] = pNode->item[i];
                pNode->data[i-1] = pNode->data[i];
            }
            pNode->item[r-1] = key;
            pNode->data[r-1] = data;

            return _insert(pNode, reinsertId, reinsertData);
        }
        else // The left than the right subtree subtree re-insert the right balance.
        {
            // Node in the value of the most right out,
            // key inserted into the location of its r.
            // The right value will be inserted to its right subtree.
            reinsertId = pNode->item[n-1];
            reinsertData = pNode->data[n-1];
            for (int i = n-1; i > r; i--)
            {
                pNode->item[i] = pNode->item[i-1];
                pNode->data[i] = pNode->data[i-1];
            }
            pNode->item[r] = key;
            pNode->data[r] = data;

            return _insert(pNode, reinsertId, reinsertData);
        }
    }
}

void CTtree::Clear()
{
    _earse(root);
}

void CTtree::_earse(TTREENODE *pNode)
{
    if (pNode == NULL)
    {
        return;
    }

    _earse(pNode->left);

    _earse(pNode->right);

    FreeNode(pNode);
}

void CTtree::Delete(ElementType key)
{
    TTREENODE *pNode = root;
    int h = remove(pNode, key);
    assert(h >= 0);
    if (pNode != root)
    {
        root = pNode;
    }
}

int CTtree::BalanceLeftBranch(TTREENODE *&pNode)
{
    if (pNode->bf < 0)
    {
        pNode->bf = 0;
        return 1;
    }
    else if (pNode->bf == 0)
    {
        pNode->bf = 1;
        return 0;
    }
    else
    {
        TTREENODE *pRightId = pNode->right;
        if (pRightId->bf >= 0)
        {
            pNode = SingleRotateRight(pNode); // single RR turn
            if (pRightId->bf == 0)
            {
                pNode->bf = 1;
                pRightId->bf = -1;
                return 0;
            }
            else
            {
                pNode->bf = 0;
                pRightId->bf = 0;
                return 1;
            }
        }
        else
        {
            pNode = DoubleRotateRight(pNode); // double RL turn
            return 1;
        }
    }
}

int CTtree::BalanceRightBranch(TTREENODE *&pNode)
{
    if (pNode->bf > 0)
    {
        pNode->bf = 0;
        return 1;
    }
    else if (pNode->bf == 0)
    {
        pNode->bf = -1;
        return 0;
    }
    else
    {
        TTREENODE * pLeftId = pNode->left;
        if (pLeftId->bf <= 0)
        {
            pNode = SingleRotateLeft(pNode); // single LL turn
            if (pLeftId->bf == 0)
            {
                pNode->bf = -1;
                pLeftId->bf = 1;
                return 0;
            }
            else
            {
                pNode->bf = 0;
                pLeftId->bf = 0;
                return 1;
            }
        }
        else
        {
            pNode = DoubleRotateLeft(pNode); // double LR turn
            return 1;
        }
    }
    return 0;
}

int CTtree::remove(TTREENODE *&pNode, ElementType key)
{
    int n = pNode->nItems;
    ElementType keymin = pNode->item[0];
    ElementType keymax = pNode->item[n > 0 ? n - 1 : 0];
    int nDiff = keycompare(key, keymin);
    if (nDiff <= 0)
    {
        TTREENODE *pLeftId = pNode->left;
        if (pLeftId != 0)
        {
            TTREENODE *pChildId = pLeftId;
            int h = remove(pChildId, key);
            if (pChildId != pLeftId)
            {
                pNode->left = pChildId;
            }
            if (h > 0)
            {
                return BalanceLeftBranch(pNode);
            }
            else if (h == 0)
            {
                return 0;
            }
        }
        assert (nDiff == 0);
    }
    nDiff = keycompare(key, keymax);
    if (nDiff <= 0)
    {
        for (int i = 0; i < n; i++)
        {
            if (pNode->item[i] == key)
            {
                if (n == 1)
                {
                    if (pNode->right == 0)
                    {
                        TTREENODE *pTempNode = pNode->left;
                        FreeNode(pNode);
                        pNode = pTempNode;
                        return 1;
                    }
                    else if (pNode->left == 0)
                    {
                        TTREENODE *pTempNode = pNode->right;
                        FreeNode(pNode);
                        pNode = pTempNode;
                        return 1;
                    }
                }
                TTREENODE *pLeftId = pNode->left, *pRightId = pNode->right;
                if (n <= minItems)
                {
                    if (pLeftId != 0 && pNode->bf <= 0)
                    {
                        while (pLeftId->right != 0)
                        {
                            pLeftId = pLeftId->right;
                        }
                        while (--i >= 0)
                        {
                            pNode->item[i+1] = pNode->item[i];
                            pNode->data[i+1] = pNode->data[i];
                        }
                        pNode->item[0] = pLeftId->item[pLeftId->nItems-1];
                        pNode->data[0] = pLeftId->data[pLeftId->nItems-1];
                        key = pNode->item[0];
                        TTREENODE *pChildId = pLeftId;
                        int h = remove(pChildId, pNode->item[0]);
                        if (pChildId != pLeftId)
                        {
                            pNode->left = pChildId;
                        }
                        if (h > 0)
                        {
                            h = BalanceLeftBranch(pNode);
                        }
                        return h;
                    }
                    else if (pNode->right != 0)
                    {
                        while (pRightId->left != 0)
                        {
                            pRightId = pRightId->left;
                        }
                        while (++i < n)
                        {
                            pNode->item[i-1] = pNode->item[i];
                            pNode->data[i-1] = pNode->data[i];
                        }
                        pNode->item[n-1] = pRightId->item[0];
                        pNode->data[n-1] = pRightId->data[0];
                        key = pNode->item[n-1];

                        TTREENODE *pChildId = pRightId;
                        int h = remove(pChildId, key);
                        if (pChildId != pRightId)
                        {
                            pNode->right = pChildId;
                        }
                        if (h > 0)
                        {
                            h = BalanceRightBranch(pNode);
                        }
                        return h;
                    }
                }
                while (++i < n)
                {
                    pNode->item[i-1] = pNode->item[i];
                    pNode->data[i-1] = pNode->data[i];
                }
                pNode->nItems -= 1;
                return 0;
            }
        }
    }
    TTREENODE *pRightId = pNode->right;
    if (pRightId != 0)
    {
        TTREENODE *pChildId = pRightId;
        int h = remove(pChildId, key);
        if (pChildId != pRightId)
        {
            pNode->right = pChildId;
        }
        if (h > 0)
        {
            return BalanceRightBranch(pNode);
        }
        else
        {
            return h;
        }
    }
    return -1;
}

bool CTtree::IsEmpty( ) const
{
    return root == NULL;
}

int CTtree::keycompare(ElementType key1, ElementType key2)
{
    int p = key1;
    int q = key2;
    return p < q ? -1 : p == q ? 0 : 1;
}

void CTtree::TraverseTree(TraverseOrder order)
{
    switch (order)
    {
    case PreOrder:
        PreOrderTraverse(root);
        break;
    case InOrder:
        InOrderTraverse(root);
        break;
    case PostOrder:
        PostOrderTraverse(root);
        break;
    case LevelOrder:
        LevelOrderTraverse(root);
        break;
    }
}

void CTtree::InOrderTraverse(TTREENODE *pNode) const
{
    if (pNode != NULL)
    {
        InOrderTraverse(pNode->left);
        int nSize = pNode->nItems;
        for (int i = 0; i < nSize; i++)
        {
            printf("%02d ", pNode->item[i]);
        }
        InOrderTraverse(pNode->right);
    }
}

void CTtree::PostOrderTraverse(TTREENODE *pNode) const
{
    if (pNode != NULL)
    {
        PostOrderTraverse(pNode->left);
        PostOrderTraverse(pNode->right);
        int nSize = pNode->nItems;
        for (int i = 0; i < nSize; i++)
        {
            printf("%02d ", pNode->item[i]);
        }
    }
}

void CTtree::PreOrderTraverse(TTREENODE *pNode) const
{
    if (pNode != NULL)
    {
        int nSize = pNode->nItems;
        for (int i = 0; i < nSize; i++)
        {
            printf("%02d ", pNode->item[i]);
        }
        PreOrderTraverse(pNode->left);
        PreOrderTraverse(pNode->right);
    }
}


void CTtree::LevelOrderTraverse(TTREENODE *pNode) const
{
    if (pNode == NULL)
    {
        return;
    }
    // store siblings of each node in a queue so that they are
    // visited in order at the next level of the tree
    linkedQueue<TTREENODE *> q;
    TTREENODE *p;

    // initialize the queue by inserting the root in the queue
    q.push(pNode);
    // continue the iterative process until the queue is empty
    while (!q.empty())
    {
        // delete front node from queue and output the node value
        p = (TTREENODE *)q.front();
        q.pop();

        int nSize = p->nItems;
        for (int i = 0; i < nSize; i++)
        {
            printf("%02d ", p->item[i]);
        }
        // if a left child exists, insert it in the queue
        if (p->left != NULL)
        {
            q.push(p->left);
        }
        // if a right child exists, insert next to its sibling
        if (p->right != NULL)
        {
            q.push(p->right);
        }
        printf("/n");
    }
    int n = 0;
}
