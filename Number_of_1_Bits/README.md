# [191. Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/)

**Difficulty:** Easy
**Status:** Solved

## Problem

Given a positive integer `n`, write a function that returns the number of set bits in its binary representation (also known as the [Hamming weight](http://en.wikipedia.org/wiki/Hamming_weight)).

### Example 1

```
Input: n = 11
Output: 3
Explanation: The input binary string 1011 has a total of three set bits.
```

### Example 2

```
Input: n = 128
Output: 1
Explanation: The input binary string 10000000 has a total of one set bit.
```

### Example 3

```
Input: n = 2147483645
Output: 30
Explanation: The input binary string 1111111111111111111111111111101 has a total of thirty set bits.
```

### Constraints

- `1 <= n <= 2^31 - 1`

### Follow-up

If this function is called many times, how would you optimize it?

## Solution (Rust)

```rust
pub fn hamming_weight(n: i32) -> i32 {
    // Reinterpret the bits as u32 so shifting doesn't hit sign-extension issues
    let bits = n as u32;
    bits.count_ones() as i32
}
```

### Manual bit-shift version (no built-in `count_ones`)

```rust
pub fn hamming_weight(n: i32) -> i32 {
    let bits = n as u32;
    let mut count = 0;
    let mut num = bits;
    while num != 0 {
        count += num & 1;
        num >>= 1;
    }
    count as i32
}
```

### Optimization notes (for the follow-up)

- `count_ones()` compiles to a single hardware `POPCNT` instruction on most modern CPUs, making it O(1) in practice rather than looping over all 32 bits.
- The manual loop can be sped up with **Brian Kernighan's algorithm**, which clears the lowest set bit each iteration (`num &= num - 1`), running in O(k) where k is the number of set bits instead of O(32):

```rust
pub fn hamming_weight(n: i32) -> i32 {
    let mut num = n as u32;
    let mut count = 0;
    while num != 0 {
        num &= num - 1; // clears the lowest set bit
        count += 1;
    }
    count
}
```

- If the function is called on the same values repeatedly, a lookup table (e.g. precomputed counts for all bytes 0–255, then summing 4 byte-lookups per `i32`) trades memory for speed.