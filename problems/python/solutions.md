# Python Solutions

- [Python Solutions](#python-solutions)
	- [Buy Sell](#buy-sell)
		- [My Answer](#my-answer)
		- [Best Solution](#best-solution)
	- [Buy Sell 2](#buy-sell-2)
		- [My Answer](#my-answer-1)
		- [Best Solution](#best-solution-1)
	- [Majority Element](#majority-element)
		- [My Answer](#my-answer-2)
		- [Best Solution](#best-solution-2)
	- [Merge Sort 1](#merge-sort-1)
		- [My Answer](#my-answer-3)
	- [Merge Sort 2](#merge-sort-2)
		- [My Answer](#my-answer-4)
		- [Best Solution](#best-solution-3)
	- [Remove Duplicate](#remove-duplicate)
		- [My Answer](#my-answer-5)
	- [Remove Element](#remove-element)
		- [My Answer](#my-answer-6)
	- [Rotate Array](#rotate-array)
		- [My Answer](#my-answer-7)
		- [Best Answer](#best-answer)
	- [Search Insert Position](#search-insert-position)
		- [My Answer](#my-answer-8)
		- [Best Solution](#best-solution-4)
	- [Integer Break](#integer-break)
		- [My Answer](#my-answer-9)
		- [Best Solution](#best-solution-5)
	- [Game of Life](#game-of-life)
		- [My Answer](#my-answer-10)
	- [Rotate Image](#rotate-image)
		- [My Answer](#my-answer-11)
		- [Best Solution](#best-solution-6)


## Buy Sell
**Question:** You are given an array prices where prices[i] is the price of a given stock on the ith day.
You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock. Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

### My Answer
```python
import json
large_price_sample = json.load(open('data/prices.json'))


class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        n = len(prices)
        buy = prices[0]
        max_profit = 0
        for i in range(1, n):
            # Checking for lower buy value
            if (buy > prices[i]):
                buy = prices[i]
            # Checking for higher profit
            elif (prices[i] - buy > max_profit):
                max_profit = prices[i] - buy
        return max_profit
```

### Best Solution
```python
class Solution:
    def maxProfit(self, prices):
        buy = prices[0]
        profit = 0
        for i in range(1, len(prices)):
            if prices[i] < buy:
                buy = prices[i]
            elif prices[i] - buy > profit:
                profit = prices[i] - buy
        return profit
```


## Buy Sell 2
**Question:** You are given an integer array prices where prices[i] is the price of a given stock on the ith day. On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day. Find and return the maximum profit you can achieve.

### My Answer
```python
class Solution:
    def maxProfit(self, prices):
        buy = prices[0] # low
        profit = 0
        
        for i in range(1, len(prices)):
            if prices[i] < buy:
                # If today price is lower than prev, buy today
                buy = prices[i]
            elif prices[i] - buy > 0:
                # Check if selling today would yield a profit greater than 0
                sell = prices[i] # high
                profit += sell - buy
                buy = prices[i] # Set buy to today
    
        return profit
```

### Best Solution
```python
class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        max = 0
        start = prices[0]
        len1 = len(prices)
        for i in range(0 , len1):
            if start < prices[i]: 
                max += prices[i] - start
            start = prices[i]
        return max
```

## Majority Element
**Question:** Given an array nums of size n, return the majority element. The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

### My Answer
```python
class Solution(object):
    def majorityElement(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        m = len(nums) / 2
        for n in nums:
            if (nums.count(n) > m): return n
```

### Best Solution
```python
class Solution:
    def majorityElement(self, nums) -> int:
        nums.sort() # increasing order
        n = len(nums) # 7
        return nums[n//2] # nums[3]
```

## Merge Sort 1
**Question:** You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively. Merge nums1 and nums2 into a single array sorted in non-decreasing order. The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

### My Answer
```python
def merge(nums1, m, nums2, n):
    """
    :type nums1: List[int]
    :type m: int
    :type nums2: List[int]
    :type n: int
    :rtype: None Do not return anything, modify nums1 in-place instead.
    """

    nums1 = [i for i in nums1 + nums2 if i != 0].sort()
```

## Merge Sort 2
**Question:** You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively. Merge nums1 and nums2 into a single array sorted in non-decreasing order. The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

### My Answer
```python
class Solution(object):
    def merge(self, nums1, m, nums2, n):
        """
        :type nums1: List[int]
        :type m: int
        :type nums2: List[int]
        :type n: int
        :rtype: None Do not return anything, modify nums1 in-place instead.
        """
        del nums1[m:]
        nums1 += nums2[:n]
        nums1.sort()
```

### Best Solution
```python
class Solution(object):
    def merge(self, nums1, m, nums2, n):
      for j in range(n):
          nums1[m+j] = nums2[j]
      nums1.sort()
```

## Remove Duplicate
**Question:** Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums. Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things: Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
Return k.

### My Answer
```python
class Solution(object):
    def removeDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        temp = []

        for i, n in enumerate(nums):
            if i == 0: continue
            if n == nums[i - 1]:
                temp.append(n)
        
        for t in temp:
            nums.remove(t)

        return len(nums)
```

## Remove Element
**Question:** Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val. Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things: Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.

### My Answer
```python
def remove_element(nums, val):
    """
    
    :type nums: List[int]
    :type val: int
    :rtype: int
    """
    i = 0
    for x in nums:
        if x != val:
            nums[i] = x
            i += 1
    return i
```

## Rotate Array
**Question:** Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

### My Answer
```python
class Solution(object):
    def rotate(self, nums, k):
        """
        :type nums: List[int]
        :type k: int
        :rtype: None Do not return anything, modify nums in-place instead.
        """
        count = len(nums)
        if count < 2: return 

        if count == 2:
            # Sets of 2 are easy, they can just be reversed for every count of k
            # There's probably a more elegant way to handle this, but I have other things to do. 
            for _ in range(k):
                nums.reverse()
            return 
        
        # Let's keep appending everything on the left side of k to the end of the array until k runs out
        while k > 0:
            left = count - k if count > k else count # Can't splice with a number higher than count
            left_array = nums[:left]
            nums += left_array # Move left segment to end of array
            del nums[:left] # Delete left segment
            k_subtract = k if count > k else count
            k = k - k_subtract # Reduce k until we hit 0
```

### Best Answer
```python
class Solution(object):
   def rotate(self, nums, k):
       # Get the actual number of rotations
       k = k % len(nums)      
       # Get the number of elements to move from the end to the beginning
       r = len(nums) - k
       # Store the elements to move
       new = nums[0:r]
       # Remove the elements from the beginning
       nums[0:r] = []
       # Append the stored elements to the end
       nums.extend(new)
```

## Search Insert Position
**Question:** Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order. You must write an algorithm with O(log n) runtime complexity.

### My Answer
```python
class Solution(object):
	def searchInsert(self, nums, target):
		"""
		:type nums: List[int]
		:type target: int
		:rtype: int
		"""
		try:
			return nums.index(target)
		except ValueError:
			nums.append(target)
			nums.sort()
			return nums.index(target)
```

### Best Solution
```python
```python
class Solution(object):
    def searchInsert(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: int
        """
        index = -1
        for i in range(0,len(nums)):
            if target==nums[i]:
                index = i
        
        if index==-1 and target <nums[len(nums)-1]:
            for i in range(0,len(nums)):
                if target<nums[i]:
                    index = i
                    break
        elif index==-1 and target > nums[len(nums)-1]:
            index = len(nums)


        return index  
```

## Integer Break
**Question:** Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers. Return the maximum product you can get

### My Answer
```python
import numpy as np

def sieve_of_eratosthenes(n):
    # Find all prime numbers of an integer
    primes = [True] * (n + 1)
    p = 2
    while (p**2 <= n):
        # If prime[p] is not changed, then it's a prime
        if primes[p]:
            # Mark all multiples of p as not prime
            for i in range(p**2, n + 1, p):
                primes[i] = False
        p += 1

    prime_numbers = []
    for i in range(2, n + 1):
        if primes[i]:
            prime_numbers.append(i)

    return prime_numbers


def maximum_factors(n):
    # Finding the largest factors from addition permutations of an integer
    if n == 2: return 1

    products = []
    maximum_permutations = []
    rng = list(range(1, n))

    for i in rng:
        pmlist = [] # permutation list

        while sum(pmlist) + i < n:
            pmlist.append(i)
        
        remainder = n - sum(pmlist)

        if remainder != 0:
            # If a remainder is one, add to the last item in list
            if remainder == 1 and pmlist[-1] + 1 != n:
                last_item = pmlist.pop()
                pmlist.append(last_item + 1)
            else:
                if (remainder != n):
                    pmlist.append(remainder)

        if pmlist and pmlist not in maximum_permutations:
            maximum_permutations.append(pmlist)

    for pmlist in maximum_permutations:
        print(pmlist)
        prod = np.prod(np.array(pmlist))
        products.append(prod)

    return max(products)


maximum_factors(3)
```

### Best Solution
```python
def integerBreak(n: int) -> int:
    if n <= 3: return n - 1
    if n % 3 == 0:
        return (3 ** (n // 3)) 
    if n % 3 == 1:
        return (3 ** (n // 3 - 1) * 4)
    else:
        return (3 ** (n // 3) * 2)
```

## Game of Life

**Question:** According to [Wikipedia's article](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life): "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."

The board is made up of an `m x n` grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its [eight neighbors](https://en.wikipedia.org/wiki/Moore_neighborhood) (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by over-population.
4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
  
The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the `m x n` grid board, return the next state.

### My Answer
```python
import copy

class Solution(object):
	def game_of_life(self, board):
		"""
		:type board: List[List[int]]
		:rtype: None Do not return anything, modify board in-place instead.
		"""
		grid = copy.deepcopy(board)  

		for row in range(0, len(grid)):
			for col in range(0, len(grid[row])):
				rows = range(max(0, row - 1), min(row + 2, len(grid)))
				cols = range(max(0, col - 1), min(col + 2, len(grid[0])))

				neighbors = [grid[r][c] for r in rows for c in cols] # moore neighborhood
				cell = grid[row][col]
				value = sum(neighbors) - cell

				if cell == 1:
					if (value < 2) or (value > 3):
						board[row][col] = 0
				else:
					if value == 3:
						board[row][col] = 1
```

## Rotate Image
**Question:** You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.

### My Answer
```python
import copy

class Solution(object):
	def rotate(self, matrix):
		"""
		:type matrix: List[List[int]]
		:rtype: None Do not return anything, modify matrix in-place instead.
		"""
		image = copy.deepcopy(matrix)
		for row in range(0, len(image)):
			for index, col in enumerate(reversed(range(0, len(image[row])))):
				matrix[row][index] = image[col][row]
```

### Best Solution
```python
class Solution:
    def rotate(self, matrix):
        matrix[:] = map(list, zip(*matrix[::-1]))
```