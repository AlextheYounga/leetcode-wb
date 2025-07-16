# 📝 LeetCode Problem‑Solving Cheatsheet (table‑free edition)

A single page of patterns, code templates, and danger signs—no tables, just text.

---

## 1  Greedy sentinels (one pass, O(1) space)

* **When to use:** maximum/minimum difference, peak‑valley profit, widest gap, etc.
* **Core idea:** Track a running *sentinel* (e.g., `min_price`) and update the answer on the fly without rescanning.
* **Template:**

  ```py
  best = arr[0]      # lowest so far
  ans  = 0           # best profit so far
  for x in arr[1:]:
      best = min(best, x)      # keep sentinel updated
      ans  = max(ans, x - best)
  return ans
  ```

---

## 2  Two‑pointer patterns

* Opposite‑ends meet inwards (sorted array target sum, rain‑water trapping).
* Fast/slow runners (cycle detection, middle of linked list).
* Write‑index pointer for in‑place filtering (remove duplicates/elements, partitioning).
* **In‑place merge** – fill `nums1` from the back so you never overwrite unmerged data:

  ```py
  p1, p2, w = m-1, n-1, m+n-1
  while p2 >= 0:
      if p1 >= 0 and nums1[p1] > nums2[p2]:
          nums1[w] = nums1[p1]; p1 -= 1
      else:
          nums1[w] = nums2[p2]; p2 -= 1
      w -= 1
  ```

---

## 3  Sliding window & Kadane (max subarray)

* Grow/shrink a window while maintaining an invariant.
* Classic max‑subarray (Kadane) in O(n) and O(1):

  ```py
  best = cur = nums[0]
  for x in nums[1:]:
      cur  = max(x, cur + x)  # either start fresh or extend
      best = max(best, cur)
  return best
  ```

---

## 4  Constant‑space tricks you should know by heart

* **Rotate array by `k`:** triple‑reverse

  ```py
  k %= n
  nums[:] = nums[::-1]
  nums[:k] = nums[:k][::-1]
  nums[k:] = nums[k:][::-1]
  ```
* **Rotate n×n matrix 90°:** transpose then reverse rows

  ```py
  matrix[:] = [list(r) for r in zip(*matrix[::-1])]
  ```
* **Game of Life in place:** encode next state in the 2nd bit (`board[r][c] |= next<<1`) then right‑shift.
* **Palindrome integer test** without string:

  ```py
  if x < 0 or (x % 10 == 0 and x):
      return False
  rev = 0
  while x > rev:
      rev = rev*10 + x%10
      x //= 10
  return x == rev or x == rev//10
  ```

---

## 5  Boyer–Moore majority vote (O(n) / O(1))

```py
cand = count = 0
for num in nums:
    if count == 0:
        cand = num
    count += 1 if num == cand else -1
return cand
```

---

## 6  Binary search (lower‑bound template)

```py
def lower_bound(lo, hi, pred):
    while lo < hi:
        mid = (lo + hi) // 2
        if pred(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo
```

*Ask yourself*: “Is there a monotonic yes/no question here?” If yes, try BS.

---

## 7  Math shortcuts & modulo wisdom

* **Integer Break:** maximum product comes from cutting into as many 3’s as possible.  One‑liner:

  ```py
  if n <= 3: return n-1
  return 3**(n//3 - (n%3==1)) * (4 if n%3==1 else 2 if n%3==2 else 1)
  ```
* Always shrink cycles: `k %= n` before rotating or stepping.
* Power‑of‑two test: `n & (n-1) == 0`.

---

## 8  Built‑in complexity flash‑cards

* `sort()` – O(n log n)
* `list.remove`, `list.insert`, `count`, `index` – O(n)
* `heapq.heappush` / `heappop` – O(log n)
* `collections.Counter(nums)` construction – O(n)
  *Putting an O(n) helper inside a loop is a classic O(n²) trap.*

---

## 9  Red‑flag anti‑patterns

* Sorting when the problem demands *in‑place* or O(1) extra space.
* Calling `nums.count(x)` or `list.remove(x)` inside a loop.
* Using `copy.deepcopy` on grids/matrices that must be updated in place.
* Shadowing Python built‑ins (`list`, `max`, `min`, `map`).

---

## 10  Loop‑invariant checklist

1. **State** the invariant: what must be true at the top of every iteration?
2. **Preserve** it: does the body keep it true?
3. **Terminate**: when loop ends, does the invariant prove correctness?

---

## 11  Edge‑case quick list

* Empty array / single element.
* All equal values.
* `k = 0`, `k % n == 0`.
* Negative numbers, INT\_MIN/INT\_MAX.
* Already sorted vs reverse sorted inputs.

---

## 12  Deliberate practice routine

1. Write down target **time/space** constraints first.
2. Match a pattern from this sheet.
3. Draft invariant & pseudocode **before** actual code.
4. Code, then test all edge cases.
5. (Optional) Refactor for clarity once AC.

Happy hacking 🚀
