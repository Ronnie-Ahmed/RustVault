# 232. Implement Queue using Stacks

**Difficulty:** Easy  
**Language:** Rust  
**Topics:** Stack, Design, Queue  

---

## Problem Description

Implement a First-In-First-Out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (`push`, `peek`, `pop`, and `empty`).

Implement the `MyQueue` class:
* `push(int x)`: Pushes element `x` to the back of the queue.
* `pop()`: Removes the element from the front of the queue and returns it.
* `peek()`: Returns the element at the front of the queue.
* `empty()`: Returns `true` if the queue is empty, `false` otherwise.

### Notes:
1. You must use **only** standard operations of a stack, which means only *push to top*, *peek/pop from top*, *size*, and *is empty* operations are valid.
2. Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only standard stack operations.

---

## Examples

### Example 1:

**Input:**
```json
["MyQueue", "push", "push", "peek", "pop", "empty"]
[[], [1], [2], [], [], []]