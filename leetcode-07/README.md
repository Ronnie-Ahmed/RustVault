# LeetCode 7: Reverse Integer

## Problem Description
Given a signed 32-bit integer `x`, return `x` with its digits reversed. If reversing `x` causes the value to go outside the signed 32-bit integer range $[-2^{31}, 2^{31} - 1]$, then return `0`.

## Approach
We can extract the digits mathematically:
1. Handle the sign of the integer.
2. Extract the last digit using modulo (`x % 10`) and add it to the reversed number after shifting previous digits left (`reversed_num * 10 + digit`).
3. Integer-divide `x` by 10 to remove the last digit.
4. Check for 32-bit integer overflow bounds before completing the operation.

## Complexity Analysis
- **Time Complexity:** $O(\log_{10}(x))$, as the number of iterations is proportional to the number of digits in `x`.
- **Space Complexity:** $O(1)$, using only a few variables.