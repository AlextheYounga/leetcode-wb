# Title: Valid Palindrome
# Link: https://leetcode.com/problems/valid-palindrome
# Difficulty: Easy
# 
# Description:
# A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
#
# Given a string s, return true if it is a palindrome, or false otherwise.
#
# Example 1:
#
# Input: s = "A man, a plan, a canal: Panama"
# Output: true
# Explanation: "amanaplanacanalpanama" is a palindrome.
#
# Example 2:
# Input: s = "race a car"
# Output: false
# Explanation: "raceacar" is not a palindrome.
#
# Example 3:
# Input: s = " "
# Output: true
# Explanation: s is an empty string "" after removing non-alphanumeric characters.
# Since an empty string reads the same forward and backward, it is a palindrome.


# @param {String} s
# @return {Boolean}
def is_palindrome(s)
    s.gsub!(/[^a-zA-Z0-9]/, '')
    s.gsub!(/ /, '')
    return s.downcase == s.downcase.reverse
end

def main()
    samples = [
        'A man, a plan, a canal: Panama', # true
        'race a car', # false
        ' ', # true
        '0P' # false
    ]

    for sample in samples do
        result = is_palindrome(sample)
        puts result
    end
end

main()