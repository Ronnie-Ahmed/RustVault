# 205. Isomorphic Strings

**Difficulty:** Easy

**Topic:** Hash Table, String

**LeetCode Link:** [Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/)

---

## 📝 Problem Statement

Given two strings `s` and `t`, determine if they are **isomorphic**.

Two strings `s` and `t` are isomorphic if the characters in `s` can be replaced to get `t`.

All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

---

## 💡 Examples

### Example 1

```text
Input: s = "egg", t = "add"
Output: true
Explanation:
The strings s and t can be made identical by:
- Mapping 'e' to 'a'
- Mapping 'g' to 'd'

```

### Example 2

```text
Input: s = "foo", t = "bar"
Output: false
Explanation:
'o' cannot map to both 'a' and 'r'.

```

### Example 3

```text
Input: s = "paper", t = "title"
Output: true
Explanation:
- Mapping 'p' to 't'
- Mapping 'a' to 'i'
- Mapping 'e' to 'l'
- Mapping 'r' to 'e'

```

---

## 🔒 Constraints

* $1 \le \text{s.length} \le 5 \times 10^4$
* $\text{t.length} == \text{s.length}$
* `s` and `t` consist of any valid ASCII character.

---

## 🧠 Approach & Explanation

To ensure a valid **bijective (one-to-one) mapping**:

1. Every character in `s` must map to exactly one character in `t`.
2. No two distinct characters in `s` can map to the same character in `t`.

We can track this bidirectional mapping using either **two HashMaps** or **fixed-size arrays** (since inputs are ASCII):

* **Map 1 (`s_to_t`):** Tracks character mappings from `s` to `t`.
* **Map 2 (`t_to_s`):** Tracks character mappings from `t` to `s`.

As we iterate through the characters of both strings simultaneously:

* If $s[i]$ is already mapped to a character other than $t[i]$, return `false`.
* If $t[i]$ is already mapped to a character other than $s[i]$, return `false`.
* Otherwise, establish the mapping and continue.

If the loop finishes without conflicts, the strings are isomorphic.

