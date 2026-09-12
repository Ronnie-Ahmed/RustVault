# 263. Ugly Number

**Difficulty:** Easy  
**Language:** Rust  
**Topics:** Math, Number Theory  

---

## Problem Description

An **ugly number** is a positive integer whose prime factors are limited to `2`, `3`, and `5`.

Given an integer `n`, return `true` if `n` is an **ugly number**, and `false` otherwise.

---

## Examples

### Example 1:
**Input:** `n = 6`  
**Output:** `true`  
**Explanation:** $6 = 2 \times 3$ (its prime factors are only 2 and 3).

### Example 2:
**Input:** `n = 1`  
**Output:** `true`  
**Explanation:** `1` has no prime factors, which satisfies the condition (by definition, $1$ is considered an ugly number).

### Example 3:
**Input:** `n = 14`  
**Output:** `false`  
**Explanation:** $14 = 2 \times 7$ (it contains a prime factor `7`, which is not permitted).

---

## Constraints

* $-2^{31} \le n \le 2^{31} - 1$

---

## Solution Approach

### Iterative Factor Reduction

An integer $n$ is ugly if its prime factorization contains **only** $2$, $3$, or $5$:

$$n = 2^a \times 3^b \times 5^c$$

#### Algorithm:
1. **Edge Case:** If $n \le 0$, return `false` immediately because ugly numbers must be strictly positive.
2. Continuously divide $n$ by `2` as long as $n \pmod 2 == 0$.
3. Continuously divide $n$ by `3` as long as $n \pmod 3 == 0$.
4. Continuously divide $n$ by `5` as long as $n \pmod 5 == 0$.
5. If $n$ reduces completely down to `1`, then all its prime factors were restricted to $\{2, 3, 5\}$, making it **ugly**. If $n > 1$, it contains another prime factor (e.g., $7, 11, 13$), making it **not ugly**.

---

