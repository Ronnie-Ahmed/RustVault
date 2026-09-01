# 202. Happy Number

**Difficulty:** Easy
**Topics:** Hash Table, Math, Two Pointers
**Link:** [LeetCode - 202. Happy Number](https://leetcode.com/problems/happy-number/)

## Problem

Write an algorithm to determine if a number `n` is happy.

A happy number is a number defined by the following process:

- Starting with any positive integer, replace the number by the sum of the squares of its digits.
- Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
- Those numbers for which this process ends in 1 are happy.

Return `true` if `n` is a happy number, and `false` if not.

### Example 1

```
Input: n = 19
Output: true
Explanation:
1^2 + 9^2 = 82
8^2 + 2^2 = 68
6^2 + 8^2 = 100
1^2 + 0^2 + 0^2 = 1
```

### Example 2

```
Input: n = 2
Output: false
```

### Constraints

- `1 <= n <= 2^31 - 1`

## Approach

This solution uses **Floyd's Cycle Detection** (tortoise and hare) instead of a `HashSet`, giving O(1) extra space.

- `slow` advances one digit-square-sum transformation per iteration.
- `fast` advances two transformations per iteration.
- If `n` is happy, `fast` reaches `1` before `slow` can catch up.
- If `n` is unhappy, the sequence falls into a cycle. Since `fast` moves twice as fast as `slow`, they are guaranteed to eventually land on the same value within that cycle — at which point the number is confirmed unhappy.

### Complexity

- **Time:** O(log n) per transformation, effectively very fast in practice.
- **Space:** O(1) — no extra data structures required.

## Solution (Rust)

```rust
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn sum_of_squares(mut num: i32) -> i32 {
            let mut sum = 0;
            while num > 0 {
                let digit = num % 10;
                sum += digit * digit;
                num /= 10;
            }
            sum
        }

        let mut slow = n;
        let mut fast = sum_of_squares(n);

        while fast != 1 && slow != fast {
            slow = sum_of_squares(slow);
            fast = sum_of_squares(sum_of_squares(fast));
        }

        fast == 1
    }
}
```

## Trace Examples

**n = 19** (happy):
```
19 → 82 → 68 → 100 → 1
```

**n = 2** (not happy, falls into a cycle):
```
2 → 4 → 16 → 37 → 58 → 89 → 145 → 42 → 20 → 4 → ...
```