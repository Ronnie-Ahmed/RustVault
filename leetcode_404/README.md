# LeetCode 404: Sum of Left Leaves

## Problem Description
Given the `root` of a binary tree, return the sum of all left leaves. A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.

## Approach
We can solve this recursively using **Depth-First Search (DFS)**:
1. Traverse the binary tree while keeping track of whether a node is a left child (e.g., using a boolean flag `is_left`).
2. If a node is a leaf (no left and no right children) **and** `is_left` is `true`, add its value to our sum.
3. Recursively call the helper function for the left child (`is_left = True`) and the right child (`is_left = False`).

## Complexity Analysis
- **Time Complexity:** $O(N)$, where $N$ is the number of nodes in the binary tree, as we visit every node once.
- **Space Complexity:** $O(H)$, where $H$ is the height of the tree, representing the recursive call stack space.