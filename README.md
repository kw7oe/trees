# Trees

An attempt to implement different type of tree structure in Rust.
Inspired by [Learn Rust With Entirely Too Many Linked
Lists](https://rust-unofficial.github.io/too-many-lists/index.html).

Currently implemented:

- Binary Search Tree
- B Tree
  - based on Introduction to Algorithms, B Tree chapter and [Programiz B
    Tree](https://www.programiz.com/dsa/b-tree) for deletion.
- B+ Tree
  - based on [Programiz B+ Tree](https://www.programiz.com/dsa/b-plus-tree)
    and [B+ Tree Visualization](https://www.cs.usfca.edu/~galles/visualization/BPlusTree.html)
  - currently does not implement pointers in leaf node
  - current implementation for delete and rebalancing is messy. It definitely
    requires clean up to make the logic simpler.
