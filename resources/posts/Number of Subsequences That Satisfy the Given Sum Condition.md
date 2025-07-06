---
note_start: 2025-06-29
type: algorithms_practice
url: https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/description
difficulty: medium
status: solved
topics:
  - "[[Subsequence]]"
  - "[[Binary search]]"
---
# Description
You are given an array of integers `nums` and an integer `target`. Return _the number of **non-empty** subsequences of_ `nums` _such that the sum of the minimum and maximum element on it is less or equal to_ `target`. Since the answer may be too large, return it **modulo** `10^9 + 7`.
# Comments
- large constraints $len(nums) \le 10^5$, $nums[i],\text{target} \le 10^6$
- topic includes sorting and binary search, no [[Dynamic programming]] possible?
- examples are all sorted - peaked comment: **sorting is fine because it does not affect the min & max of the subsequences** - same number of subsequences with min + max smaller than target in the sorted array
	- $O(n^2)$ solution - [[Two pointers]], unidirectional - fastest? - **how would I know there's no better solution?** 
	- binary search the limit for each? - $O(nlog(n))$ - passes - but still slow
	- [[Two pointers]] Inwards also works with the sorted array, since if the condition doesn't hold for right (`nums[right] + nums[left] <= target` , it wouldn't hold for right + 1 either as right +1 is guaranteed to be bigger - $O(n)$
- `[2,3,3,4,6,7]`, `[0,1,2,3,4,5]`
# Solution
$O(n^2) solution$ (time limit exceeded):
```Python
class Solution:
    def numSubseq(self, nums: List[int], target: int) -> int:
        nums.sort()
        subs = 0
        for i in range(len(nums)):
            for j in range(i, len(nums)):
                if nums[i] + nums[j] <= target: 
                    subs += 2**(max(j-i - 1,0))
        return subs%(10**9 + 7)
```
$O(nlog(n))$ solution:
```python
import bisect
MOD = 10**9 + 7
class Solution:
    def numSubseq(self, nums: List[int], target: int) -> int:
        nums.sort()
        subs = 0
        for i in range(len(nums)):
            right_target = target - nums[i]
            j_lim = bisect.bisect_right(nums,right_target) - 1

            if j_lim >= i and nums[i] + nums[j_lim] <= target:
                subs += (2**(max(j_lim - i, 0)) % MOD)
                
        return subs % MOD
```
$O(n)$ solution:
```python
class Solution:
    def numSubseq(self, nums: List[int], target: int) -> int:
        nums.sort()
        res = 0
        MOD = 10**9 + 7
        end = len(nums)-1
        start = 0
        while start<=end:
            if nums[start]+nums[end]>target:
                end-=1
            else:
                res+=2**(end-start)
                res = res%MOD
                start+=1
        return res
```