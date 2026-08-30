# 190. Reverse Bits

**Difficulty:** Easy
**Topics:** Bit Manipulation
**Companies:** Premium 🔒

## Problem

Reverse bits of a given 32 bits signed integer.

## Examples

### Example 1

```
Input:  n = 43261596
Output: 964176192
```

| Integer   | Binary                             |
|-----------|-------------------------------------|
| 43261596  | 00000010100101000001111010011100    |
| 964176192 | 00111001011110000010100101000000    |

### Example 2

```
Input:  n = 2147483644
Output: 1073741822
```

| Integer    | Binary                             |
|------------|-------------------------------------|
| 2147483644 | 01111111111111111111111111111100    |
| 1073741822 | 00111111111111111111111111111110    |

## Constraints

- `0 <= n <= 2^31 - 2`
- `n` is even.

## Follow-up

If this function is called many times, how would you optimize it?

> 💡 Hint: Cache results for byte-sized chunks (8 bits) and reuse them to reverse the 32-bit integer in 4 lookups instead of processing bit-by-bit each time.

## Approach

1. **Bit-by-bit reversal** — iterate through all 32 bits, shifting the result left and appending the least significant bit of `n` each time.
2. **Byte-lookup optimization** — precompute the reversed value for every possible byte (0–255), then reverse the input 8 bits at a time and combine using shifts.

## Complexity

| Approach          | Time  | Space |
|-------------------|-------|-------|
| Bit-by-bit        | O(1)  | O(1)  |
| Byte lookup table | O(1)  | O(1) (cache reused across calls) |
<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->
