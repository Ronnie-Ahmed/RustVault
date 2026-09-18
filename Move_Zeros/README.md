# 283. Move Zeroes

**Difficulty:** Easy  
**Topic:** Array, Two Pointers  
**LeetCode Link:** [283. Move Zeroes](https://leetcode.com/problems/move-zeroes/)

---

## Problem Description

Given an integer array `nums`, move all `0`'s to the end of it while maintaining the relative order of the non-zero elements.

**Note:** You must do this **in-place** without making a copy of the array.

---

## Examples

### Example 1
**Input:** `nums = [0,1,0,3,12]`  
**Output:** `[1,3,12,0,0]`  

### Example 2
**Input:** `nums = [0]`  
**Output:** `[0]`  

---

## Constraints

* $1 \le \text{nums.length} \le 10^4$
* $-2^{31} \le \text{nums}[i] \le 2^{31} - 1$

---

## Solution & Approach

### Two-Pointer Swap Approach

To solve this in **$\mathcal{O}(n)$ time complexity** and **$\mathcal{O}(1)$ space complexity**, we use a two-pointer technique:
1. `write_ptr`: Keeps track of the index where the next non-zero element should be placed.
2. `read_ptr`: Iterates through the array to find non-zero elements.

Whenever `read_ptr` encounters a non-zero element, it swaps values with `write_ptr`, and `write_ptr` advances by 1. This guarantees that all non-zero elements stay in their original relative order at the front of the array, while pushing all zeroes to the end.


