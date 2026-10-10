# String to Integer (atoi)

**Difficulty:** Medium  
**Platform:** LeetCode  

## Problem Description

Implement the `myAtoi(string s)` function, which converts a string to a $32$-bit signed integer.

The algorithm for `myAtoi(string s)` is as follows:

1. **Whitespace:** Ignore any leading whitespace (`" "`).
2. **Signedness:** Determine the sign by checking if the next character is `'-'` or `'+'`, assuming positivity if neither is present.
3. **Conversion:** Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, the result is $0$.
4. **Rounding:** If the integer is out of the $32$-bit signed integer range $[-2^{31}, 2^{31}-1]$, round the integer to remain within this range. Specifically:
   - Integers less than $-2^{31}$ should be rounded to $-2^{31}$.
   - Integers greater than $2^{31}-1$ should be rounded to $2^{31}-1$.

Return the integer as the final result.

> **Note:** You must not use any built-in library function that converts a string to a number (for example, `atoi`, `stoi`, `Integer.parseInt`, `int()`, `parseInt`, `Number()`). Perform the conversion manually.

---

## Examples

### Example 1
* **Input:** `s = "42"`
* **Output:** `42`

### Example 2
* **Input:** `s = "   -042"`
* **Output:** `-42`
* **Explanation:** Leading spaces are ignored, the negative sign is captured, and leading zeros are discarded during conversion.

### Example 3
* **Input:** `s = "1337c0d3"`
* **Output:** `1337`
* **Explanation:** Reading stops when the non-digit character `'c'` is encountered.

### Example 4
* **Input:** `s = "0-1"`
* **Output:** `0`

### Example 5
* **Input:** `s = "words and 987"`
* **Output:** `0`
* **Explanation:** Reading stops at the very first non-digit character `'w'`, resulting in $0$ digits read.

---

## Constraints
* $0 \le \text{s.length} \le 200$
* `s` consists of English letters (lower-case and upper-case), digits (`0` - `9`), `' '`, `'+'`, `'-'`, and `'.'`.

---

## Approach & Complexity

1. **Index Tracking:** Iterate through the string using a pointer/index to handle whitespace and optional signs.
2. **Overflow Prevention:** Before multiplying and adding digits to the running total, check against integer bounds ($\lfloor \text{INT\_MAX} / 10 \rfloor$) to prevent integer overflow/underflow.
3. **Time Complexity:** $\mathcal{O}(n)$ where $n$ is the length of the string, since we traverse the string at most once.
4. **Space Complexity:** $\mathcal{O}(1)$ as only a few variables are needed for tracking state, sign, and result.