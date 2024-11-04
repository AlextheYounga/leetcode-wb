# Notes

- [Notes](#notes)
	- [Buy Sell 1](#buy-sell-1)
		- [Attempt 1](#attempt-1)
		- [Attempt 2](#attempt-2)
		- [Attempt 3](#attempt-3)
		- [Attempt 4](#attempt-4)
	- [Game of Life](#game-of-life)
		- [Attempt 1](#attempt-1-1)
	- [Binary Tree](#binary-tree)
		- [Example of Binary Tree](#example-of-binary-tree)

## Buy Sell 1
### Attempt 1
```py
# First Attempt
# STATUS: FAILED
# I got time limit exceeded on this one. Apparently it's too computationally inefficient for their large test sample case

import json
large_price_sample = json.load(open('data/prices.json'))

class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        pr = []
        for i,p in enumerate(prices):
            m = max(prices[i:])
            pr.append(m - p)

        return max(pr)

            

# prices = [7,6,4,3,1]
prices = [7,1,5,3,6,4]

# Large sample: 10.3 seconds
solution = Solution()
result = solution.maxProfit(prices)

print(result)
```

### Attempt 2
```py
import json
large_price_sample = json.load(open('data/prices.json'))

# Second Strategy
# STATUS: FAILED

class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        pr = []
        for i,p in enumerate(prices):
            temp = prices[i:]
            temp.sort()
            pr.append(temp[-1] - p)

        return max(pr)

            

# Large sample: 1 minute 33 seconds (wow)
# prices = [7,6,4,3,1]
prices = [7,1,5,3,6,4]
# prices = [1] # handle single list test case
solution = Solution()
result = solution.maxProfit(prices)

print(result)
```

### Attempt 3
```py
# Third Strategy - exceeded time limit
# STATUS: FAILED

import json
large_price_sample = json.load(open('data/prices.json'))

class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        if (len(prices) <= 1): return 0
        pr = []
        
        while len(prices) > 1:
            d = prices.pop(0)
            pr.append(max(prices) - d)

        max_p = max(pr)
        return max_p if max_p > 0 else 0

            
# 8.6 seconds; best yet
# prices = [7,6,4,3,1]
# prices = [7,1,5,3,6,4]
# prices = [1] # handle single list test case
# prices = [2,1,2,1,0,1,2] # expects output of 2
solution = Solution()
result = solution.maxProfit(prices)

print(result)
```

### Attempt 4
```py
# STATUS: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHHHHHHHHHH
# STATUS: FAILED

import json
large_price_sample = json.load(open('data/prices.json'))

class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        while len(prices) > 1:
            h = max(prices)
            hidx = prices.index(h)

            if hidx == 0: 
                prices.pop(0)
                continue

            l = min(prices[:h])
            lidx = prices.index(l)

            print(l, h)

            if hidx > lidx: 
                return h - l
            del prices[hidx]
            
        return 0


def run(set):
    solution = Solution()
    result = solution.maxProfit(set)
    print(result)

# run([7,6,4,3,1]) # => 0
# run([7,1,5,3,6,4]) # => 5
# run([1]) # => 0
# run([2,1,2,1,0,1,2]) #=>  2
run([2,4,1]) #=> 2
# run(large_price_sample) #=> 999

# So many problems here. Abandoning this attempt

```

```py
# STATUS: CRIPPLING DEPRESSION

import json
large_price_sample = json.load(open('data/prices.json'))

class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        while len(prices) > 1:
            l = min(prices)
            lidx = prices.index(l)
            if lidx == (len(prices) - 1):
                prices.pop()
                continue
            h = max(prices[lidx:])
            return h - l
        return 0
            

def run(set):
    solution = Solution()
    result = solution.maxProfit(set)
    print(result)

run([7,6,4,3,1]) # => 0
run([7,1,5,3,6,4]) # => 5
run([1]) # => 0
run([2,1,2,1,0,1,2]) #=>  2
run([2,4,1]) #=> 2
run([3,2,6,5,0,3]) #=> 4
# run(large_price_sample) #=> 999

# HOW TF IS THIS THE HARDEST QUESTION I'VE EVER BEEN ASKED !??!?!?!?
# I am giving up. I am cheating...
```

## Game of Life
Getting the sum value of the moore_neighborhood is actually easier than getting all the neighbor items, because it's quite difficult to remove the current particular value from the list. It is far easier to get all the values including the current value, and then just subtract that particular value from the list. I was not able to get the answer until I settled for that solution. 

This does NOT equal the Moore's Neighborhood.

```python
# This will not sufficiently account for items against corners and walls of the matrix
moore_neighborhood = flat_board[(index - 4):index] + flat_board[index+1:(index+1) + 4]
```

Correct Moore's neighborhood algorithm
```python
def get_moore_neighborhood(self, matrix, row, col):
	rows = len(matrix)
	cols = len(matrix[0])
	neighbors = []

	# Iterate over the 3x3 grid centered at (row, col)
	for i in range(row - 1, row + 2):
		for j in range(col - 1, col + 2):
			# Skip the center element itself
			if (i, j) == (row, col):
				continue
			
			# Check if the neighbor is within the matrix boundaries
			if 0 <= i < rows and 0 <= j < cols:
				neighbors.append(matrix[i][j])

	return neighbors
```

### Attempt 1
```python
	def game_of_life(self, board):
		"""
		:type board: List[List[int]]
		:rtype: None Do not return anything, modify board in-place instead.
		"""
		matrix_length = len(board[0])
		flat_board = [item for sublist in board for item in sublist]

		for index, cell in enumerate(flat_board):
			# Get moore neighborhood
			row = index // matrix_length
			col = index % matrix_length
			moore_neighborhood = self.get_moore_neighborhood(board, row, col)
			value = sum(moore_neighborhood)
			
			if index == 3:
				print(moore_neighborhood, value)

			if cell == 1:
				if (value < 2) or (value > 3):
					board[row][col] = 0
			else:
				if value == 3:
					board[row][col] = 1
```

## Binary Tree
A binary tree has at most two children per node.

### Example of Binary Tree
```py
# Definition for a binary tree node.
class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class BinaryTree:
    def __init__(self, root_value):
        self.root = TreeNode(root_value)

    def insert_left(self, parent_node, value):
        new_node = TreeNode(value)
        if parent_node.left is None:
            parent_node.left = new_node
        else:
            # If left child already exists, push the current left child down
            new_node.left = parent_node.left
            parent_node.left = new_node

    def insert_right(self, parent_node, value):
        new_node = TreeNode(value)
        if parent_node.right is None:
            parent_node.right = new_node
        else:
            # If right child already exists, push the current right child down
            new_node.right = parent_node.right
            parent_node.right = new_node



tree1 = BinaryTree(3)
tree1.insert_left(tree1.root, 9)
tree1.insert_right(tree1.root, 20)
tree1.insert_left(tree1.root.right, 15)
tree1.insert_right(tree1.root.right, 7)

tree2 = BinaryTree(1)
tree2.insert_right(tree1.root, 2)
```