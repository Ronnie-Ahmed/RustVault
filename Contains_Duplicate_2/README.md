# 219. Contains Duplicate II

**Difficulty:** Easy
**Status:** Solved
**Link:** [leetcode.com/problems/contains-duplicate-ii](https://leetcode.com/problems/contains-duplicate-ii/)

## Problem

Given an integer array `nums` and an integer `k`, return `true` if there are two
distinct indices `i` and `j` in the array such that `nums[i] == nums[j]` and
`abs(i - j) <= k`.

### Examples

```
Input: nums = [1,2,3,1], k = 3
Output: true

Input: nums = [1,0,1,1], k = 1
Output: true

Input: nums = [1,2,3,1,2,3], k = 2
Output: false
```

### Constraints

- `1 <= nums.length <= 10^5`
- `-10^9 <= nums[i] <= 10^9`
- `0 <= k <= 10^5`

## Approach

A single left-to-right pass with a hash map that tracks the **most recent
index** at which each value was seen.

For every element:
1. If the value already exists in the map, check whether the gap between the
   current index and the stored index is `<= k`. If so, return `true`.
2. Update the map with the current index for that value (whether or not it
   was a match) — we always want the *closest* previous occurrence going
   forward, since it gives the duplicate its best chance of satisfying the
   distance constraint.
3. If the scan finishes with no match, return `false`.

This only needs one pass and one lookup/update per element, so it's linear
in the size of the input.

## Complexity

- **Time:** O(n) — one pass, O(1) average-case hash map operations
- **Space:** O(min(n, distinct values)) — the map holds at most one entry
  per distinct value seen so far