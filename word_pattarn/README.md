# 290. Word Pattern

**Difficulty:** Easy  
**Topics:** Hash Table, String, Bijection  

---

## 📝 Problem Statement

Given a `pattern` and a string `s`, find if `s` follows the same pattern.

Here **follow** means a full match, such that there is a **bijection** between a letter in `pattern` and a non-empty word in `s`. Specifically:

1. Each letter in `pattern` maps to exactly one unique word in `s`.
2. Each unique word in `s` maps to exactly one letter in `pattern`.
3. No two letters map to the same word, and no two words map to the same letter.

---

## 💡 Examples

### Example 1
```text
Input: pattern = "abba", s = "dog cat cat dog"
Output: true

Explanation:
The bijection can be established as:
- 'a' maps to "dog"
- 'b' maps to "cat"