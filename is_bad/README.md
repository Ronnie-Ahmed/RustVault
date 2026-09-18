# 278. First Bad Version

**Difficulty:** Easy  
**Topic:** Binary Search, Interactive  
**LeetCode Link:** [278. First Bad Version](https://leetcode.com/problems/first-bad-version/)

---

## Problem Description

You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.

Suppose you have $n$ versions $[1, 2, \dots, n]$ and you want to find out the first bad one, which causes all the following ones to be bad.

You are given an API `isBadVersion(version)` which returns whether `version` is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.

---

## Examples

### Example 1
**Input:** `n = 5`, `bad = 4`  
**Output:** `4`  
**Explanation:**  
* `isBadVersion(3)` $\rightarrow$ `false`  
* `isBadVersion(5)` $\rightarrow$ `true`  
* `isBadVersion(4)` $\rightarrow$ `true`  
Then `4` is the first bad version.

### Example 2
**Input:** `n = 1`, `bad = 1`  
**Output:** `1`  

---

## Constraints

* $1 \le \text{bad} \le n \le 2^{31} - 1$

---

## Solution & Approach

### Binary Search Approach

A naive linear search from $1$ to $n$ requires $\mathcal{O}(n)$ API calls, which results in a **Time Limit Exceeded (TLE)** error for large values of $n$.

Since the array of versions is sorted (`[false, false, ..., true, true]`), we can apply **Binary Search**:
1. Maintain two pointers: `left = 1` and `right = n`.
2. Compute `mid = left + (right - left) / 2` to prevent potential integer overflow when $left + right > \text{i32::MAX}$.
3. If `isBadVersion(mid)` returns `true`, then `mid` could be the first bad version or the first bad version lies to its left. We narrow our search space by setting `right = mid`.
4. If `isBadVersion(mid)` returns `false`, then `mid` and all preceding versions are good. The first bad version must lie to the right, so we set `left = mid + 1`.
5. The loop terminates when `left == right`, pointing to the first bad version.

