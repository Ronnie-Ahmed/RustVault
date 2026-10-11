# LeetCode 392: Is Subsequence

## Problem Description
Given two strings `s` and `t`, return `true` if `s` is a subsequence of `t`, or `false` otherwise. A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters.

## Approach
We can use a **Two-Pointer** approach:
1. Initialize one pointer `i` for string `s` and another pointer `j` for string `t`.
2. Iterate through string `t` using `j`. If the character at `t[j]` matches `s[i]`, increment `i`.
3. Increment `j` in every iteration.
4. If `i` reaches the length of `s`, it means all characters of `s` were found in `t` in the correct order, so return `true`. Otherwise, return `false`.

## Complexity Analysis
- **Time Complexity:** $O(N)$, where $N$ is the length of string `t`. In the worst case, we traverse `t` completely.
- **Space Complexity:** $O(1)$, since we only use two pointers.