#include <iostream>
#include <string>
#include <vector>

using namespace std; // generally not recommended in larger projects as there could be naming conflicts

/*
Title: Is Subsequence
Link: https://leetcode.com/problems/is-subsequence/description/
Difficulty: Easy

Description:
Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a new string that is formed from the original string by deleting some (can be none)
of the characters without disturbing the relative positions of the remaining characters.
(i.e., "ace" is a subsequence of "abcde" while "aec" is not).

Example 1:
Input: s = "abc", t = "ahbgdc"
Output: true

Example 2:
Input: s = "axc", t = "ahbgdc"
Output: false

Constraints:
0 <= s.length <= 100
0 <= t.length <= 104
s and t consist only of lowercase English letters.


Follow up:
Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 109, and you want to check one by one to see
if t has its subsequence. In this scenario, how would you change your code?
*/

class Solution
{
public:
	bool isSubsequence(string s, string t)
	{
		// WIP
	};
};

int main()
{
	Solution solution;
	vector<vector<string>> samples = {{"abc", "ahbgdc"}, {"axc", "ahbgdc"}};

	for (size_t i = 0; i < samples.size(); ++i) {
		vector<string> sample = samples[i];
		string s = sample[0];
		string t = sample[1];
		solution.isSubsequence(s, t);
	};

	return 0;
}