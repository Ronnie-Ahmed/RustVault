# 258. Add Digits

**Difficulty:** Easy  
**Language:** Rust  
**Topics:** Math, Number Theory, Simulation  

---

## Problem Description

Given an integer `num`, repeatedly add all its digits until the result has only one digit, and return it.

---

## Examples

### Example 1:
**Input:** `num = 38`  
**Output:** `2`  
**Explanation:** The process is:
$$38 \implies 3 + 8 = 11$$
$$11 \implies 1 + 1 = 2$$
Since `2` has only one digit, return `2`.

### Example 2:
**Input:** `num = 0`  
**Output:** `0`

---

## Constraints

* $0 \le \text{num} \le 2^{31} - 1$

---

## Approach & Intuition

### Approach 1: Digital Root Formula ($O(1)$ Time & Space)

> **Follow up:** Could you do it without any loop/recursion in $O(1)$ runtime?

This problem asks for the **Digital Root** of a number in base 10. Mathematically, the digital root of a non-zero integer $N$ is congruent to $N \pmod 9$.

The formula for digital root in base 10 is:
$$dr(n) = 1 + (n - 1) \pmod 9$$

#### Edge Cases & Properties:
1. If $\text{num} = 0$, the answer is `0`.
2. If $\text{num} > 0$ and $\text{num} \pmod 9 == 0$, the digital root is `9`.
3. Otherwise, the digital root is $\text{num} \pmod 9$.

