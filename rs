SkiplistMap
s2 node_alloc : new node 0x1b1cec0 32 levels
 s1 sl_insert_new: going to insert key 507 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 507 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0 next is 0 level is 0
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 0
 find_index_node:  foud proper place for key 507 in skiplist. pred is  returning null 
 s3 sl_insert_new with preds[0]: attempting to insert an new index node between 0x1b1cec0 and 0 0?
s2 node_alloc : new node 0x1b1d270 1 levels
0x1b1d3a0 set key 507 at 0
 s1 sl_insert_new: going to insert key 92 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 92 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 507 sum is 507 next is 0
 s4 find_index_node: key 507
item->max: 507 item->min 507
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 0
 find_index_node:  foud proper place for key 92 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 92 index node exists 0x1b1d270 max: 507 min 507 sum 507
0x1b1d3a0 set key 92 at 1
 s1 sl_insert_new: going to insert key 61 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 61 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 92 sum is 599 next is 0
 s4 find_index_node: key 507
item->max: 507 item->min 92
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 0
 find_index_node:  foud proper place for key 61 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 61 index node exists 0x1b1d270 max: 507 min 92 sum 599
0x1b1d3a0 set key 61 at 2
 s1 sl_insert_new: going to insert key 573 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 573 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 660 next is 0
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s3 find_index_node: foud pred 0x1b1d270 next 0 at level 0
 find_index_node:  foud proper place for key 573 in skiplist. pred is  returning null 
 s3 sl_insert_new with preds[0]: attempting to insert an new index node between 0x1b1d270 and 0 0?
s2 node_alloc : new node 0x1b1d450 1 levels
0x1b1d580 set key 573 at 0
 s1 sl_insert_new: going to insert key 398 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 398 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 660 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
s3 find_index_node: 0x1b1d270 with min 61 max 507 sum 660
 s3 sl_insert_new: going to insert key 398 index node exists 0x1b1d270 max: 507 min 61 sum 660
0x1b1d580 set key 398 at 1
 s1 sl_insert_new: going to insert key 528 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 528 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1058 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 573 sum is 573 next is 0
 s4 find_index_node: key 573
item->max: 573 item->min 573
 s3 find_index_node: foud pred 0x1b1d270 next 0 at level 0
 find_index_node:  foud proper place for key 528 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 528 index node exists 0x1b1d450 max: 573 min 573 sum 573
0x1b1d580 set key 528 at 2
 s1 sl_insert_new: going to insert key 126 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 126 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1058 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
s3 find_index_node: 0x1b1d270 with min 61 max 507 sum 1058
 s3 sl_insert_new: going to insert key 126 index node exists 0x1b1d270 max: 507 min 61 sum 1058
0x1b1d580 set key 126 at 3
 s1 sl_insert_new: going to insert key 872 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 872 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1184 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s3 find_index_node: foud pred 0x1b1d450 next 0 at level 0
 find_index_node:  foud proper place for key 872 in skiplist. pred is  returning null 
 s3 sl_insert_new with preds[0]: attempting to insert an new index node between 0x1b1d450 and 0 0?
s2 node_alloc : new node 0x1b1d630 1 levels
0x1b1d760 set key 872 at 0
 s1 sl_insert_new: going to insert key 186 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 186 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1184 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
s3 find_index_node: 0x1b1d270 with min 61 max 507 sum 1184
 s3 sl_insert_new: going to insert key 186 index node exists 0x1b1d270 max: 507 min 61 sum 1184
0x1b1d580 set key 186 at 4
 s1 sl_insert_new: going to insert key 496 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 496 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1370 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
s3 find_index_node: 0x1b1d270 with min 61 max 507 sum 1370
 s3 sl_insert_new: going to insert key 496 index node exists 0x1b1d270 max: 507 min 61 sum 1370
0x1b1d580 set key 496 at 5
 s1 sl_insert_new: going to insert key 922 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 1
 s2 find_index_node: searching for min key that >= key 922 in skiplist head is 0x1b1cec0 sl->high_water is 1 n is 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1866 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 872 sum is 872 next is 0
 s4 find_index_node: key 872
item->max: 872 item->min 872
 s3 find_index_node: foud pred 0x1b1d630 next 0 at level 0
 find_index_node:  foud proper place for key 922 in skiplist. pred is  returning null 
 s3 sl_insert_new with preds[0]: attempting to insert an new index node between 0x1b1d630 and 0 0?
s2 node_alloc : new node 0x1b1d810 1 levels
0x1b1d940 set key 922 at 0
 s1 sl_insert_new: going to insert key 664 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 664 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1866 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 872 sum is 872 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 872
 s3 find_index_node: foud pred 0x1b1d450 next 0x1b1d810 at level 0
 find_index_node:  foud proper place for key 664 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 664 index node exists 0x1b1d630 max: 872 min 872 sum 872
0x1b1d940 set key 664 at 1
 s1 sl_insert_new: going to insert key 97 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 97 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1866 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
s3 find_index_node: 0x1b1d270 with min 61 max 507 sum 1866
 s3 sl_insert_new: going to insert key 97 index node exists 0x1b1d270 max: 507 min 61 sum 1866
0x1b1d580 set key 97 at 6
 s1 sl_insert_new: going to insert key 706 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 706 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1963 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 664 sum is 1536 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 664
s3 find_index_node: 0x1b1d630 with min 664 max 872 sum 1536
 s3 sl_insert_new: going to insert key 706 index node exists 0x1b1d630 max: 872 min 664 sum 1536
0x1b1d940 set key 706 at 2
 s1 sl_insert_new: going to insert key 589 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 589 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1963 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 664 sum is 2242 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 664
 s3 find_index_node: foud pred 0x1b1d450 next 0x1b1d810 at level 0
 find_index_node:  foud proper place for key 589 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 589 index node exists 0x1b1d630 max: 872 min 664 sum 2242
0x1b1d940 set key 589 at 3
 s1 sl_insert_new: going to insert key 897 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 897 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1963 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 2831 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 922 sum is 922 next is 0
 s4 find_index_node: key 922
item->max: 922 item->min 922
 s3 find_index_node: foud pred 0x1b1d630 next 0 at level 0
 find_index_node:  foud proper place for key 897 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 897 index node exists 0x1b1d810 max: 922 min 922 sum 922
0x1b1d940 set key 897 at 4
 s1 sl_insert_new: going to insert key 19 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 19 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 61 sum is 1963 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 61
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1d450 at level 0
 find_index_node:  foud proper place for key 19 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 19 index node exists 0x1b1d270 max: 507 min 61 sum 1963
0x1b1d580 set key 19 at 7
 s1 sl_insert_new: going to insert key 912 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 912 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 19 sum is 1982 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 19
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 2831 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 897 sum is 1819 next is 0
 s4 find_index_node: key 922
item->max: 922 item->min 897
s3 find_index_node: 0x1b1d810 with min 897 max 922 sum 1819
 s3 sl_insert_new: going to insert key 912 index node exists 0x1b1d810 max: 922 min 897 sum 1819
0x1b1d940 set key 912 at 5
 s1 sl_insert_new: going to insert key 743 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 743 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 19 sum is 1982 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 19
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 2831 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
s3 find_index_node: 0x1b1d630 with min 589 max 872 sum 2831
 s3 sl_insert_new: going to insert key 743 index node exists 0x1b1d630 max: 872 min 589 sum 2831
0x1b1d940 set key 743 at 6
 s1 sl_insert_new: going to insert key 163 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 163 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1d270 next is 0x1b1d270 level is 0
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 19 sum is 1982 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 19
s3 find_index_node: 0x1b1d270 with min 19 max 507 sum 1982
 s3 sl_insert_new: going to insert key 163 index node exists 0x1b1d270 max: 507 min 19 sum 1982
 nvnode 0x1b1d580 is full, need to be split 
 sum of index_node: 0x1b1d270 is 1982
target key 247 temp_key 573
0x1b1d9f0 set key 573 at 0
target key 247 temp_key 398
0x1b1d9f0 set key 398 at 1
target key 247 temp_key 528
0x1b1d9f0 set key 528 at 2
target key 247 temp_key 126
0x1b1daa0 set key 126 at 0
target key 247 temp_key 186
0x1b1daa0 set key 186 at 1
target key 247 temp_key 496
0x1b1d9f0 set key 496 at 3
target key 247 temp_key 97
0x1b1daa0 set key 97 at 2
target key 247 temp_key 19
0x1b1daa0 set key 19 at 3
0x1b1daa0 set key 163 at 4
 orig_nvnode: 0x1b1d9f0 with slot 4 new_leaf 0x1b1daa0 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1cec0 and 0x1b1d270 with new_sum 591
s2 node_alloc : new node 0x1b1db50 1 levels
0x1b1db50 sum is 591 0x1b1d270 sum is 1995
 pred of new_index is 0x1b1cec0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1cec0 next is 0x1b1db50
 s1 sl_insert_new: going to insert key 972 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 972 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1db50 next is 0x1b1db50 level is 0
 s4 find_index_node: visiting item 0x1b1db50 max 186 min 19 sum is 591 next is 0x1b1d270
 s4 find_index_node: key 186
item->max: 186 item->min 19
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 398 sum is 1995 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 398
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 3574 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 897 sum is 2731 next is 0
 s4 find_index_node: key 922
item->max: 922 item->min 897
 s3 find_index_node: foud pred 0x1b1d810 next 0 at level 0
 find_index_node:  foud proper place for key 972 in skiplist. pred is  returning null 
 s3 sl_insert_new with preds[0]: attempting to insert an new index node between 0x1b1d810 and 0 0?
s2 node_alloc : new node 0x1b1dc80 1 levels
0x1b1ddb0 set key 972 at 0
 s1 sl_insert_new: going to insert key 874 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 874 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1db50 next is 0x1b1db50 level is 0
 s4 find_index_node: visiting item 0x1b1db50 max 186 min 19 sum is 591 next is 0x1b1d270
 s4 find_index_node: key 186
item->max: 186 item->min 19
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 398 sum is 1995 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 398
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 3574 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 897 sum is 2731 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 897
 s3 find_index_node: foud pred 0x1b1d630 next 0x1b1dc80 at level 0
 find_index_node:  foud proper place for key 874 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 874 index node exists 0x1b1d810 max: 922 min 897 sum 2731
0x1b1ddb0 set key 874 at 1
 s1 sl_insert_new: going to insert key 652 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 652 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1db50 next is 0x1b1db50 level is 0
 s4 find_index_node: visiting item 0x1b1db50 max 186 min 19 sum is 591 next is 0x1b1d270
 s4 find_index_node: key 186
item->max: 186 item->min 19
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 398 sum is 1995 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 398
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 3574 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
s3 find_index_node: 0x1b1d630 with min 589 max 872 sum 3574
 s3 sl_insert_new: going to insert key 652 index node exists 0x1b1d630 max: 872 min 589 sum 3574
0x1b1d940 set key 652 at 7
 s1 sl_insert_new: going to insert key 905 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 905 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1db50 next is 0x1b1db50 level is 0
 s4 find_index_node: visiting item 0x1b1db50 max 186 min 19 sum is 591 next is 0x1b1d270
 s4 find_index_node: key 186
item->max: 186 item->min 19
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 398 sum is 1995 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 398
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 4226 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 874 sum is 3605 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 874
s3 find_index_node: 0x1b1d810 with min 874 max 922 sum 3605
 s3 sl_insert_new: going to insert key 905 index node exists 0x1b1d810 max: 922 min 874 sum 3605
0x1b1ddb0 set key 905 at 2
 s1 sl_insert_new: going to insert key 124 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 124 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1db50 next is 0x1b1db50 level is 0
 s4 find_index_node: visiting item 0x1b1db50 max 186 min 19 sum is 591 next is 0x1b1d270
 s4 find_index_node: key 186
item->max: 186 item->min 19
s3 find_index_node: 0x1b1db50 with min 19 max 186 sum 591
 s3 sl_insert_new: going to insert key 124 index node exists 0x1b1db50 max: 186 min 19 sum 591
0x1b1daa0 set key 124 at 5
 s1 sl_insert_new: going to insert key 488 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 2
 s2 find_index_node: searching for min key that >= key 488 in skiplist head is 0x1b1cec0 sl->high_water is 2 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1db50 next is 0x1b1db50 level is 0
 s4 find_index_node: visiting item 0x1b1db50 max 186 min 19 sum is 715 next is 0x1b1d270
 s4 find_index_node: key 186
item->max: 186 item->min 19
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 398 sum is 1995 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 398
s3 find_index_node: 0x1b1d270 with min 398 max 507 sum 1995
 s3 sl_insert_new: going to insert key 488 index node exists 0x1b1d270 max: 507 min 398 sum 1995
0x1b1d9f0 set key 488 at 4
 s1 sl_insert_new: going to insert key 825 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 3
 s2 find_index_node: searching for min key that >= key 825 in skiplist head is 0x1b1cec0 sl->high_water is 3 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0 next is 0 level is 2
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1cec0
item is 0x1b1db50 next is 0x1b1db50 level is 0
 s4 find_index_node: visiting item 0x1b1db50 max 186 min 19 sum is 715 next is 0x1b1d270
 s4 find_index_node: key 186
item->max: 186 item->min 19
 s4 find_index_node: visiting item 0x1b1d270 max 507 min 398 sum is 2483 next is 0x1b1d450
 s4 find_index_node: key 507
item->max: 507 item->min 398
 s4 find_index_node: visiting item 0x1b1d450 max 573 min 528 sum is 1101 next is 0x1b1d630
 s4 find_index_node: key 573
item->max: 573 item->min 528
 s4 find_index_node: visiting item 0x1b1d630 max 872 min 589 sum is 4226 next is 0x1b1d810
 s4 find_index_node: key 872
item->max: 872 item->min 589
s3 find_index_node: 0x1b1d630 with min 589 max 872 sum 4226
 s3 sl_insert_new: going to insert key 825 index node exists 0x1b1d630 max: 872 min 589 sum 4226
 nvnode 0x1b1d940 is full, need to be split 
 sum of index_node: 0x1b1d630 is 4226
target key 528 temp_key 922
0x1b1de60 set key 922 at 0
target key 528 temp_key 664
0x1b1de60 set key 664 at 1
target key 528 temp_key 706
0x1b1de60 set key 706 at 2
target key 528 temp_key 589
0x1b1de60 set key 589 at 3
target key 528 temp_key 897
0x1b1de60 set key 897 at 4
target key 528 temp_key 912
0x1b1de60 set key 912 at 5
target key 528 temp_key 743
0x1b1de60 set key 743 at 6
target key 528 temp_key 652
0x1b1de60 set key 652 at 7
0x1b1de60 set key 825 at -1
 orig_nvnode: 0x1b1de60 with slot 9 new_leaf 0x1b1df10 with slot 0
 sl_insert_new: attempting to insert an new index node between 0x1b1d630 and 0x1b1d810 with new_sum 0
s2 node_alloc : new node 0x1b1dfc0 2 levels
0x1b1dfc0 sum is 0 0x1b1d630 sum is 6910
 pred of new_index is 0x1b1d630 sl->head is 0x1b1cec0
preds level 0 is 0x1b1d630 next is 0x1b1dfc0
preds level 1 is 0x1b1cec0 next is 0x1b1dfc0
 s1 sl_insert_new: going to insert key 891 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 3
 s2 find_index_node: searching for min key that >= key 891 in skiplist head is 0x1b1cec0 sl->high_water is 3 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s3 find_index_node: foud pred 0x1b1dfc0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 874 sum is 4510 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 874
s3 find_index_node: 0x1b1d810 with min 874 max 922 sum 4510
 s3 sl_insert_new: going to insert key 891 index node exists 0x1b1d810 max: 922 min 874 sum 4510
0x1b1ddb0 set key 891 at 3
 s1 sl_insert_new: going to insert key 639 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 3
 s2 find_index_node: searching for min key that >= key 639 in skiplist head is 0x1b1cec0 sl->high_water is 3 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s3 find_index_node: foud pred 0x1b1dfc0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 874 sum is 5401 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 874
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1dc80 at level 0
 find_index_node:  foud proper place for key 639 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 639 index node exists 0x1b1d810 max: 922 min 874 sum 5401
0x1b1ddb0 set key 639 at 4
 s1 sl_insert_new: going to insert key 126 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 126 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 3
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0 next is 0 level is 2
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s3 find_index_node: foud pred 0x1b1dfc0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 639 sum is 6040 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 639
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1dc80 at level 0
 find_index_node:  foud proper place for key 126 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 126 index node exists 0x1b1d810 max: 922 min 639 sum 6040
0x1b1ddb0 set key 126 at 5
 s1 sl_insert_new: going to insert key 212 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 212 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s3 find_index_node: foud pred 0x1b1dfc0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 126 sum is 6166 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 126
s3 find_index_node: 0x1b1d810 with min 126 max 922 sum 6166
 s3 sl_insert_new: going to insert key 212 index node exists 0x1b1d810 max: 922 min 126 sum 6166
0x1b1ddb0 set key 212 at 6
 s1 sl_insert_new: going to insert key 344 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 344 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s3 find_index_node: foud pred 0x1b1dfc0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 126 sum is 6378 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 126
s3 find_index_node: 0x1b1d810 with min 126 max 922 sum 6378
 s3 sl_insert_new: going to insert key 344 index node exists 0x1b1d810 max: 922 min 126 sum 6378
0x1b1ddb0 set key 344 at 7
 s1 sl_insert_new: going to insert key 178 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 178 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 3
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0 next is 0 level is 2
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s3 find_index_node: foud pred 0x1b1dfc0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 126 sum is 6722 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 126
s3 find_index_node: 0x1b1d810 with min 126 max 922 sum 6722
 s3 sl_insert_new: going to insert key 178 index node exists 0x1b1d810 max: 922 min 126 sum 6722
 nvnode 0x1b1ddb0 is full, need to be split 
 sum of index_node: 0x1b1d810 is 6722
target key 840 temp_key 972
0x1b1e100 set key 972 at 0
target key 840 temp_key 874
0x1b1e100 set key 874 at 1
target key 840 temp_key 905
0x1b1e100 set key 905 at 2
target key 840 temp_key 891
0x1b1e100 set key 891 at 3
target key 840 temp_key 639
0x1b1e1b0 set key 639 at 0
target key 840 temp_key 126
0x1b1e1b0 set key 126 at 1
target key 840 temp_key 212
0x1b1e1b0 set key 212 at 2
target key 840 temp_key 344
0x1b1e1b0 set key 344 at 3
0x1b1e1b0 set key 178 at 4
 orig_nvnode: 0x1b1e100 with slot 4 new_leaf 0x1b1e1b0 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1dfc0 and 0x1b1d810 with new_sum 1499
s2 node_alloc : new node 0x1b1e260 3 levels
0x1b1e260 sum is 1499 0x1b1d810 sum is 3642
 pred of new_index is 0x1b1dfc0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1dfc0 next is 0x1b1e260
preds level 1 is 0x1b1dfc0 next is 0x1b1e260
preds level 2 is 0x1b1cec0 next is 0x1b1e260
 s1 sl_insert_new: going to insert key 713 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 713 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 126 sum is 1499 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 126
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 874 sum is 3642 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 874
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1dc80 at level 0
 find_index_node:  foud proper place for key 713 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 713 index node exists 0x1b1d810 max: 922 min 874 sum 3642
0x1b1e100 set key 713 at 4
 s1 sl_insert_new: going to insert key 95 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 95 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 126 sum is 1499 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 126
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 126 sum is 1499 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 126
 s3 find_index_node: foud pred 0x1b1dfc0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1e260 next is 0x1b1e260 level is 0
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 126 sum is 1499 next is 0x1b1d810
 s4 find_index_node: key 639
item->max: 639 item->min 126
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1d810 at level 0
 find_index_node:  foud proper place for key 95 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 95 index node exists 0x1b1e260 max: 639 min 126 sum 1499
0x1b1e1b0 set key 95 at 5
 s1 sl_insert_new: going to insert key 219 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 219 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 95 sum is 1594 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 95
s3 find_index_node: 0x1b1e260 with min 95 max 639 sum 1594
 s3 sl_insert_new: going to insert key 219 index node exists 0x1b1e260 max: 639 min 95 sum 1594
0x1b1e1b0 set key 219 at 6
 s1 sl_insert_new: going to insert key 708 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 708 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 95 sum is 1813 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 95
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 713 sum is 4355 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 713
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1dc80 at level 0
 find_index_node:  foud proper place for key 708 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 708 index node exists 0x1b1d810 max: 922 min 713 sum 4355
0x1b1e100 set key 708 at 5
 s1 sl_insert_new: going to insert key 893 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 893 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 95 sum is 1813 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 95
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 708 sum is 5063 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 708
s3 find_index_node: 0x1b1d810 with min 708 max 922 sum 5063
 s3 sl_insert_new: going to insert key 893 index node exists 0x1b1d810 max: 922 min 708 sum 5063
0x1b1e100 set key 893 at 6
 s1 sl_insert_new: going to insert key 264 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 264 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 95 sum is 1813 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 95
s3 find_index_node: 0x1b1e260 with min 95 max 639 sum 1813
 s3 sl_insert_new: going to insert key 264 index node exists 0x1b1e260 max: 639 min 95 sum 1813
0x1b1e1b0 set key 264 at 7
 s1 sl_insert_new: going to insert key 753 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 753 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 95 sum is 2077 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 95
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 708 sum is 5956 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 708
s3 find_index_node: 0x1b1d810 with min 708 max 922 sum 5956
 s3 sl_insert_new: going to insert key 753 index node exists 0x1b1d810 max: 922 min 708 sum 5956
0x1b1e100 set key 753 at 7
 s1 sl_insert_new: going to insert key 879 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 879 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 95 sum is 2077 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 95
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0 next is 0 level is 1
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 708 sum is 6709 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 708
s3 find_index_node: 0x1b1d810 with min 708 max 922 sum 6709
 s3 sl_insert_new: going to insert key 879 index node exists 0x1b1d810 max: 922 min 708 sum 6709
 nvnode 0x1b1e100 is full, need to be split 
 sum of index_node: 0x1b1d810 is 6709
target key 838 temp_key 972
0x1b1e3a0 set key 972 at 0
target key 838 temp_key 874
0x1b1e3a0 set key 874 at 1
target key 838 temp_key 905
0x1b1e3a0 set key 905 at 2
target key 838 temp_key 891
0x1b1e3a0 set key 891 at 3
target key 838 temp_key 713
0x1b1e450 set key 713 at 0
target key 838 temp_key 708
0x1b1e450 set key 708 at 1
target key 838 temp_key 893
0x1b1e3a0 set key 893 at 4
target key 838 temp_key 753
0x1b1e450 set key 753 at 2
0x1b1e3a0 set key 879 at 5
 orig_nvnode: 0x1b1e3a0 with slot 6 new_leaf 0x1b1e450 with slot 3
 sl_insert_new: attempting to insert an new index node between 0x1b1e260 and 0x1b1d810 with new_sum 2174
s2 node_alloc : new node 0x1b1e500 2 levels
0x1b1e500 sum is 2174 0x1b1d810 sum is 5414
 pred of new_index is 0x1b1e260 sl->head is 0x1b1cec0
preds level 0 is 0x1b1e260 next is 0x1b1e500
preds level 1 is 0x1b1e260 next is 0x1b1e500
 s1 sl_insert_new: going to insert key 151 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 151 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 95 sum is 2077 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 95
s3 find_index_node: 0x1b1e260 with min 95 max 639 sum 2077
 s3 sl_insert_new: going to insert key 151 index node exists 0x1b1e260 max: 639 min 95 sum 2077
 nvnode 0x1b1e1b0 is full, need to be split 
 sum of index_node: 0x1b1e260 is 2077
target key 259 temp_key 639
0x1b1e640 set key 639 at 0
target key 259 temp_key 126
0x1b1e6f0 set key 126 at 0
target key 259 temp_key 212
0x1b1e6f0 set key 212 at 1
target key 259 temp_key 344
0x1b1e640 set key 344 at 1
target key 259 temp_key 178
0x1b1e6f0 set key 178 at 2
target key 259 temp_key 95
0x1b1e6f0 set key 95 at 3
target key 259 temp_key 219
0x1b1e6f0 set key 219 at 4
target key 259 temp_key 264
0x1b1e640 set key 264 at 2
0x1b1e6f0 set key 151 at 5
 orig_nvnode: 0x1b1e640 with slot 3 new_leaf 0x1b1e6f0 with slot 6
 sl_insert_new: attempting to insert an new index node between 0x1b1e260 and 0x1b1d810 with new_sum 981
s2 node_alloc : new node 0x1b1e7a0 1 levels
0x1b1e7a0 sum is 981 0x1b1e260 sum is 1247
 pred of new_index is 0x1b1e260 sl->head is 0x1b1cec0
preds level 0 is 0x1b1e260 next is 0x1b1e7a0
 s1 sl_insert_new: going to insert key 579 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 579 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 1247 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 264
s3 find_index_node: 0x1b1e260 with min 264 max 639 sum 1247
 s3 sl_insert_new: going to insert key 579 index node exists 0x1b1e260 max: 639 min 264 sum 1247
0x1b1e640 set key 579 at 3
 s1 sl_insert_new: going to insert key 564 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 564 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 1826 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 264
s3 find_index_node: 0x1b1e260 with min 264 max 639 sum 1826
 s3 sl_insert_new: going to insert key 564 index node exists 0x1b1e260 max: 639 min 264 sum 1826
0x1b1e640 set key 564 at 4
 s1 sl_insert_new: going to insert key 883 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 883 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 2390 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 264
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e500 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e500
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 874 sum is 5414 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 874
s3 find_index_node: 0x1b1d810 with min 874 max 922 sum 5414
 s3 sl_insert_new: going to insert key 883 index node exists 0x1b1d810 max: 922 min 874 sum 5414
0x1b1e3a0 set key 883 at 6
 s1 sl_insert_new: going to insert key 811 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 811 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 2390 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 264
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e500 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e500
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 874 sum is 6297 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 874
 s3 find_index_node: foud pred 0x1b1e500 next 0x1b1dc80 at level 0
 find_index_node:  foud proper place for key 811 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 811 index node exists 0x1b1d810 max: 922 min 874 sum 6297
0x1b1e3a0 set key 811 at 7
 s1 sl_insert_new: going to insert key 906 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 906 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 2390 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 264
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e500 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e500
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 811 sum is 7108 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 811
s3 find_index_node: 0x1b1d810 with min 811 max 922 sum 7108
 s3 sl_insert_new: going to insert key 906 index node exists 0x1b1d810 max: 922 min 811 sum 7108
 nvnode 0x1b1e3a0 is full, need to be split 
 sum of index_node: 0x1b1d810 is 7108
target key 888 temp_key 972
0x1b1e8d0 set key 972 at 0
target key 888 temp_key 874
0x1b1e980 set key 874 at 0
target key 888 temp_key 905
0x1b1e8d0 set key 905 at 1
target key 888 temp_key 891
0x1b1e8d0 set key 891 at 2
target key 888 temp_key 893
0x1b1e8d0 set key 893 at 3
target key 888 temp_key 879
0x1b1e980 set key 879 at 1
target key 888 temp_key 883
0x1b1e980 set key 883 at 2
target key 888 temp_key 811
0x1b1e980 set key 811 at 3
0x1b1e8d0 set key 906 at 4
 orig_nvnode: 0x1b1e8d0 with slot 5 new_leaf 0x1b1e980 with slot 4
 sl_insert_new: attempting to insert an new index node between 0x1b1e500 and 0x1b1d810 with new_sum 3447
s2 node_alloc : new node 0x1b1ea30 2 levels
0x1b1ea30 sum is 3447 0x1b1d810 sum is 4567
 pred of new_index is 0x1b1e500 sl->head is 0x1b1cec0
preds level 0 is 0x1b1e500 next is 0x1b1ea30
preds level 1 is 0x1b1e500 next is 0x1b1ea30
 s1 sl_insert_new: going to insert key 102 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 102 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 2390 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 264
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 2390 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 264
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1e260 next is 0x1b1e260 level is 0
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 264 sum is 2390 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 264
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e7a0 at level 0
 find_index_node:  foud proper place for key 102 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 102 index node exists 0x1b1e260 max: 639 min 264 sum 2390
0x1b1e640 set key 102 at 5
 s1 sl_insert_new: going to insert key 249 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 249 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 102 sum is 2492 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 102
s3 find_index_node: 0x1b1e260 with min 102 max 639 sum 2492
 s3 sl_insert_new: going to insert key 249 index node exists 0x1b1e260 max: 639 min 102 sum 2492
0x1b1e640 set key 249 at 6
 s1 sl_insert_new: going to insert key 463 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 463 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 102 sum is 2741 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 102
s3 find_index_node: 0x1b1e260 with min 102 max 639 sum 2741
 s3 sl_insert_new: going to insert key 463 index node exists 0x1b1e260 max: 639 min 102 sum 2741
0x1b1e640 set key 463 at 7
 s1 sl_insert_new: going to insert key 263 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 263 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 102 sum is 3204 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 102
s3 find_index_node: 0x1b1e260 with min 102 max 639 sum 3204
 s3 sl_insert_new: going to insert key 263 index node exists 0x1b1e260 max: 639 min 102 sum 3204
 nvnode 0x1b1e640 is full, need to be split 
 sum of index_node: 0x1b1e260 is 3204
target key 400 temp_key 639
0x1b1eb70 set key 639 at 0
target key 400 temp_key 344
0x1b1ec20 set key 344 at 0
target key 400 temp_key 264
0x1b1ec20 set key 264 at 1
target key 400 temp_key 579
0x1b1eb70 set key 579 at 1
target key 400 temp_key 564
0x1b1eb70 set key 564 at 2
target key 400 temp_key 102
0x1b1ec20 set key 102 at 2
target key 400 temp_key 249
0x1b1ec20 set key 249 at 3
target key 400 temp_key 463
0x1b1eb70 set key 463 at 3
0x1b1ec20 set key 263 at 4
 orig_nvnode: 0x1b1eb70 with slot 4 new_leaf 0x1b1ec20 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1dfc0 and 0x1b1e260 with new_sum 1222
s2 node_alloc : new node 0x1b1ecd0 1 levels
0x1b1ecd0 sum is 1222 0x1b1e260 sum is 2245
 pred of new_index is 0x1b1dfc0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1dfc0 next is 0x1b1ecd0
 s1 sl_insert_new: going to insert key 415 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 415 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 463 sum is 2245 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 463
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 463 sum is 2245 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 463
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1ecd0 next is 0x1b1ecd0 level is 0
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 102 sum is 1222 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 102
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 463 sum is 2245 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 463
 s3 find_index_node: foud pred 0x1b1ecd0 next 0x1b1e7a0 at level 0
 find_index_node:  foud proper place for key 415 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 415 index node exists 0x1b1e260 max: 639 min 463 sum 2245
0x1b1eb70 set key 415 at 4
 s1 sl_insert_new: going to insert key 607 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 607 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 2660 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
s3 find_index_node: 0x1b1e260 with min 415 max 639 sum 2660
 s3 sl_insert_new: going to insert key 607 index node exists 0x1b1e260 max: 639 min 415 sum 2660
0x1b1eb70 set key 607 at 5
 s1 sl_insert_new: going to insert key 872 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 872 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3267 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 3447 next is 0
 s4 find_index_node: key 883
item->max: 883 item->min 811
s3 find_index_node: 0x1b1ea30 with min 811 max 883 sum 3447
 s3 sl_insert_new: going to insert key 872 index node exists 0x1b1ea30 max: 883 min 811 sum 3447
0x1b1e980 set key 872 at 4
 s1 sl_insert_new: going to insert key 312 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 312 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3267 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3267 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1ecd0 next is 0x1b1ecd0 level is 0
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 102 sum is 1222 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 102
s3 find_index_node: 0x1b1ecd0 with min 102 max 344 sum 1222
 s3 sl_insert_new: going to insert key 312 index node exists 0x1b1ecd0 max: 344 min 102 sum 1222
0x1b1ec20 set key 312 at 5
 s1 sl_insert_new: going to insert key 628 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 628 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3267 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
s3 find_index_node: 0x1b1e260 with min 415 max 639 sum 3267
 s3 sl_insert_new: going to insert key 628 index node exists 0x1b1e260 max: 639 min 415 sum 3267
0x1b1eb70 set key 628 at 6
 s1 sl_insert_new: going to insert key 648 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 648 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 3
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b1d810
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 891 sum is 4567 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 891
 s3 find_index_node: foud pred 0x1b1e7a0 next 0x1b1dc80 at level 0
 find_index_node:  foud proper place for key 648 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 648 index node exists 0x1b1d810 max: 922 min 891 sum 4567
0x1b1e8d0 set key 648 at 5
 s1 sl_insert_new: going to insert key 928 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 928 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 4319 next is 0
 s4 find_index_node: key 883
item->max: 883 item->min 811
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 648 sum is 5215 next is 0x1b1dc80
 s4 find_index_node: key 922
item->max: 922 item->min 648
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 972 sum is 972 next is 0
 s4 find_index_node: key 972
item->max: 972 item->min 972
 s3 find_index_node: foud pred 0x1b1d810 next 0 at level 0
 find_index_node:  foud proper place for key 928 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 928 index node exists 0x1b1dc80 max: 972 min 972 sum 972
 nvnode 0x1b1ddb0 is full, need to be split 
 sum of index_node: 0x1b1dc80 is 972
target key 121 temp_key 972
0x1b1ee00 set key 972 at 0
target key 121 temp_key 874
0x1b1ee00 set key 874 at 1
target key 121 temp_key 905
0x1b1ee00 set key 905 at 2
target key 121 temp_key 891
0x1b1ee00 set key 891 at 3
target key 121 temp_key 639
0x1b1ee00 set key 639 at 4
target key 121 temp_key 126
0x1b1ee00 set key 126 at 5
target key 121 temp_key 212
0x1b1ee00 set key 212 at 6
target key 121 temp_key 344
0x1b1ee00 set key 344 at 7
0x1b1ee00 set key 928 at -1
 orig_nvnode: 0x1b1ee00 with slot 9 new_leaf 0x1b1eeb0 with slot 0
 sl_insert_new: attempting to insert an new index node between 0x1b1d810 and 0x1b1dc80 with new_sum 0
s2 node_alloc : new node 0x1b1ef60 1 levels
0x1b1ef60 sum is 0 0x1b1dc80 sum is 5891
 pred of new_index is 0x1b1d810 sl->head is 0x1b1cec0
preds level 0 is 0x1b1d810 next is 0x1b1ef60
 s1 sl_insert_new: going to insert key 55 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 55 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1ecd0 next is 0x1b1ecd0 level is 0
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 102 sum is 1534 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 102
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e260 at level 0
 find_index_node:  foud proper place for key 55 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 55 index node exists 0x1b1ecd0 max: 344 min 102 sum 1534
0x1b1ec20 set key 55 at 6
 s1 sl_insert_new: going to insert key 178 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 178 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1ecd0 next is 0x1b1ecd0 level is 0
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 55 sum is 1589 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 55
s3 find_index_node: 0x1b1ecd0 with min 55 max 344 sum 1589
 s3 sl_insert_new: going to insert key 178 index node exists 0x1b1ecd0 max: 344 min 55 sum 1589
0x1b1ec20 set key 178 at 7
 s1 sl_insert_new: going to insert key 989 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 989 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 3
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 4319 next is 0
 s4 find_index_node: key 883
item->max: 883 item->min 811
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 648 sum is 5215 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 648
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 5891 next is 0
 s4 find_index_node: key 972
item->max: 972 item->min 126
 s3 find_index_node: foud pred 0x1b1dc80 next 0 at level 0
 find_index_node:  foud proper place for key 989 in skiplist. pred is  returning null 
 s3 sl_insert_new with preds[0]: attempting to insert an new index node between 0x1b1dc80 and 0 0?
s2 node_alloc : new node 0x1b1f090 3 levels
0x1b1f1d0 set key 989 at 0
 s1 sl_insert_new: going to insert key 66 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 66 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1ecd0 next is 0x1b1ecd0 level is 0
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 55 sum is 1767 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 55
s3 find_index_node: 0x1b1ecd0 with min 55 max 344 sum 1767
 s3 sl_insert_new: going to insert key 66 index node exists 0x1b1ecd0 max: 344 min 55 sum 1767
 nvnode 0x1b1ec20 is full, need to be split 
 sum of index_node: 0x1b1ecd0 is 1767
target key 220 temp_key 344
0x1b1f280 set key 344 at 0
target key 220 temp_key 264
0x1b1f280 set key 264 at 1
target key 220 temp_key 102
0x1b1f330 set key 102 at 0
target key 220 temp_key 249
0x1b1f280 set key 249 at 2
target key 220 temp_key 263
0x1b1f280 set key 263 at 3
target key 220 temp_key 312
0x1b1f280 set key 312 at 4
target key 220 temp_key 55
0x1b1f330 set key 55 at 1
target key 220 temp_key 178
0x1b1f330 set key 178 at 2
0x1b1f330 set key 66 at 3
 orig_nvnode: 0x1b1f280 with slot 5 new_leaf 0x1b1f330 with slot 4
s2 node_alloc : new node 0x1b1f3e0 1 levels
 s1 sl_insert_new: going to insert key 678 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 678 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 415
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b1d810
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 648 sum is 5215 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 648
s3 find_index_node: 0x1b1d810 with min 648 max 922 sum 5215
 s3 sl_insert_new: going to insert key 678 index node exists 0x1b1d810 max: 922 min 648 sum 5215
0x1b1e8d0 set key 678 at 6
 s1 sl_insert_new: going to insert key 526 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 526 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 3895 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 415
s3 find_index_node: 0x1b1e260 with min 415 max 639 sum 3895
 s3 sl_insert_new: going to insert key 526 index node exists 0x1b1e260 max: 639 min 415 sum 3895
0x1b1eb70 set key 526 at 7
 s1 sl_insert_new: going to insert key 519 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 519 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 415 sum is 4421 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 415
s3 find_index_node: 0x1b1e260 with min 415 max 639 sum 4421
 s3 sl_insert_new: going to insert key 519 index node exists 0x1b1e260 max: 639 min 415 sum 4421
 nvnode 0x1b1eb70 is full, need to be split 
 sum of index_node: 0x1b1e260 is 4421
target key 552 temp_key 639
0x1b1f510 set key 639 at 0
target key 552 temp_key 579
0x1b1f510 set key 579 at 1
target key 552 temp_key 564
0x1b1f510 set key 564 at 2
target key 552 temp_key 463
0x1b1f5c0 set key 463 at 0
target key 552 temp_key 415
0x1b1f5c0 set key 415 at 1
target key 552 temp_key 607
0x1b1f510 set key 607 at 3
target key 552 temp_key 628
0x1b1f510 set key 628 at 4
target key 552 temp_key 526
0x1b1f5c0 set key 526 at 2
0x1b1f5c0 set key 519 at 3
 orig_nvnode: 0x1b1f510 with slot 5 new_leaf 0x1b1f5c0 with slot 4
 sl_insert_new: attempting to insert an new index node between 0x1b1dfc0 and 0x1b1ecd0 with new_sum 1923
s2 node_alloc : new node 0x1b1f670 1 levels
0x1b1f670 sum is 1923 0x1b1e260 sum is 3017
 pred of new_index is 0x1b1dfc0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1dfc0 next is 0x1b1f670
 s1 sl_insert_new: going to insert key 81 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 81 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 415 sum is 1923 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 415
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1ecd0 at level 0
 find_index_node:  foud proper place for key 81 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 81 index node exists 0x1b1f670 max: 526 min 415 sum 1923
0x1b1f5c0 set key 81 at 4
 s1 sl_insert_new: going to insert key 918 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 918 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 4319 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 811
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 648 sum is 5893 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 648
s3 find_index_node: 0x1b1d810 with min 648 max 922 sum 5893
 s3 sl_insert_new: going to insert key 918 index node exists 0x1b1d810 max: 922 min 648 sum 5893
0x1b1e8d0 set key 918 at 7
 s1 sl_insert_new: going to insert key 367 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 367 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 81 sum is 2004 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 81
s3 find_index_node: 0x1b1f670 with min 81 max 526 sum 2004
 s3 sl_insert_new: going to insert key 367 index node exists 0x1b1f670 max: 526 min 81 sum 2004
0x1b1f5c0 set key 367 at 5
 s1 sl_insert_new: going to insert key 261 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 261 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 81 sum is 2371 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 81
s3 find_index_node: 0x1b1f670 with min 81 max 526 sum 2371
 s3 sl_insert_new: going to insert key 261 index node exists 0x1b1f670 max: 526 min 81 sum 2371
0x1b1f5c0 set key 261 at 6
 s1 sl_insert_new: going to insert key 147 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 147 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 81 sum is 2632 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 81
s3 find_index_node: 0x1b1f670 with min 81 max 526 sum 2632
 s3 sl_insert_new: going to insert key 147 index node exists 0x1b1f670 max: 526 min 81 sum 2632
0x1b1f5c0 set key 147 at 7
 s1 sl_insert_new: going to insert key 17 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 17 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1e260
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 81 sum is 2779 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 81
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1ecd0 at level 0
 find_index_node:  foud proper place for key 17 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 17 index node exists 0x1b1f670 max: 526 min 81 sum 2779
 nvnode 0x1b1f5c0 is full, need to be split 
 sum of index_node: 0x1b1f670 is 2779
target key 347 temp_key 463
0x1b1f7a0 set key 463 at 0
target key 347 temp_key 415
0x1b1f7a0 set key 415 at 1
target key 347 temp_key 526
0x1b1f7a0 set key 526 at 2
target key 347 temp_key 519
0x1b1f7a0 set key 519 at 3
target key 347 temp_key 81
0x1b1f850 set key 81 at 0
target key 347 temp_key 367
0x1b1f7a0 set key 367 at 4
target key 347 temp_key 261
0x1b1f850 set key 261 at 1
target key 347 temp_key 147
0x1b1f850 set key 147 at 2
0x1b1f850 set key 17 at 3
 orig_nvnode: 0x1b1f7a0 with slot 5 new_leaf 0x1b1f850 with slot 4
 sl_insert_new: attempting to insert an new index node between 0x1b1dfc0 and 0x1b1f670 with new_sum 506
s2 node_alloc : new node 0x1b1f900 2 levels
0x1b1f900 sum is 506 0x1b1f670 sum is 2290
 pred of new_index is 0x1b1dfc0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1dfc0 next is 0x1b1f900
preds level 1 is 0x1b1dfc0 next is 0x1b1f900
 s1 sl_insert_new: going to insert key 312 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 312 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 506 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1f900 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1f900
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 367 sum is 2290 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 367
 s3 find_index_node: foud pred 0x1b1f900 next 0x1b1ecd0 at level 0
 find_index_node:  foud proper place for key 312 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 312 index node exists 0x1b1f670 max: 526 min 367 sum 2290
0x1b1f7a0 set key 312 at 5
 s1 sl_insert_new: going to insert key 728 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 728 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2174 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
s3 find_index_node: 0x1b1e500 with min 708 max 753 sum 2174
 s3 sl_insert_new: going to insert key 728 index node exists 0x1b1e500 max: 753 min 708 sum 2174
0x1b1e450 set key 728 at 3
 s1 sl_insert_new: going to insert key 39 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 39 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 506 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
s3 find_index_node: 0x1b1f900 with min 17 max 261 sum 506
 s3 sl_insert_new: going to insert key 39 index node exists 0x1b1f900 max: 261 min 17 sum 506
0x1b1f850 set key 39 at 4
 s1 sl_insert_new: going to insert key 540 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 540 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 545 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1f900 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1f900
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 312 sum is 2602 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 312
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 249 sum is 1432 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 249
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 564 sum is 3017 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 564
 s3 find_index_node: foud pred 0x1b1ecd0 next 0x1b1e7a0 at level 0
 find_index_node:  foud proper place for key 540 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 540 index node exists 0x1b1e260 max: 639 min 564 sum 3017
0x1b1f510 set key 540 at 5
 s1 sl_insert_new: going to insert key 836 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 836 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 3557 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 4319 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 811
s3 find_index_node: 0x1b1ea30 with min 811 max 883 sum 4319
 s3 sl_insert_new: going to insert key 836 index node exists 0x1b1ea30 max: 883 min 811 sum 4319
0x1b1e980 set key 836 at 5
 s1 sl_insert_new: going to insert key 562 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 562 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 3557 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
s3 find_index_node: 0x1b1e260 with min 540 max 639 sum 3557
 s3 sl_insert_new: going to insert key 562 index node exists 0x1b1e260 max: 639 min 540 sum 3557
0x1b1f510 set key 562 at 6
 s1 sl_insert_new: going to insert key 305 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 305 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 545 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1f900 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1f900
item is 0x1b1f670 next is 0x1b1f670 level is 0
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 312 sum is 2602 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 312
 s3 find_index_node: foud pred 0x1b1f900 next 0x1b1ecd0 at level 0
 find_index_node:  foud proper place for key 305 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 305 index node exists 0x1b1f670 max: 526 min 312 sum 2602
0x1b1f7a0 set key 305 at 6
 s1 sl_insert_new: going to insert key 693 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 693 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b1d810
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 648 sum is 6811 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 648
s3 find_index_node: 0x1b1d810 with min 648 max 922 sum 6811
 s3 sl_insert_new: going to insert key 693 index node exists 0x1b1d810 max: 922 min 648 sum 6811
 nvnode 0x1b1e8d0 is full, need to be split 
 sum of index_node: 0x1b1d810 is 6811
target key 851 temp_key 972
0x1b1fa40 set key 972 at 0
target key 851 temp_key 905
0x1b1fa40 set key 905 at 1
target key 851 temp_key 891
0x1b1fa40 set key 891 at 2
target key 851 temp_key 893
0x1b1fa40 set key 893 at 3
target key 851 temp_key 906
0x1b1fa40 set key 906 at 4
target key 851 temp_key 648
0x1b1faf0 set key 648 at 0
target key 851 temp_key 678
0x1b1faf0 set key 678 at 1
target key 851 temp_key 918
0x1b1fa40 set key 918 at 5
0x1b1faf0 set key 693 at 2
 orig_nvnode: 0x1b1fa40 with slot 6 new_leaf 0x1b1faf0 with slot 3
 sl_insert_new: attempting to insert an new index node between 0x1b1f900 and 0x1b1f670 with new_sum 2019
s2 node_alloc : new node 0x1b1fba0 1 levels
0x1b1fba0 sum is 2019 0x1b1d810 sum is 5485
 pred of new_index is 0x1b1f900 sl->head is 0x1b1cec0
preds level 0 is 0x1b1f900 next is 0x1b1fba0
 s1 sl_insert_new: going to insert key 909 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 909 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 5155 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 811
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 891 sum is 5485 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 891
s3 find_index_node: 0x1b1d810 with min 891 max 922 sum 5485
 s3 sl_insert_new: going to insert key 909 index node exists 0x1b1d810 max: 922 min 891 sum 5485
0x1b1fa40 set key 909 at 6
 s1 sl_insert_new: going to insert key 169 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 169 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 3
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 545 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
s3 find_index_node: 0x1b1f900 with min 17 max 261 sum 545
 s3 sl_insert_new: going to insert key 169 index node exists 0x1b1f900 max: 261 min 17 sum 545
0x1b1f850 set key 169 at 5
 s1 sl_insert_new: going to insert key 303 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 303 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 714 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1f900 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1f900
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s3 find_index_node: foud pred 0x1b1f900 next 0x1b1f670 at level 0
 find_index_node:  foud proper place for key 303 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 303 index node exists 0x1b1fba0 max: 693 min 648 sum 2019
0x1b1faf0 set key 303 at 3
 s1 sl_insert_new: going to insert key 45 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 45 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 714 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
s3 find_index_node: 0x1b1f900 with min 17 max 261 sum 714
 s3 sl_insert_new: going to insert key 45 index node exists 0x1b1f900 max: 261 min 17 sum 714
0x1b1f850 set key 45 at 6
 s1 sl_insert_new: going to insert key 75 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 75 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 759 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
s3 find_index_node: 0x1b1f900 with min 17 max 261 sum 759
 s3 sl_insert_new: going to insert key 75 index node exists 0x1b1f900 max: 261 min 17 sum 759
0x1b1f850 set key 75 at 7
 s1 sl_insert_new: going to insert key 81 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 81 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 17 sum is 834 next is 0x1b1e260
 s4 find_index_node: key 261
item->max: 261 item->min 17
s3 find_index_node: 0x1b1f900 with min 17 max 261 sum 834
 s3 sl_insert_new: going to insert key 81 index node exists 0x1b1f900 max: 261 min 17 sum 834
 nvnode 0x1b1f850 is full, need to be split 
 sum of index_node: 0x1b1f900 is 834
target key 104 temp_key 81
0x1b1fd80 set key 81 at 0
target key 104 temp_key 261
0x1b1fcd0 set key 261 at 0
target key 104 temp_key 147
0x1b1fcd0 set key 147 at 1
target key 104 temp_key 17
0x1b1fd80 set key 17 at 1
target key 104 temp_key 39
0x1b1fd80 set key 39 at 2
target key 104 temp_key 169
0x1b1fcd0 set key 169 at 2
target key 104 temp_key 45
0x1b1fd80 set key 45 at 3
target key 104 temp_key 75
0x1b1fd80 set key 75 at 4
0x1b1fd80 set key 81 at 5
 orig_nvnode: 0x1b1fcd0 with slot 3 new_leaf 0x1b1fd80 with slot 6
 sl_insert_new: attempting to insert an new index node between 0x1b1f900 and 0x1b1fba0 with new_sum 338
s2 node_alloc : new node 0x1b1fe30 2 levels
0x1b1fe30 sum is 338 0x1b1f900 sum is 577
 pred of new_index is 0x1b1f900 sl->head is 0x1b1cec0
preds level 0 is 0x1b1f900 next is 0x1b1fe30
preds level 1 is 0x1b1f900 next is 0x1b1fe30
 s1 sl_insert_new: going to insert key 792 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 792 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 3
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 5155 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 811
 s3 find_index_node: foud pred 0x1b1e500 next 0x1b1f090 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e500
item is 0x1b1ea30 next is 0x1b1ea30 level is 0
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 811 sum is 5155 next is 0x1b1d810
 s4 find_index_node: key 883
item->max: 883 item->min 811
 s3 find_index_node: foud pred 0x1b1e500 next 0x1b1d810 at level 0
 find_index_node:  foud proper place for key 792 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 792 index node exists 0x1b1ea30 max: 883 min 811 sum 5155
0x1b1e980 set key 792 at 6
 s1 sl_insert_new: going to insert key 46 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 46 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 577 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1f900 next is 0x1b1f900 level is 0
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 577 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 0
 find_index_node:  foud proper place for key 46 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 46 index node exists 0x1b1f900 max: 261 min 147 sum 577
0x1b1fcd0 set key 46 at 3
 s1 sl_insert_new: going to insert key 347 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 347 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 623 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b1e260
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 303 sum is 2322 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 303
s3 find_index_node: 0x1b1fba0 with min 303 max 693 sum 2322
 s3 sl_insert_new: going to insert key 347 index node exists 0x1b1fba0 max: 693 min 303 sum 2322
0x1b1faf0 set key 347 at 4
 s1 sl_insert_new: going to insert key 424 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 424 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 623 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b1e260
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 303 sum is 2669 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 303
s3 find_index_node: 0x1b1fba0 with min 303 max 693 sum 2669
 s3 sl_insert_new: going to insert key 424 index node exists 0x1b1fba0 max: 693 min 303 sum 2669
0x1b1faf0 set key 424 at 5
 s1 sl_insert_new: going to insert key 492 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 492 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 623 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b1e260
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 303 sum is 3093 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 303
s3 find_index_node: 0x1b1fba0 with min 303 max 693 sum 3093
 s3 sl_insert_new: going to insert key 492 index node exists 0x1b1fba0 max: 693 min 303 sum 3093
0x1b1faf0 set key 492 at 6
 s1 sl_insert_new: going to insert key 432 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 432 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 623 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b1e260
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 303 sum is 3585 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 303
s3 find_index_node: 0x1b1fba0 with min 303 max 693 sum 3585
 s3 sl_insert_new: going to insert key 432 index node exists 0x1b1fba0 max: 693 min 303 sum 3585
0x1b1faf0 set key 432 at 7
 s1 sl_insert_new: going to insert key 651 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 651 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b1d810
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 891 sum is 6394 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 891
 s3 find_index_node: foud pred 0x1b1e7a0 next 0x1b1ef60 at level 0
 find_index_node:  foud proper place for key 651 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 651 index node exists 0x1b1d810 max: 922 min 891 sum 6394
0x1b1fa40 set key 651 at 7
 s1 sl_insert_new: going to insert key 298 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 298 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 623 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b1e260
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 303 sum is 4017 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 303
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1f670 at level 0
 find_index_node:  foud proper place for key 298 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 298 index node exists 0x1b1fba0 max: 693 min 303 sum 4017
 nvnode 0x1b1faf0 is full, need to be split 
 sum of index_node: 0x1b1fba0 is 4017
target key 502 temp_key 648
0x1b1ff70 set key 648 at 0
target key 502 temp_key 678
0x1b1ff70 set key 678 at 1
target key 502 temp_key 693
0x1b1ff70 set key 693 at 2
target key 502 temp_key 303
0x1b20020 set key 303 at 0
target key 502 temp_key 347
0x1b20020 set key 347 at 1
target key 502 temp_key 424
0x1b20020 set key 424 at 2
target key 502 temp_key 492
0x1b20020 set key 492 at 3
target key 502 temp_key 432
0x1b20020 set key 432 at 4
0x1b20020 set key 298 at 5
 orig_nvnode: 0x1b1ff70 with slot 3 new_leaf 0x1b20020 with slot 6
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b1fba0 with new_sum 2296
s2 node_alloc : new node 0x1b200d0 2 levels
0x1b200d0 sum is 2296 0x1b1fba0 sum is 2019
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b200d0
preds level 1 is 0x1b1fe30 next is 0x1b200d0
 s1 sl_insert_new: going to insert key 78 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 78 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 623 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
s3 find_index_node: 0x1b1f900 with min 46 max 261 sum 623
 s3 sl_insert_new: going to insert key 78 index node exists 0x1b1f900 max: 261 min 46 sum 623
0x1b1fcd0 set key 78 at 4
 s1 sl_insert_new: going to insert key 938 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 938 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 792 sum is 5947 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 792
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 651 sum is 7045 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 651
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 5891 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
s3 find_index_node: 0x1b1dc80 with min 126 max 972 sum 5891
 s3 sl_insert_new: going to insert key 938 index node exists 0x1b1dc80 max: 972 min 126 sum 5891
0x1b1f1d0 set key 938 at 1
 s1 sl_insert_new: going to insert key 951 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 951 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 792 sum is 5947 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 792
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 651 sum is 7045 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 651
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 6829 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
s3 find_index_node: 0x1b1dc80 with min 126 max 972 sum 6829
 s3 sl_insert_new: going to insert key 951 index node exists 0x1b1dc80 max: 972 min 126 sum 6829
0x1b1f1d0 set key 951 at 2
 s1 sl_insert_new: going to insert key 831 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 831 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 792 sum is 5947 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 792
s3 find_index_node: 0x1b1ea30 with min 792 max 883 sum 5947
 s3 sl_insert_new: going to insert key 831 index node exists 0x1b1ea30 max: 883 min 792 sum 5947
0x1b1e980 set key 831 at 7
 s1 sl_insert_new: going to insert key 21 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 21 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 701 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b1f900 next is 0x1b1f900 level is 0
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 46 sum is 701 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 46
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 0
 find_index_node:  foud proper place for key 21 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 21 index node exists 0x1b1f900 max: 261 min 46 sum 701
0x1b1fcd0 set key 21 at 5
 s1 sl_insert_new: going to insert key 675 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 675 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b1d810
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 651 sum is 7045 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 651
s3 find_index_node: 0x1b1d810 with min 651 max 922 sum 7045
 s3 sl_insert_new: going to insert key 675 index node exists 0x1b1d810 max: 922 min 651 sum 7045
 nvnode 0x1b1fa40 is full, need to be split 
 sum of index_node: 0x1b1d810 is 7045
target key 880 temp_key 972
0x1b20210 set key 972 at 0
target key 880 temp_key 905
0x1b20210 set key 905 at 1
target key 880 temp_key 891
0x1b20210 set key 891 at 2
target key 880 temp_key 893
0x1b20210 set key 893 at 3
target key 880 temp_key 906
0x1b20210 set key 906 at 4
target key 880 temp_key 918
0x1b20210 set key 918 at 5
target key 880 temp_key 909
0x1b20210 set key 909 at 6
target key 880 temp_key 651
0x1b202c0 set key 651 at 0
0x1b202c0 set key 675 at 1
 orig_nvnode: 0x1b20210 with slot 7 new_leaf 0x1b202c0 with slot 2
 sl_insert_new: attempting to insert an new index node between 0x1b1dfc0 and 0x1b1f900 with new_sum 1326
s2 node_alloc : new node 0x1b20370 1 levels
0x1b20370 sum is 1326 0x1b1d810 sum is 6394
 pred of new_index is 0x1b1dfc0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1dfc0 next is 0x1b20370
 s1 sl_insert_new: going to insert key 681 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 681 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b1d810
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 891 sum is 6394 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 891
 s3 find_index_node: foud pred 0x1b1e7a0 next 0x1b1ef60 at level 0
 find_index_node:  foud proper place for key 681 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 681 index node exists 0x1b1d810 max: 922 min 891 sum 6394
0x1b20210 set key 681 at 7
 s1 sl_insert_new: going to insert key 237 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 237 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 722 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
s3 find_index_node: 0x1b1f900 with min 21 max 261 sum 722
 s3 sl_insert_new: going to insert key 237 index node exists 0x1b1f900 max: 261 min 21 sum 722
0x1b1fcd0 set key 237 at 6
 s1 sl_insert_new: going to insert key 848 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 848 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 792 sum is 6778 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 792
s3 find_index_node: 0x1b1ea30 with min 792 max 883 sum 6778
 s3 sl_insert_new: going to insert key 848 index node exists 0x1b1ea30 max: 883 min 792 sum 6778
 nvnode 0x1b1e980 is full, need to be split 
 sum of index_node: 0x1b1ea30 is 6778
target key 847 temp_key 874
0x1b204a0 set key 874 at 0
target key 847 temp_key 879
0x1b204a0 set key 879 at 1
target key 847 temp_key 883
0x1b204a0 set key 883 at 2
target key 847 temp_key 811
0x1b20550 set key 811 at 0
target key 847 temp_key 872
0x1b204a0 set key 872 at 3
target key 847 temp_key 836
0x1b20550 set key 836 at 1
target key 847 temp_key 792
0x1b20550 set key 792 at 2
target key 847 temp_key 831
0x1b20550 set key 831 at 3
0x1b20550 set key 848 at 4
 orig_nvnode: 0x1b204a0 with slot 4 new_leaf 0x1b20550 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1e7a0 and 0x1b1d810 with new_sum 4118
s2 node_alloc : new node 0x1b20600 1 levels
0x1b20600 sum is 4118 0x1b1ea30 sum is 3508
 pred of new_index is 0x1b1e7a0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1e7a0 next is 0x1b20600
 s1 sl_insert_new: going to insert key 5 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 5 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b20370 next is 0x1b20370 level is 0
 s4 find_index_node: visiting item 0x1b20370 max 675 min 651 sum is 1326 next is 0x1b1f900
 s4 find_index_node: key 675
item->max: 675 item->min 651
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1f900 at level 0
 find_index_node:  foud proper place for key 5 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 5 index node exists 0x1b20370 max: 675 min 651 sum 1326
0x1b202c0 set key 5 at 2
 s1 sl_insert_new: going to insert key 497 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 497 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 298 sum is 2296 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 298
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b200d0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b200d0
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s3 find_index_node: foud pred 0x1b200d0 next 0x1b1f670 at level 0
 find_index_node:  foud proper place for key 497 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 497 index node exists 0x1b1fba0 max: 693 min 648 sum 2019
0x1b1ff70 set key 497 at 3
 s1 sl_insert_new: going to insert key 370 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 370 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 298 sum is 2296 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 298
s3 find_index_node: 0x1b200d0 with min 298 max 492 sum 2296
 s3 sl_insert_new: going to insert key 370 index node exists 0x1b200d0 max: 492 min 298 sum 2296
0x1b20020 set key 370 at 6
 s1 sl_insert_new: going to insert key 603 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 603 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4119 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
s3 find_index_node: 0x1b1e260 with min 540 max 639 sum 4119
 s3 sl_insert_new: going to insert key 603 index node exists 0x1b1e260 max: 639 min 540 sum 4119
0x1b1f510 set key 603 at 7
 s1 sl_insert_new: going to insert key 780 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 780 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 872 sum is 3508 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 872
 s3 find_index_node: foud pred 0x1b1e500 next 0x1b1f090 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e500
item is 0x1b1ea30 next is 0x1b1ea30 level is 0
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 872 sum is 3508 next is 0x1b1d810
 s4 find_index_node: key 883
item->max: 883 item->min 872
 s3 find_index_node: foud pred 0x1b1e500 next 0x1b1d810 at level 0
 find_index_node:  foud proper place for key 780 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 780 index node exists 0x1b1ea30 max: 883 min 872 sum 3508
0x1b204a0 set key 780 at 4
 s1 sl_insert_new: going to insert key 643 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 643 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 792 sum is 4118 next is 0x1b1d810
 s4 find_index_node: key 848
item->max: 848 item->min 792
 s3 find_index_node: foud pred 0x1b1e7a0 next 0x1b1d810 at level 0
 find_index_node:  foud proper place for key 643 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 643 index node exists 0x1b20600 max: 848 min 792 sum 4118
0x1b20550 set key 643 at 5
 s1 sl_insert_new: going to insert key 879 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 879 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 780 sum is 4288 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 780
s3 find_index_node: 0x1b1ea30 with min 780 max 883 sum 4288
 s3 sl_insert_new: going to insert key 879 index node exists 0x1b1ea30 max: 883 min 780 sum 4288
0x1b204a0 set key 879 at 5
 s1 sl_insert_new: going to insert key 709 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 709 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 2902 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
s3 find_index_node: 0x1b1e500 with min 708 max 753 sum 2902
 s3 sl_insert_new: going to insert key 709 index node exists 0x1b1e500 max: 753 min 708 sum 2902
0x1b1e450 set key 709 at 4
 s1 sl_insert_new: going to insert key 339 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 339 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 298 sum is 2666 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 298
s3 find_index_node: 0x1b200d0 with min 298 max 492 sum 2666
 s3 sl_insert_new: going to insert key 339 index node exists 0x1b200d0 max: 492 min 298 sum 2666
0x1b20020 set key 339 at 7
 s1 sl_insert_new: going to insert key 273 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 273 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 298 sum is 3005 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 298
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e260 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b200d0 next is 0x1b200d0 level is 0
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 298 sum is 3005 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 298
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1fba0 at level 0
 find_index_node:  foud proper place for key 273 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 273 index node exists 0x1b200d0 max: 492 min 298 sum 3005
 nvnode 0x1b20020 is full, need to be split 
 sum of index_node: 0x1b200d0 is 3005
target key 375 temp_key 303
0x1b207e0 set key 303 at 0
target key 375 temp_key 347
0x1b207e0 set key 347 at 1
target key 375 temp_key 424
0x1b20730 set key 424 at 0
target key 375 temp_key 492
0x1b20730 set key 492 at 1
target key 375 temp_key 432
0x1b20730 set key 432 at 2
target key 375 temp_key 298
0x1b207e0 set key 298 at 2
target key 375 temp_key 370
0x1b207e0 set key 370 at 3
target key 375 temp_key 339
0x1b207e0 set key 339 at 4
0x1b207e0 set key 273 at 5
 orig_nvnode: 0x1b20730 with slot 3 new_leaf 0x1b207e0 with slot 6
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b200d0 with new_sum 1930
s2 node_alloc : new node 0x1b20890 1 levels
0x1b20890 sum is 1930 0x1b200d0 sum is 1348
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b20890
 s1 sl_insert_new: going to insert key 388 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 388 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 424 sum is 1348 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 424
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e260 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b20890 next is 0x1b20890 level is 0
 s4 find_index_node: visiting item 0x1b20890 max 370 min 273 sum is 1930 next is 0x1b200d0
 s4 find_index_node: key 370
item->max: 370 item->min 273
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 424 sum is 1348 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 424
 s3 find_index_node: foud pred 0x1b20890 next 0x1b1fba0 at level 0
 find_index_node:  foud proper place for key 388 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 388 index node exists 0x1b200d0 max: 492 min 424 sum 1348
0x1b20730 set key 388 at 3
 s1 sl_insert_new: going to insert key 485 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 485 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 1736 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
s3 find_index_node: 0x1b200d0 with min 388 max 492 sum 1736
 s3 sl_insert_new: going to insert key 485 index node exists 0x1b200d0 max: 492 min 388 sum 1736
0x1b20730 set key 485 at 4
 s1 sl_insert_new: going to insert key 334 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 334 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 2221 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e260 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b20890 next is 0x1b20890 level is 0
 s4 find_index_node: visiting item 0x1b20890 max 370 min 273 sum is 1930 next is 0x1b200d0
 s4 find_index_node: key 370
item->max: 370 item->min 273
s3 find_index_node: 0x1b20890 with min 273 max 370 sum 1930
 s3 sl_insert_new: going to insert key 334 index node exists 0x1b20890 max: 370 min 273 sum 1930
0x1b207e0 set key 334 at 6
 s1 sl_insert_new: going to insert key 467 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 467 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 2221 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
s3 find_index_node: 0x1b200d0 with min 388 max 492 sum 2221
 s3 sl_insert_new: going to insert key 467 index node exists 0x1b200d0 max: 492 min 388 sum 2221
0x1b20730 set key 467 at 5
 s1 sl_insert_new: going to insert key 965 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 965 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 780 sum is 5167 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 780
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 681 sum is 7075 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 681
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 7780 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
s3 find_index_node: 0x1b1dc80 with min 126 max 972 sum 7780
 s3 sl_insert_new: going to insert key 965 index node exists 0x1b1dc80 max: 972 min 126 sum 7780
0x1b1f1d0 set key 965 at 3
 s1 sl_insert_new: going to insert key 606 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 606 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 540 sum is 4722 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 540
s3 find_index_node: 0x1b1e260 with min 540 max 639 sum 4722
 s3 sl_insert_new: going to insert key 606 index node exists 0x1b1e260 max: 639 min 540 sum 4722
 nvnode 0x1b1f510 is full, need to be split 
 sum of index_node: 0x1b1e260 is 4722
target key 590 temp_key 639
0x1b209c0 set key 639 at 0
target key 590 temp_key 579
0x1b20a70 set key 579 at 0
target key 590 temp_key 564
0x1b20a70 set key 564 at 1
target key 590 temp_key 607
0x1b209c0 set key 607 at 1
target key 590 temp_key 628
0x1b209c0 set key 628 at 2
target key 590 temp_key 540
0x1b20a70 set key 540 at 2
target key 590 temp_key 562
0x1b20a70 set key 562 at 3
target key 590 temp_key 603
0x1b209c0 set key 603 at 3
0x1b209c0 set key 606 at 4
 orig_nvnode: 0x1b209c0 with slot 5 new_leaf 0x1b20a70 with slot 4
 sl_insert_new: attempting to insert an new index node between 0x1b20890 and 0x1b200d0 with new_sum 2245
s2 node_alloc : new node 0x1b20b20 1 levels
0x1b20b20 sum is 2245 0x1b1e260 sum is 3083
 pred of new_index is 0x1b20890 sl->head is 0x1b1cec0
preds level 0 is 0x1b20890 next is 0x1b20b20
 s1 sl_insert_new: going to insert key 622 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 622 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 3083 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
s3 find_index_node: 0x1b1e260 with min 603 max 639 sum 3083
 s3 sl_insert_new: going to insert key 622 index node exists 0x1b1e260 max: 639 min 603 sum 3083
0x1b209c0 set key 622 at 5
 s1 sl_insert_new: going to insert key 11 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 11 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 3705 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b20370 next is 0x1b20370 level is 0
 s4 find_index_node: visiting item 0x1b20370 max 675 min 5 sum is 1331 next is 0x1b1f900
 s4 find_index_node: key 675
item->max: 675 item->min 5
s3 find_index_node: 0x1b20370 with min 5 max 675 sum 1331
 s3 sl_insert_new: going to insert key 11 index node exists 0x1b20370 max: 675 min 5 sum 1331
0x1b202c0 set key 11 at 3
 s1 sl_insert_new: going to insert key 167 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 167 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 3705 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 959 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
s3 find_index_node: 0x1b1f900 with min 21 max 261 sum 959
 s3 sl_insert_new: going to insert key 167 index node exists 0x1b1f900 max: 261 min 21 sum 959
0x1b1fcd0 set key 167 at 7
 s1 sl_insert_new: going to insert key 612 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 612 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 3705 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
s3 find_index_node: 0x1b1e260 with min 603 max 639 sum 3705
 s3 sl_insert_new: going to insert key 612 index node exists 0x1b1e260 max: 639 min 603 sum 3705
0x1b209c0 set key 612 at 6
 s1 sl_insert_new: going to insert key 916 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 916 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 780 sum is 5167 next is 0x1b1f090
 s4 find_index_node: key 883
item->max: 883 item->min 780
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1ea30 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1ea30
item is 0x1b1d810 next is 0x1b1d810 level is 0
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 681 sum is 7075 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 681
s3 find_index_node: 0x1b1d810 with min 681 max 922 sum 7075
 s3 sl_insert_new: going to insert key 916 index node exists 0x1b1d810 max: 922 min 681 sum 7075
 nvnode 0x1b20210 is full, need to be split 
 sum of index_node: 0x1b1d810 is 7075
target key 884 temp_key 972
0x1b20c50 set key 972 at 0
target key 884 temp_key 905
0x1b20c50 set key 905 at 1
target key 884 temp_key 891
0x1b20c50 set key 891 at 2
target key 884 temp_key 893
0x1b20c50 set key 893 at 3
target key 884 temp_key 906
0x1b20c50 set key 906 at 4
target key 884 temp_key 918
0x1b20c50 set key 918 at 5
target key 884 temp_key 909
0x1b20c50 set key 909 at 6
target key 884 temp_key 681
0x1b20d00 set key 681 at 0
0x1b20c50 set key 916 at 7
 orig_nvnode: 0x1b20c50 with slot 8 new_leaf 0x1b20d00 with slot 1
 sl_insert_new: attempting to insert an new index node between 0x1b20890 and 0x1b200d0 with new_sum 681
s2 node_alloc : new node 0x1b20db0 2 levels
0x1b20db0 sum is 681 0x1b1d810 sum is 7310
 pred of new_index is 0x1b20890 sl->head is 0x1b1cec0
preds level 0 is 0x1b20890 next is 0x1b20db0
preds level 1 is 0x1b1ea30 next is 0x1b20db0
 s1 sl_insert_new: going to insert key 342 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 342 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 1126 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 2688 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e260 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b20890 next is 0x1b20890 level is 0
 s4 find_index_node: visiting item 0x1b20890 max 370 min 273 sum is 2264 next is 0x1b20db0
 s4 find_index_node: key 370
item->max: 370 item->min 273
s3 find_index_node: 0x1b20890 with min 273 max 370 sum 2264
 s3 sl_insert_new: going to insert key 342 index node exists 0x1b20890 max: 370 min 273 sum 2264
0x1b207e0 set key 342 at 7
 s1 sl_insert_new: going to insert key 819 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 819 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 780 sum is 5167 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 780
s3 find_index_node: 0x1b1ea30 with min 780 max 883 sum 5167
 s3 sl_insert_new: going to insert key 819 index node exists 0x1b1ea30 max: 883 min 780 sum 5167
0x1b204a0 set key 819 at 6
 s1 sl_insert_new: going to insert key 340 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 340 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 1126 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 2688 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e260 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b20890 next is 0x1b20890 level is 0
 s4 find_index_node: visiting item 0x1b20890 max 370 min 273 sum is 2606 next is 0x1b20db0
 s4 find_index_node: key 370
item->max: 370 item->min 273
s3 find_index_node: 0x1b20890 with min 273 max 370 sum 2606
 s3 sl_insert_new: going to insert key 340 index node exists 0x1b20890 max: 370 min 273 sum 2606
 nvnode 0x1b207e0 is full, need to be split 
 sum of index_node: 0x1b20890 is 2606
target key 325 temp_key 303
0x1b20fa0 set key 303 at 0
target key 325 temp_key 347
0x1b20ef0 set key 347 at 0
target key 325 temp_key 298
0x1b20fa0 set key 298 at 1
target key 325 temp_key 370
0x1b20ef0 set key 370 at 1
target key 325 temp_key 339
0x1b20ef0 set key 339 at 2
target key 325 temp_key 273
0x1b20fa0 set key 273 at 2
target key 325 temp_key 334
0x1b20ef0 set key 334 at 3
target key 325 temp_key 342
0x1b20ef0 set key 342 at 4
0x1b20ef0 set key 340 at 5
 orig_nvnode: 0x1b20ef0 with slot 6 new_leaf 0x1b20fa0 with slot 3
 sl_insert_new: attempting to insert an new index node between 0x1b20890 and 0x1b200d0 with new_sum 874
s2 node_alloc : new node 0x1b21050 1 levels
0x1b21050 sum is 874 0x1b20890 sum is 2072
 pred of new_index is 0x1b20890 sl->head is 0x1b1cec0
preds level 0 is 0x1b20890 next is 0x1b21050
 s1 sl_insert_new: going to insert key 678 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 678 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 643 sum is 4761 next is 0x1b1d810
 s4 find_index_node: key 848
item->max: 848 item->min 643
s3 find_index_node: 0x1b20600 with min 643 max 848 sum 4761
 s3 sl_insert_new: going to insert key 678 index node exists 0x1b20600 max: 848 min 643 sum 4761
0x1b20550 set key 678 at 6
 s1 sl_insert_new: going to insert key 578 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 578 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 1126 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 2688 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b200d0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b200d0
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 497 sum is 2516 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 497
s3 find_index_node: 0x1b1fba0 with min 497 max 693 sum 2516
 s3 sl_insert_new: going to insert key 578 index node exists 0x1b1fba0 max: 693 min 497 sum 2516
0x1b1ff70 set key 578 at 4
 s1 sl_insert_new: going to insert key 167 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 167 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 21 sum is 1126 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 21
s3 find_index_node: 0x1b1f900 with min 21 max 261 sum 1126
 s3 sl_insert_new: going to insert key 167 index node exists 0x1b1f900 max: 261 min 21 sum 1126
 nvnode 0x1b1fcd0 is full, need to be split 
 sum of index_node: 0x1b1f900 is 1126
target key 140 temp_key 261
0x1b21180 set key 261 at 0
target key 140 temp_key 147
0x1b21180 set key 147 at 1
target key 140 temp_key 169
0x1b21180 set key 169 at 2
target key 140 temp_key 46
0x1b21230 set key 46 at 0
target key 140 temp_key 78
0x1b21230 set key 78 at 1
target key 140 temp_key 21
0x1b21230 set key 21 at 2
target key 140 temp_key 237
0x1b21180 set key 237 at 3
target key 140 temp_key 167
0x1b21180 set key 167 at 4
0x1b21180 set key 167 at 5
 orig_nvnode: 0x1b21180 with slot 6 new_leaf 0x1b21230 with slot 3
 sl_insert_new: attempting to insert an new index node between 0x1b20890 and 0x1b200d0 with new_sum 145
s2 node_alloc : new node 0x1b212e0 1 levels
0x1b212e0 sum is 145 0x1b1f900 sum is 1148
 pred of new_index is 0x1b20890 sl->head is 0x1b1cec0
preds level 0 is 0x1b20890 next is 0x1b212e0
 s1 sl_insert_new: going to insert key 436 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 436 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1148 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 2688 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
s3 find_index_node: 0x1b200d0 with min 388 max 492 sum 2688
 s3 sl_insert_new: going to insert key 436 index node exists 0x1b200d0 max: 492 min 388 sum 2688
0x1b20730 set key 436 at 6
 s1 sl_insert_new: going to insert key 472 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 4
 s2 find_index_node: searching for min key that >= key 472 in skiplist head is 0x1b1cec0 sl->high_water is 4 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1148 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3124 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
s3 find_index_node: 0x1b200d0 with min 388 max 492 sum 3124
 s3 sl_insert_new: going to insert key 472 index node exists 0x1b200d0 max: 492 min 388 sum 3124
0x1b20730 set key 472 at 7
 s1 sl_insert_new: going to insert key 237 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 237 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 4
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0 next is 0 level is 4
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1148 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
s3 find_index_node: 0x1b1f900 with min 147 max 261 sum 1148
 s3 sl_insert_new: going to insert key 237 index node exists 0x1b1f900 max: 261 min 147 sum 1148
0x1b21180 set key 237 at 6
 s1 sl_insert_new: going to insert key 691 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 691 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 643 sum is 5439 next is 0x1b1d810
 s4 find_index_node: key 848
item->max: 848 item->min 643
s3 find_index_node: 0x1b20600 with min 643 max 848 sum 5439
 s3 sl_insert_new: going to insert key 691 index node exists 0x1b20600 max: 848 min 643 sum 5439
0x1b20550 set key 691 at 7
 s1 sl_insert_new: going to insert key 280 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 280 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1385 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3596 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b1e260 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b20890 next is 0x1b20890 level is 0
 s4 find_index_node: visiting item 0x1b20890 max 370 min 334 sum is 2072 next is 0x1b212e0
 s4 find_index_node: key 370
item->max: 370 item->min 334
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b212e0 at level 0
 find_index_node:  foud proper place for key 280 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 280 index node exists 0x1b20890 max: 370 min 334 sum 2072
0x1b20ef0 set key 280 at 6
 s1 sl_insert_new: going to insert key 578 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 578 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1385 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3596 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b200d0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b200d0
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 497 sum is 3094 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 497
s3 find_index_node: 0x1b1fba0 with min 497 max 693 sum 3094
 s3 sl_insert_new: going to insert key 578 index node exists 0x1b1fba0 max: 693 min 497 sum 3094
0x1b1ff70 set key 578 at 5
 s1 sl_insert_new: going to insert key 588 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 588 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1385 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3596 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b200d0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b200d0
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 497 sum is 3672 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 497
s3 find_index_node: 0x1b1fba0 with min 497 max 693 sum 3672
 s3 sl_insert_new: going to insert key 588 index node exists 0x1b1fba0 max: 693 min 497 sum 3672
0x1b1ff70 set key 588 at 6
 s1 sl_insert_new: going to insert key 556 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 556 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1385 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3596 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b200d0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b200d0
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 497 sum is 4260 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 497
s3 find_index_node: 0x1b1fba0 with min 497 max 693 sum 4260
 s3 sl_insert_new: going to insert key 556 index node exists 0x1b1fba0 max: 693 min 497 sum 4260
0x1b1ff70 set key 556 at 7
 s1 sl_insert_new: going to insert key 579 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 579 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1385 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3596 next is 0x1b1e260
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b200d0 next 0x1b1e500 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b200d0
item is 0x1b1fba0 next is 0x1b1fba0 level is 0
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 497 sum is 4816 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 497
s3 find_index_node: 0x1b1fba0 with min 497 max 693 sum 4816
 s3 sl_insert_new: going to insert key 579 index node exists 0x1b1fba0 max: 693 min 497 sum 4816
 nvnode 0x1b1ff70 is full, need to be split 
 sum of index_node: 0x1b1fba0 is 4816
target key 602 temp_key 648
0x1b21410 set key 648 at 0
target key 602 temp_key 678
0x1b21410 set key 678 at 1
target key 602 temp_key 693
0x1b21410 set key 693 at 2
target key 602 temp_key 497
0x1b214c0 set key 497 at 0
target key 602 temp_key 578
0x1b214c0 set key 578 at 1
target key 602 temp_key 578
0x1b214c0 set key 578 at 2
target key 602 temp_key 588
0x1b214c0 set key 588 at 3
target key 602 temp_key 556
0x1b214c0 set key 556 at 4
0x1b214c0 set key 579 at 5
 orig_nvnode: 0x1b21410 with slot 3 new_leaf 0x1b214c0 with slot 6
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b20890 with new_sum 3376
s2 node_alloc : new node 0x1b21570 2 levels
0x1b21570 sum is 3376 0x1b1fba0 sum is 2019
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b21570
preds level 1 is 0x1b200d0 next is 0x1b21570
 s1 sl_insert_new: going to insert key 233 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 233 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1385 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
s3 find_index_node: 0x1b1f900 with min 147 max 261 sum 1385
 s3 sl_insert_new: going to insert key 233 index node exists 0x1b1f900 max: 261 min 147 sum 1385
0x1b21180 set key 233 at 7
 s1 sl_insert_new: going to insert key 6 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 6 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b20370 next is 0x1b20370 level is 0
 s4 find_index_node: visiting item 0x1b20370 max 675 min 5 sum is 1342 next is 0x1b1f900
 s4 find_index_node: key 675
item->max: 675 item->min 5
s3 find_index_node: 0x1b20370 with min 5 max 675 sum 1342
 s3 sl_insert_new: going to insert key 6 index node exists 0x1b20370 max: 675 min 5 sum 1342
0x1b202c0 set key 6 at 4
 s1 sl_insert_new: going to insert key 768 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 768 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 780 sum is 5986 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 780
 s3 find_index_node: foud pred 0x1b1e500 next 0x1b20db0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e500
item is 0x1b1ea30 next is 0x1b1ea30 level is 0
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 780 sum is 5986 next is 0x1b1d810
 s4 find_index_node: key 883
item->max: 883 item->min 780
 s3 find_index_node: foud pred 0x1b1e500 next 0x1b1d810 at level 0
 find_index_node:  foud proper place for key 768 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 768 index node exists 0x1b1ea30 max: 883 min 780 sum 5986
0x1b204a0 set key 768 at 7
 s1 sl_insert_new: going to insert key 378 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 378 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3596 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 388
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b21570 next is 0x1b21570 level is 0
 s4 find_index_node: visiting item 0x1b21570 max 588 min 497 sum is 3376 next is 0x1b20890
 s4 find_index_node: key 588
item->max: 588 item->min 497
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b20890 at level 0
 find_index_node:  foud proper place for key 378 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 378 index node exists 0x1b21570 max: 588 min 497 sum 3376
0x1b214c0 set key 378 at 6
 s1 sl_insert_new: going to insert key 457 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 457 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 388 sum is 3596 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 388
s3 find_index_node: 0x1b200d0 with min 388 max 492 sum 3596
 s3 sl_insert_new: going to insert key 457 index node exists 0x1b200d0 max: 492 min 388 sum 3596
 nvnode 0x1b20730 is full, need to be split 
 sum of index_node: 0x1b200d0 is 3596
target key 449 temp_key 424
0x1b21760 set key 424 at 0
target key 449 temp_key 492
0x1b216b0 set key 492 at 0
target key 449 temp_key 432
0x1b21760 set key 432 at 1
target key 449 temp_key 388
0x1b21760 set key 388 at 2
target key 449 temp_key 485
0x1b216b0 set key 485 at 1
target key 449 temp_key 467
0x1b216b0 set key 467 at 2
target key 449 temp_key 436
0x1b21760 set key 436 at 3
target key 449 temp_key 472
0x1b216b0 set key 472 at 3
0x1b21760 set key 457 at 4
 orig_nvnode: 0x1b216b0 with slot 4 new_leaf 0x1b21760 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b21570 with new_sum 2137
s2 node_alloc : new node 0x1b21810 1 levels
0x1b21810 sum is 2137 0x1b200d0 sum is 1916
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b21810
 s1 sl_insert_new: going to insert key 604 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 604 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4317 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
s3 find_index_node: 0x1b1e260 with min 603 max 639 sum 4317
 s3 sl_insert_new: going to insert key 604 index node exists 0x1b1e260 max: 639 min 603 sum 4317
0x1b209c0 set key 604 at 7
 s1 sl_insert_new: going to insert key 332 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 332 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b21810 next is 0x1b21810 level is 0
 s4 find_index_node: visiting item 0x1b21810 max 457 min 388 sum is 2137 next is 0x1b21570
 s4 find_index_node: key 457
item->max: 457 item->min 388
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 0
 find_index_node:  foud proper place for key 332 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 332 index node exists 0x1b21810 max: 457 min 388 sum 2137
0x1b21760 set key 332 at 5
 s1 sl_insert_new: going to insert key 127 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 127 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b20370 next is 0x1b20370 level is 0
 s4 find_index_node: visiting item 0x1b20370 max 675 min 5 sum is 1348 next is 0x1b1f900
 s4 find_index_node: key 675
item->max: 675 item->min 5
s3 find_index_node: 0x1b20370 with min 5 max 675 sum 1348
 s3 sl_insert_new: going to insert key 127 index node exists 0x1b20370 max: 675 min 5 sum 1348
0x1b202c0 set key 127 at 5
 s1 sl_insert_new: going to insert key 107 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 107 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b20370 next is 0x1b20370 level is 0
 s4 find_index_node: visiting item 0x1b20370 max 675 min 5 sum is 1475 next is 0x1b1f900
 s4 find_index_node: key 675
item->max: 675 item->min 5
s3 find_index_node: 0x1b20370 with min 5 max 675 sum 1475
 s3 sl_insert_new: going to insert key 107 index node exists 0x1b20370 max: 675 min 5 sum 1475
0x1b202c0 set key 107 at 6
 s1 sl_insert_new: going to insert key 408 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 408 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b21810 next is 0x1b21810 level is 0
 s4 find_index_node: visiting item 0x1b21810 max 457 min 332 sum is 2469 next is 0x1b21570
 s4 find_index_node: key 457
item->max: 457 item->min 332
s3 find_index_node: 0x1b21810 with min 332 max 457 sum 2469
 s3 sl_insert_new: going to insert key 408 index node exists 0x1b21810 max: 457 min 332 sum 2469
0x1b21760 set key 408 at 6
 s1 sl_insert_new: going to insert key 379 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 379 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b21810 next is 0x1b21810 level is 0
 s4 find_index_node: visiting item 0x1b21810 max 457 min 332 sum is 2877 next is 0x1b21570
 s4 find_index_node: key 457
item->max: 457 item->min 332
s3 find_index_node: 0x1b21810 with min 332 max 457 sum 2877
 s3 sl_insert_new: going to insert key 379 index node exists 0x1b21810 max: 457 min 332 sum 2877
0x1b21760 set key 379 at 7
 s1 sl_insert_new: going to insert key 962 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 962 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 768 sum is 6754 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 768
 s4 find_index_node: visiting item 0x1b20db0 max 681 min 681 sum is 681 next is 0x1b1f090
 s4 find_index_node: key 681
item->max: 681 item->min 681
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b20db0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b20db0
item is 0x1b200d0 next is 0x1b200d0 level is 0
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 305 sum is 2907 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 305
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 249 sum is 1432 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 249
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 643 sum is 6130 next is 0x1b1d810
 s4 find_index_node: key 848
item->max: 848 item->min 643
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 891 sum is 7310 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 891
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 8745 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
s3 find_index_node: 0x1b1dc80 with min 126 max 972 sum 8745
 s3 sl_insert_new: going to insert key 962 index node exists 0x1b1dc80 max: 972 min 126 sum 8745
0x1b1f1d0 set key 962 at 4
 s1 sl_insert_new: going to insert key 884 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 884 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 768 sum is 6754 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 768
 s4 find_index_node: visiting item 0x1b20db0 max 681 min 681 sum is 681 next is 0x1b1f090
 s4 find_index_node: key 681
item->max: 681 item->min 681
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b20db0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b20db0
item is 0x1b200d0 next is 0x1b200d0 level is 0
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 305 sum is 2907 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 305
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 249 sum is 1432 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 249
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 643 sum is 6130 next is 0x1b1d810
 s4 find_index_node: key 848
item->max: 848 item->min 643
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 891 sum is 7310 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 891
 s3 find_index_node: foud pred 0x1b20600 next 0x1b1ef60 at level 0
 find_index_node:  foud proper place for key 884 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 884 index node exists 0x1b1d810 max: 922 min 891 sum 7310
 nvnode 0x1b20c50 is full, need to be split 
 sum of index_node: 0x1b1d810 is 7310
target key 913 temp_key 972
0x1b21940 set key 972 at 0
target key 913 temp_key 905
0x1b219f0 set key 905 at 0
target key 913 temp_key 891
0x1b219f0 set key 891 at 1
target key 913 temp_key 893
0x1b219f0 set key 893 at 2
target key 913 temp_key 906
0x1b219f0 set key 906 at 3
target key 913 temp_key 918
0x1b21940 set key 918 at 1
target key 913 temp_key 909
0x1b219f0 set key 909 at 4
target key 913 temp_key 916
0x1b21940 set key 916 at 2
0x1b219f0 set key 884 at 5
 orig_nvnode: 0x1b21940 with slot 3 new_leaf 0x1b219f0 with slot 6
 sl_insert_new: attempting to insert an new index node between 0x1b20600 and 0x1b1d810 with new_sum 5388
s2 node_alloc : new node 0x1b21aa0 1 levels
0x1b21aa0 sum is 5388 0x1b1d810 sum is 2806
 pred of new_index is 0x1b20600 sl->head is 0x1b1cec0
preds level 0 is 0x1b20600 next is 0x1b21aa0
 s1 sl_insert_new: going to insert key 326 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 326 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b21810 next is 0x1b21810 level is 0
 s4 find_index_node: visiting item 0x1b21810 max 457 min 332 sum is 3256 next is 0x1b21570
 s4 find_index_node: key 457
item->max: 457 item->min 332
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 0
 find_index_node:  foud proper place for key 326 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 326 index node exists 0x1b21810 max: 457 min 332 sum 3256
 nvnode 0x1b21760 is full, need to be split 
 sum of index_node: 0x1b21810 is 3256
target key 407 temp_key 424
0x1b21bd0 set key 424 at 0
target key 407 temp_key 432
0x1b21bd0 set key 432 at 1
target key 407 temp_key 388
0x1b21c80 set key 388 at 0
target key 407 temp_key 436
0x1b21bd0 set key 436 at 2
target key 407 temp_key 457
0x1b21bd0 set key 457 at 3
target key 407 temp_key 332
0x1b21c80 set key 332 at 1
target key 407 temp_key 408
0x1b21bd0 set key 408 at 4
target key 407 temp_key 379
0x1b21c80 set key 379 at 2
0x1b21c80 set key 326 at 3
 orig_nvnode: 0x1b21bd0 with slot 5 new_leaf 0x1b21c80 with slot 4
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b21810 with new_sum 1425
s2 node_alloc : new node 0x1b21d30 1 levels
0x1b21d30 sum is 1425 0x1b21810 sum is 2157
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b21d30
 s1 sl_insert_new: going to insert key 367 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 367 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b21d30 next is 0x1b21d30 level is 0
 s4 find_index_node: visiting item 0x1b21d30 max 388 min 326 sum is 1425 next is 0x1b21810
 s4 find_index_node: key 388
item->max: 388 item->min 326
s3 find_index_node: 0x1b21d30 with min 326 max 388 sum 1425
 s3 sl_insert_new: going to insert key 367 index node exists 0x1b21d30 max: 388 min 326 sum 1425
0x1b21c80 set key 367 at 4
 s1 sl_insert_new: going to insert key 672 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 672 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 643 sum is 6130 next is 0x1b21aa0
 s4 find_index_node: key 848
item->max: 848 item->min 643
s3 find_index_node: 0x1b20600 with min 643 max 848 sum 6130
 s3 sl_insert_new: going to insert key 672 index node exists 0x1b20600 max: 848 min 643 sum 6130
 nvnode 0x1b20550 is full, need to be split 
 sum of index_node: 0x1b20600 is 6130
target key 766 temp_key 811
0x1b21e60 set key 811 at 0
target key 766 temp_key 836
0x1b21e60 set key 836 at 1
target key 766 temp_key 792
0x1b21e60 set key 792 at 2
target key 766 temp_key 831
0x1b21e60 set key 831 at 3
target key 766 temp_key 848
0x1b21e60 set key 848 at 4
target key 766 temp_key 643
0x1b21f10 set key 643 at 0
target key 766 temp_key 678
0x1b21f10 set key 678 at 1
target key 766 temp_key 691
0x1b21f10 set key 691 at 2
0x1b21f10 set key 672 at 3
 orig_nvnode: 0x1b21e60 with slot 5 new_leaf 0x1b21f10 with slot 4
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b21810 with new_sum 2684
s2 node_alloc : new node 0x1b21fc0 1 levels
0x1b21fc0 sum is 2684 0x1b20600 sum is 4118
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b21fc0
 s1 sl_insert_new: going to insert key 785 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 785 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 768 sum is 6754 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 768
s3 find_index_node: 0x1b1ea30 with min 768 max 883 sum 6754
 s3 sl_insert_new: going to insert key 785 index node exists 0x1b1ea30 max: 883 min 768 sum 6754
 nvnode 0x1b204a0 is full, need to be split 
 sum of index_node: 0x1b1ea30 is 6754
target key 844 temp_key 874
0x1b220f0 set key 874 at 0
target key 844 temp_key 879
0x1b220f0 set key 879 at 1
target key 844 temp_key 883
0x1b220f0 set key 883 at 2
target key 844 temp_key 872
0x1b220f0 set key 872 at 3
target key 844 temp_key 780
0x1b221a0 set key 780 at 0
target key 844 temp_key 879
0x1b220f0 set key 879 at 4
target key 844 temp_key 819
0x1b221a0 set key 819 at 1
target key 844 temp_key 768
0x1b221a0 set key 768 at 2
0x1b221a0 set key 785 at 3
 orig_nvnode: 0x1b220f0 with slot 5 new_leaf 0x1b221a0 with slot 4
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b21810 with new_sum 3152
s2 node_alloc : new node 0x1b22250 1 levels
0x1b22250 sum is 3152 0x1b1ea30 sum is 4387
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b22250
 s1 sl_insert_new: going to insert key 712 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 712 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 2
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1e260 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1e260
item is 0x1b1e500 next is 0x1b1e500 level is 1
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 3611 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
s3 find_index_node: 0x1b1e500 with min 708 max 753 sum 3611
 s3 sl_insert_new: going to insert key 712 index node exists 0x1b1e500 max: 753 min 708 sum 3611
0x1b1e450 set key 712 at 5
 s1 sl_insert_new: going to insert key 66 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 66 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b20370 next is 0x1b20370 level is 0
 s4 find_index_node: visiting item 0x1b20370 max 675 min 5 sum is 1582 next is 0x1b1f900
 s4 find_index_node: key 675
item->max: 675 item->min 5
s3 find_index_node: 0x1b20370 with min 5 max 675 sum 1582
 s3 sl_insert_new: going to insert key 66 index node exists 0x1b20370 max: 675 min 5 sum 1582
0x1b202c0 set key 66 at 7
 s1 sl_insert_new: going to insert key 264 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 264 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1f090 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
 s4 find_index_node: visiting item 0x1b1fe30 max 81 min 17 sum is 338 next is 0x1b200d0
 s4 find_index_node: key 81
item->max: 81 item->min 17
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1fe30
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 768 sum is 3152 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 768
 s3 find_index_node: foud pred 0x1b1fe30 next 0x1b21810 at level 0
 find_index_node:  foud proper place for key 264 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 264 index node exists 0x1b22250 max: 819 min 768 sum 3152
0x1b221a0 set key 264 at 4
 s1 sl_insert_new: going to insert key 628 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 628 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 5
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0 next is 0 level is 4
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0 next is 0 level is 3
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 603 sum is 4921 next is 0x1b1f090
 s4 find_index_node: key 639
item->max: 639 item->min 603
s3 find_index_node: 0x1b1e260 with min 603 max 639 sum 4921
 s3 sl_insert_new: going to insert key 628 index node exists 0x1b1e260 max: 639 min 603 sum 4921
 nvnode 0x1b209c0 is full, need to be split 
 sum of index_node: 0x1b1e260 is 4921
target key 615 temp_key 639
0x1b22380 set key 639 at 0
target key 615 temp_key 607
0x1b22430 set key 607 at 0
target key 615 temp_key 628
0x1b22380 set key 628 at 1
target key 615 temp_key 603
0x1b22430 set key 603 at 1
target key 615 temp_key 606
0x1b22430 set key 606 at 2
target key 615 temp_key 622
0x1b22380 set key 622 at 2
target key 615 temp_key 612
0x1b22430 set key 612 at 3
target key 615 temp_key 604
0x1b22430 set key 604 at 4
0x1b22380 set key 628 at 3
 orig_nvnode: 0x1b22380 with slot 4 new_leaf 0x1b22430 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b22250 with new_sum 3032
s2 node_alloc : new node 0x1b224e0 5 levels
0x1b224e0 sum is 3032 0x1b1e260 sum is 2517
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b224e0
preds level 1 is 0x1b1fe30 next is 0x1b224e0
preds level 2 is 0x1b1e260 next is 0x1b224e0
preds level 3 is 0x1b1cec0 next is 0x1b224e0
preds level 4 is 0x1b1cec0 next is 0x1b224e0
 s1 sl_insert_new: going to insert key 151 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 151 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 3
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b1e260 next is 0x1b1e260 level is 2
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 147 sum is 1618 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 147
s3 find_index_node: 0x1b1f900 with min 147 max 261 sum 1618
 s3 sl_insert_new: going to insert key 151 index node exists 0x1b1f900 max: 261 min 147 sum 1618
 nvnode 0x1b21180 is full, need to be split 
 sum of index_node: 0x1b1f900 is 1618
target key 202 temp_key 261
0x1b22630 set key 261 at 0
target key 202 temp_key 147
0x1b226e0 set key 147 at 0
target key 202 temp_key 169
0x1b226e0 set key 169 at 1
target key 202 temp_key 237
0x1b22630 set key 237 at 1
target key 202 temp_key 167
0x1b226e0 set key 167 at 2
target key 202 temp_key 167
0x1b226e0 set key 167 at 3
target key 202 temp_key 237
0x1b22630 set key 237 at 2
target key 202 temp_key 233
0x1b22630 set key 233 at 3
0x1b226e0 set key 151 at 4
 orig_nvnode: 0x1b22630 with slot 4 new_leaf 0x1b226e0 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b22250 with new_sum 801
s2 node_alloc : new node 0x1b22790 3 levels
0x1b22790 sum is 801 0x1b1f900 sum is 968
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b22790
preds level 1 is 0x1b1fe30 next is 0x1b22790
preds level 2 is 0x1b1cec0 next is 0x1b22790
 s1 sl_insert_new: going to insert key 167 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 167 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 801 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
s3 find_index_node: 0x1b22790 with min 147 max 169 sum 801
 s3 sl_insert_new: going to insert key 167 index node exists 0x1b22790 max: 169 min 147 sum 801
0x1b226e0 set key 167 at 5
 s1 sl_insert_new: going to insert key 476 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 476 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 1916 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
s3 find_index_node: 0x1b200d0 with min 467 max 492 sum 1916
 s3 sl_insert_new: going to insert key 476 index node exists 0x1b200d0 max: 492 min 467 sum 1916
0x1b216b0 set key 476 at 4
 s1 sl_insert_new: going to insert key 282 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 282 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 264 sum is 3416 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 264
s3 find_index_node: 0x1b22250 with min 264 max 819 sum 3416
 s3 sl_insert_new: going to insert key 282 index node exists 0x1b22250 max: 819 min 264 sum 3416
0x1b221a0 set key 282 at 5
 s1 sl_insert_new: going to insert key 437 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 437 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 264 sum is 3698 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 264
s3 find_index_node: 0x1b22250 with min 264 max 819 sum 3698
 s3 sl_insert_new: going to insert key 437 index node exists 0x1b22250 max: 819 min 264 sum 3698
0x1b221a0 set key 437 at 6
 s1 sl_insert_new: going to insert key 314 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 314 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 264 sum is 4135 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 264
s3 find_index_node: 0x1b22250 with min 264 max 819 sum 4135
 s3 sl_insert_new: going to insert key 314 index node exists 0x1b22250 max: 819 min 264 sum 4135
0x1b221a0 set key 314 at 7
 s1 sl_insert_new: going to insert key 370 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 370 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 264 sum is 4449 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 264
s3 find_index_node: 0x1b22250 with min 264 max 819 sum 4449
 s3 sl_insert_new: going to insert key 370 index node exists 0x1b22250 max: 819 min 264 sum 4449
 nvnode 0x1b221a0 is full, need to be split 
 sum of index_node: 0x1b22250 is 4449
target key 556 temp_key 780
0x1b228d0 set key 780 at 0
target key 556 temp_key 819
0x1b228d0 set key 819 at 1
target key 556 temp_key 768
0x1b228d0 set key 768 at 2
target key 556 temp_key 785
0x1b228d0 set key 785 at 3
target key 556 temp_key 264
0x1b22980 set key 264 at 0
target key 556 temp_key 282
0x1b22980 set key 282 at 1
target key 556 temp_key 437
0x1b22980 set key 437 at 2
target key 556 temp_key 314
0x1b22980 set key 314 at 3
0x1b22980 set key 370 at 4
 orig_nvnode: 0x1b228d0 with slot 4 new_leaf 0x1b22980 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1fe30 and 0x1b22250 with new_sum 1667
s2 node_alloc : new node 0x1b22a30 1 levels
0x1b22a30 sum is 1667 0x1b22250 sum is 3152
 pred of new_index is 0x1b1fe30 sl->head is 0x1b1cec0
preds level 0 is 0x1b1fe30 next is 0x1b22a30
 s1 sl_insert_new: going to insert key 976 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 976 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 3754 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 4323 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 872 sum is 4387 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 872
 s4 find_index_node: visiting item 0x1b20db0 max 681 min 681 sum is 681 next is 0x1b1f090
 s4 find_index_node: key 681
item->max: 681 item->min 681
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b20db0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b20db0
item is 0x1b200d0 next is 0x1b200d0 level is 0
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 305 sum is 2907 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 305
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 249 sum is 1432 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 249
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 792 sum is 4118 next is 0x1b21aa0
 s4 find_index_node: key 848
item->max: 848 item->min 792
 s4 find_index_node: visiting item 0x1b21aa0 max 909 min 884 sum is 5388 next is 0x1b1d810
 s4 find_index_node: key 909
item->max: 909 item->min 884
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 916 sum is 2806 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 916
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 9707 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 989 sum is 989 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 989
 s3 find_index_node: foud pred 0x1b1dc80 next 0 at level 0
 find_index_node:  foud proper place for key 976 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 976 index node exists 0x1b1f090 max: 989 min 989 sum 989
0x1b1f1d0 set key 976 at 5
 s1 sl_insert_new: going to insert key 970 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 970 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 976 sum is 1965 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 976
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 3754 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 4323 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 872 sum is 4387 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 872
 s4 find_index_node: visiting item 0x1b20db0 max 681 min 681 sum is 681 next is 0x1b1f090
 s4 find_index_node: key 681
item->max: 681 item->min 681
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 976 sum is 1965 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 976
 s3 find_index_node: foud pred 0x1b20db0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b20db0
item is 0x1b200d0 next is 0x1b200d0 level is 0
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 305 sum is 2907 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 305
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 249 sum is 1432 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 249
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 792 sum is 4118 next is 0x1b21aa0
 s4 find_index_node: key 848
item->max: 848 item->min 792
 s4 find_index_node: visiting item 0x1b21aa0 max 909 min 884 sum is 5388 next is 0x1b1d810
 s4 find_index_node: key 909
item->max: 909 item->min 884
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 916 sum is 2806 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 916
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 9707 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
s3 find_index_node: 0x1b1dc80 with min 126 max 972 sum 9707
 s3 sl_insert_new: going to insert key 970 index node exists 0x1b1dc80 max: 972 min 126 sum 9707
0x1b1f1d0 set key 970 at 6
 s1 sl_insert_new: going to insert key 198 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 198 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 768 sum is 3152 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 768
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21810 at level 0
 find_index_node:  foud proper place for key 198 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 198 index node exists 0x1b22250 max: 819 min 768 sum 3152
0x1b228d0 set key 198 at 4
 s1 sl_insert_new: going to insert key 657 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 657 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 976 sum is 1965 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 976
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 3754 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 4323 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b20600
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b20600 max 848 min 792 sum is 4118 next is 0x1b21aa0
 s4 find_index_node: key 848
item->max: 848 item->min 792
 s3 find_index_node: foud pred 0x1b1e7a0 next 0x1b21aa0 at level 0
 find_index_node:  foud proper place for key 657 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 657 index node exists 0x1b20600 max: 848 min 792 sum 4118
0x1b21e60 set key 657 at 5
 s1 sl_insert_new: going to insert key 9 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 9 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s3 find_index_node: foud pred 0x1b1cec0 next 0x1b1e260 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b1cec0
item is 0x1b1dfc0 next is 0x1b1dfc0 level is 1
 s4 find_index_node: visiting item 0x1b1dfc0 max 0 min 589 sum is 0 next is 0x1b1f900
 s4 find_index_node: key 0
item->max: 0 item->min 589
 s4 find_index_node: visiting item 0x1b1f900 max 261 min 233 sum is 968 next is 0x1b1fe30
 s4 find_index_node: key 261
item->max: 261 item->min 233
 s3 find_index_node: foud pred 0x1b1dfc0 next 0x1b1fe30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1dfc0
item is 0x1b20370 next is 0x1b20370 level is 0
 s4 find_index_node: visiting item 0x1b20370 max 675 min 5 sum is 1648 next is 0x1b1f900
 s4 find_index_node: key 675
item->max: 675 item->min 5
s3 find_index_node: 0x1b20370 with min 5 max 675 sum 1648
 s3 sl_insert_new: going to insert key 9 index node exists 0x1b20370 max: 675 min 5 sum 1648
 nvnode 0x1b202c0 is full, need to be split 
 sum of index_node: 0x1b20370 is 1648
target key 206 temp_key 651
0x1b22b60 set key 651 at 0
target key 206 temp_key 675
0x1b22b60 set key 675 at 1
target key 206 temp_key 5
0x1b22c10 set key 5 at 0
target key 206 temp_key 11
0x1b22c10 set key 11 at 1
target key 206 temp_key 6
0x1b22c10 set key 6 at 2
target key 206 temp_key 127
0x1b22c10 set key 127 at 3
target key 206 temp_key 107
0x1b22c10 set key 107 at 4
target key 206 temp_key 66
0x1b22c10 set key 66 at 5
0x1b22c10 set key 9 at 6
 orig_nvnode: 0x1b22b60 with slot 2 new_leaf 0x1b22c10 with slot 7
 sl_insert_new: attempting to insert an new index node between 0x1b1e7a0 and 0x1b20600 with new_sum 331
s2 node_alloc : new node 0x1b22cc0 1 levels
0x1b22cc0 sum is 331 0x1b20370 sum is 1326
 pred of new_index is 0x1b1e7a0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1e7a0 next is 0x1b22cc0
 s1 sl_insert_new: going to insert key 415 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 415 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 198 sum is 3350 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 198
s3 find_index_node: 0x1b22250 with min 198 max 819 sum 3350
 s3 sl_insert_new: going to insert key 415 index node exists 0x1b22250 max: 819 min 198 sum 3350
0x1b228d0 set key 415 at 5
 s1 sl_insert_new: going to insert key 305 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 305 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 198 sum is 3765 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 198
s3 find_index_node: 0x1b22250 with min 198 max 819 sum 3765
 s3 sl_insert_new: going to insert key 305 index node exists 0x1b22250 max: 819 min 198 sum 3765
0x1b228d0 set key 305 at 6
 s1 sl_insert_new: going to insert key 389 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 389 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 198 sum is 4070 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 198
s3 find_index_node: 0x1b22250 with min 198 max 819 sum 4070
 s3 sl_insert_new: going to insert key 389 index node exists 0x1b22250 max: 819 min 198 sum 4070
0x1b228d0 set key 389 at 7
 s1 sl_insert_new: going to insert key 282 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 282 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 2
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22790 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22790
item is 0x1b22250 next is 0x1b22250 level is 0
 s4 find_index_node: visiting item 0x1b22250 max 819 min 198 sum is 4459 next is 0x1b21810
 s4 find_index_node: key 819
item->max: 819 item->min 198
s3 find_index_node: 0x1b22250 with min 198 max 819 sum 4459
 s3 sl_insert_new: going to insert key 282 index node exists 0x1b22250 max: 819 min 198 sum 4459
 nvnode 0x1b228d0 is full, need to be split 
 sum of index_node: 0x1b22250 is 4459
target key 557 temp_key 780
0x1b22df0 set key 780 at 0
target key 557 temp_key 819
0x1b22df0 set key 819 at 1
target key 557 temp_key 768
0x1b22df0 set key 768 at 2
target key 557 temp_key 785
0x1b22df0 set key 785 at 3
target key 557 temp_key 198
0x1b22ea0 set key 198 at 0
target key 557 temp_key 415
0x1b22ea0 set key 415 at 1
target key 557 temp_key 305
0x1b22ea0 set key 305 at 2
target key 557 temp_key 389
0x1b22ea0 set key 389 at 3
0x1b22ea0 set key 282 at 4
 orig_nvnode: 0x1b22df0 with slot 4 new_leaf 0x1b22ea0 with slot 5
 sl_insert_new: attempting to insert an new index node between 0x1b1e7a0 and 0x1b20600 with new_sum 1589
s2 node_alloc : new node 0x1b22f50 2 levels
0x1b22f50 sum is 1589 0x1b22250 sum is 3152
 pred of new_index is 0x1b1e7a0 sl->head is 0x1b1cec0
preds level 0 is 0x1b1e7a0 next is 0x1b22f50
preds level 1 is 0x1b22790 next is 0x1b22f50
 s1 sl_insert_new: going to insert key 450 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 450 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 2
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b22f50 next is 0x1b22f50 level is 1
 s4 find_index_node: visiting item 0x1b22f50 max 415 min 198 sum is 1589 next is 0x1b200d0
 s4 find_index_node: key 415
item->max: 415 item->min 198
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s3 find_index_node: foud pred 0x1b22f50 next 0x1b21570 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b22f50
item is 0x1b20600 next is 0x1b20600 level is 0
 s4 find_index_node: visiting item 0x1b20600 max 848 min 657 sum is 4775 next is 0x1b21aa0
 s4 find_index_node: key 848
item->max: 848 item->min 657
 s3 find_index_node: foud pred 0x1b22f50 next 0x1b21aa0 at level 0
 find_index_node:  foud proper place for key 450 in skiplist. pred is  returning null 
 s3 sl_insert_new: going to insert key 450 index node exists 0x1b20600 max: 848 min 657 sum 4775
0x1b21e60 set key 450 at 6
 s1 sl_insert_new: going to insert key 560 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 560 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b22f50 next is 0x1b22f50 level is 1
 s4 find_index_node: visiting item 0x1b22f50 max 415 min 198 sum is 1589 next is 0x1b200d0
 s4 find_index_node: key 415
item->max: 415 item->min 198
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 3754 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
s3 find_index_node: 0x1b21570 with min 378 max 588 sum 3754
 s3 sl_insert_new: going to insert key 560 index node exists 0x1b21570 max: 588 min 378 sum 3754
0x1b214c0 set key 560 at 7
 s1 sl_insert_new: going to insert key 394 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 394 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b22f50 next is 0x1b22f50 level is 1
 s4 find_index_node: visiting item 0x1b22f50 max 415 min 198 sum is 1589 next is 0x1b200d0
 s4 find_index_node: key 415
item->max: 415 item->min 198
s3 find_index_node: 0x1b22f50 with min 198 max 415 sum 1589
 s3 sl_insert_new: going to insert key 394 index node exists 0x1b22f50 max: 415 min 198 sum 1589
0x1b22ea0 set key 394 at 5
 s1 sl_insert_new: going to insert key 227 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 227 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b224e0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s3 find_index_node: foud pred 0x1b22790 next 0x1b224e0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b22790
item is 0x1b22f50 next is 0x1b22f50 level is 1
 s4 find_index_node: visiting item 0x1b22f50 max 415 min 198 sum is 1983 next is 0x1b200d0
 s4 find_index_node: key 415
item->max: 415 item->min 198
s3 find_index_node: 0x1b22f50 with min 198 max 415 sum 1983
 s3 sl_insert_new: going to insert key 227 index node exists 0x1b22f50 max: 415 min 198 sum 1983
0x1b22ea0 set key 227 at 6
 s1 sl_insert_new: going to insert key 165 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 165 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 4
 s3 find_index_node: traverling level  3 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 3
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b1cec0 next 0 at level 3
 s3 find_index_node: traverling level  2 starting at 0x1b1cec0
item is 0x1b22790 next is 0x1b22790 level is 2
 s4 find_index_node: visiting item 0x1b22790 max 169 min 147 sum is 968 next is 0x1b1e260
 s4 find_index_node: key 169
item->max: 169 item->min 147
s3 find_index_node: 0x1b22790 with min 147 max 169 sum 968
 s3 sl_insert_new: going to insert key 165 index node exists 0x1b22790 max: 169 min 147 sum 968
0x1b226e0 set key 165 at 6
 s1 sl_insert_new: going to insert key 966 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 966 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 976 sum is 1965 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 976
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 4314 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 4323 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 872 sum is 4387 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 872
 s4 find_index_node: visiting item 0x1b20db0 max 681 min 681 sum is 681 next is 0x1b1f090
 s4 find_index_node: key 681
item->max: 681 item->min 681
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 976 sum is 1965 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 976
 s3 find_index_node: foud pred 0x1b20db0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b20db0
item is 0x1b200d0 next is 0x1b200d0 level is 0
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 305 sum is 2907 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 305
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 249 sum is 1432 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 249
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b22f50
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b22f50 max 415 min 198 sum is 2210 next is 0x1b20600
 s4 find_index_node: key 415
item->max: 415 item->min 198
 s4 find_index_node: visiting item 0x1b20600 max 848 min 450 sum is 5225 next is 0x1b21aa0
 s4 find_index_node: key 848
item->max: 848 item->min 450
 s4 find_index_node: visiting item 0x1b21aa0 max 909 min 884 sum is 5388 next is 0x1b1d810
 s4 find_index_node: key 909
item->max: 909 item->min 884
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 916 sum is 2806 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 916
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 10677 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
s3 find_index_node: 0x1b1dc80 with min 126 max 972 sum 10677
 s3 sl_insert_new: going to insert key 966 index node exists 0x1b1dc80 max: 972 min 126 sum 10677
0x1b1f1d0 set key 966 at 7
 s1 sl_insert_new: going to insert key 988 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 988 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 976 sum is 1965 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 976
s3 find_index_node: 0x1b1f090 with min 976 max 989 sum 1965
 s3 sl_insert_new: going to insert key 988 index node exists 0x1b1f090 max: 989 min 976 sum 1965
 nvnode 0x1b1f1d0 is full, need to be split 
 sum of index_node: 0x1b1f090 is 1965
target key 245 temp_key 989
0x1b23090 set key 989 at 0
target key 245 temp_key 938
0x1b23090 set key 938 at 1
target key 245 temp_key 951
0x1b23090 set key 951 at 2
target key 245 temp_key 965
0x1b23090 set key 965 at 3
target key 245 temp_key 962
0x1b23090 set key 962 at 4
target key 245 temp_key 976
0x1b23090 set key 976 at 5
target key 245 temp_key 970
0x1b23090 set key 970 at 6
target key 245 temp_key 966
0x1b23090 set key 966 at 7
0x1b23090 set key 988 at -1
 orig_nvnode: 0x1b23090 with slot 9 new_leaf 0x1b23140 with slot 0
 sl_insert_new: attempting to insert an new index node between 0x1b22f50 and 0x1b20600 with new_sum 0
s2 node_alloc : new node 0x1b231f0 1 levels
0x1b231f0 sum is 0 0x1b1f090 sum is 8705
 pred of new_index is 0x1b22f50 sl->head is 0x1b1cec0
preds level 0 is 0x1b22f50 next is 0x1b231f0
 s1 sl_insert_new: going to insert key 750 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 750 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 938 sum is 8705 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 938
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 4314 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 4323 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
s3 find_index_node: 0x1b1e500 with min 708 max 753 sum 4323
 s3 sl_insert_new: going to insert key 750 index node exists 0x1b1e500 max: 753 min 708 sum 4323
0x1b1e450 set key 750 at 6
 s1 sl_insert_new: going to insert key 924 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 924 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 938 sum is 8705 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 938
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 4314 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 5073 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s4 find_index_node: visiting item 0x1b1ea30 max 883 min 872 sum is 4387 next is 0x1b20db0
 s4 find_index_node: key 883
item->max: 883 item->min 872
 s4 find_index_node: visiting item 0x1b20db0 max 681 min 681 sum is 681 next is 0x1b1f090
 s4 find_index_node: key 681
item->max: 681 item->min 681
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 938 sum is 8705 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 938
 s3 find_index_node: foud pred 0x1b20db0 next 0 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b20db0
item is 0x1b200d0 next is 0x1b200d0 level is 0
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b1fba0
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b1fba0 max 693 min 648 sum is 2019 next is 0x1b1f670
 s4 find_index_node: key 693
item->max: 693 item->min 648
 s4 find_index_node: visiting item 0x1b1f670 max 526 min 305 sum is 2907 next is 0x1b1ecd0
 s4 find_index_node: key 526
item->max: 526 item->min 305
 s4 find_index_node: visiting item 0x1b1ecd0 max 344 min 249 sum is 1432 next is 0x1b1e260
 s4 find_index_node: key 344
item->max: 344 item->min 249
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e7a0
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b22f50
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b22f50 max 415 min 198 sum is 2210 next is 0x1b231f0
 s4 find_index_node: key 415
item->max: 415 item->min 198
 s4 find_index_node: visiting item 0x1b231f0 max 0 min 976 sum is 0 next is 0x1b20600
 s4 find_index_node: key 0
item->max: 0 item->min 976
 s4 find_index_node: visiting item 0x1b20600 max 848 min 450 sum is 5225 next is 0x1b21aa0
 s4 find_index_node: key 848
item->max: 848 item->min 450
 s4 find_index_node: visiting item 0x1b21aa0 max 909 min 884 sum is 5388 next is 0x1b1d810
 s4 find_index_node: key 909
item->max: 909 item->min 884
 s4 find_index_node: visiting item 0x1b1d810 max 922 min 916 sum is 2806 next is 0x1b1ef60
 s4 find_index_node: key 922
item->max: 922 item->min 916
 s4 find_index_node: visiting item 0x1b1ef60 max 0 min 928 sum is 0 next is 0x1b1dc80
 s4 find_index_node: key 0
item->max: 0 item->min 928
 s4 find_index_node: visiting item 0x1b1dc80 max 972 min 126 sum is 11643 next is 0x1b1f090
 s4 find_index_node: key 972
item->max: 972 item->min 126
s3 find_index_node: 0x1b1dc80 with min 126 max 972 sum 11643
 s3 sl_insert_new: going to insert key 924 index node exists 0x1b1dc80 max: 972 min 126 sum 11643
 nvnode 0x1b1f1d0 is full, need to be split 
 sum of index_node: 0x1b1dc80 is 11643
target key 1455 temp_key 989
0x1b233d0 set key 989 at 0
target key 1455 temp_key 938
0x1b233d0 set key 938 at 1
target key 1455 temp_key 951
0x1b233d0 set key 951 at 2
target key 1455 temp_key 965
0x1b233d0 set key 965 at 3
target key 1455 temp_key 962
0x1b233d0 set key 962 at 4
target key 1455 temp_key 976
0x1b233d0 set key 976 at 5
target key 1455 temp_key 970
0x1b233d0 set key 970 at 6
target key 1455 temp_key 966
0x1b233d0 set key 966 at 7
0x1b233d0 set key 924 at -1
 orig_nvnode: 0x1b23320 with slot 0 new_leaf 0x1b233d0 with slot 9
 sl_insert_new: attempting to insert an new index node between 0x1b22f50 and 0x1b20600 with new_sum 8641
s2 node_alloc : new node 0x1b23480 1 levels
0x1b23480 sum is 8641 0x1b1dc80 sum is 0
 pred of new_index is 0x1b22f50 sl->head is 0x1b1cec0
preds level 0 is 0x1b22f50 next is 0x1b23480
 s1 sl_insert_new: going to insert key 626 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 626 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 938 sum is 8705 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 938
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 4314 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 2517 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
s3 find_index_node: 0x1b1e260 with min 622 max 639 sum 2517
 s3 sl_insert_new: going to insert key 626 index node exists 0x1b1e260 max: 639 min 622 sum 2517
0x1b22380 set key 626 at 4
 s1 sl_insert_new: going to insert key 645 into sl 0x1b1ce70
 s2 random_levels: increased high water mark to 5
 s2 find_index_node: searching for min key that >= key 645 in skiplist head is 0x1b1cec0 sl->high_water is 5 n is 1
 s3 find_index_node: traverling level  4 starting at 0x1b1cec0
item is 0x1b224e0 next is 0x1b224e0 level is 4
 s4 find_index_node: visiting item 0x1b224e0 max 612 min 603 sum is 3032 next is 0
 s4 find_index_node: key 612
item->max: 612 item->min 603
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 4
 s3 find_index_node: traverling level  2 starting at 0x1b224e0
item is 0x1b1f090 next is 0x1b1f090 level is 2
 s4 find_index_node: visiting item 0x1b1f090 max 989 min 938 sum is 8705 next is 0
 s4 find_index_node: key 989
item->max: 989 item->min 938
 s3 find_index_node: foud pred 0x1b224e0 next 0 at level 2
 s3 find_index_node: traverling level  1 starting at 0x1b224e0
item is 0x1b200d0 next is 0x1b200d0 level is 1
 s4 find_index_node: visiting item 0x1b200d0 max 492 min 467 sum is 2392 next is 0x1b21570
 s4 find_index_node: key 492
item->max: 492 item->min 467
 s4 find_index_node: visiting item 0x1b21570 max 588 min 378 sum is 4314 next is 0x1b1e260
 s4 find_index_node: key 588
item->max: 588 item->min 378
 s4 find_index_node: visiting item 0x1b1e260 max 639 min 622 sum is 3143 next is 0x1b1e500
 s4 find_index_node: key 639
item->max: 639 item->min 622
 s4 find_index_node: visiting item 0x1b1e500 max 753 min 708 sum is 5073 next is 0x1b1ea30
 s4 find_index_node: key 753
item->max: 753 item->min 708
 s3 find_index_node: foud pred 0x1b1e260 next 0x1b1ea30 at level 1
 s3 find_index_node: traverling level  0 starting at 0x1b1e260
item is 0x1b1e7a0 next is 0x1b1e7a0 level is 0
 s4 find_index_node: visiting item 0x1b1e7a0 max 219 min 95 sum is 981 next is 0x1b22f50
 s4 find_index_node: key 219
item->max: 219 item->min 95
 s4 find_index_node: visiting item 0x1b22f50 max 415 min 198 sum is 2210 next is 0x1b23480
 s4 find_index_node: key 415
item->max: 415 item->min 198
 s4 find_index_node: visiting item 0x1b23480 max 989 min 126 sum is 8641 next is 0x1b20600
 s4 find_index_node: key 989
item->max: 989 item->min 126
s3 find_index_node: 0x1b23480 with min 126 max 989 sum 8641
 s3 sl_insert_new: going to insert key 645 index node exists 0x1b23480 max: 989 min 126 sum 8641
