# Ruby Solutions

- [Ruby Solutions](#ruby-solutions)
	- [Reverse Words](#reverse-words)
		- [My Answer](#my-answer)
	- [Valid Palendrome](#valid-palendrome)
		- [My Answer](#my-answer-1)


## Reverse Words
**Question:** Given an input string s, reverse the order of the words.

### My Answer
```python
# @param {String} s
# @return {String}
def reverse_words(s)
    words = s.gsub(/\s+/m, ' ').strip.split(" ")
    words.reverse.join(' ')
end
```


## Valid Palendrome
**Question:** A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

### My Answer
```python
# @param {String} s
# @return {Boolean}
def is_palindrome(s)
    s.gsub!(/[^a-zA-Z0-9]/, '')
    s.gsub!(/ /, '')
    return s.downcase == s.downcase.reverse
end
```

