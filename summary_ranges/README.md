# 228. Summary Ranges

![Difficulty](https://img.shields.io/badge/Difficulty-Easy-brightgreen?style=for-the-badge)
![Language](https://img.shields.io/badge/Language-Rust-CE412B?style=for-the-badge&logo=rust&logoColor=white)

## Problem Description

You are given a **sorted unique** integer array `nums`.

A range `[a,b]` is the set of all integers from `a` to `b` (inclusive).

Return *the smallest sorted list of ranges that cover all the numbers in the array exactly*. That is, each element of `nums` is covered by exactly one of the ranges, and there is no integer `x` such that `x` is in one of the ranges but not in `nums`.

Each range `[a,b]` in the list should be output as:
* `"a->b"` if `a != b`
* `"a"` if `a == b`

---

## Examples

### Example 1
```text
Input: nums = [0,1,2,4,5,7]
Output: ["0->2","4->5","7"]
Explanation: The ranges are:
[0,2] --> "0->2"
[4,5] --> "4->5"
[7,7] --> "7"