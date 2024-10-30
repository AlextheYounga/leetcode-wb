# Notes

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