---
note_start: 2025-07-05
type: algorithms_practice
url: https://leetcode.com/problems/find-lucky-integer-in-an-array/description/?envType=daily-question&envId=2025-07-05
difficulty: easy
status: solved
topics: 
tags:
  - "#template"
---
# Description
# Comments
- 
# Solution
```python
class Solution:
    def findLucky(self, arr: List[int]) -> int:
        ap = {}
        for i in arr:
            if i not in ap:
                ap[i] = 1
            else:
                ap[i]+=1
        l = -1
        ma = -1
        for v, a in ap.items():
            if v == a and l < v:
                l = v
                ma = a
        return l
```