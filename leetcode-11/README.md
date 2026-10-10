# 11. Container With Most Water (Medium)

## 📌 Problem Overview

You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the $i$-th line are $(i, 0)$ and $(i, \text{height}[i])$.

Find two lines that, together with the x-axis, form a container such that the container contains the most water. Return the maximum amount of water a container can store.

> **Note:** You may not slant the container.

### Examples

* **Example 1:**
* **Input:** `height = [1,8,6,2,5,4,8,3,7]`
* **Output:** `49`
* **Explanation:** The max area is formed between index 1 (height 8) and index 8 (height 7). Width = $8 - 1 = 7$, Height = $\min(8, 7) = 7$, Area = $7 \times 7 = 49$.


* **Example 2:**
* **Input:** `height = [1,1]`
* **Output:** `1`




---
