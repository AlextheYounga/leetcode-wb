#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>
#include <sstream>  // For std::stringstream

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


// Beats 100% !!
class Solution
{
public:
	bool isSubsequence(string s, string t)
	{
		// Check and see if s is inside t
		if (t.find(s) != string::npos) { return true; };

		// Lambda function to check if a character is neither a digit nor alphabetic
		t.erase(std::remove_if(t.begin(), t.end(), [s](char c) {
			return s.find(c) == string::npos;  // Keep only digits and alphabetic characters
		}), t.end());

		// Run some quick checks
		if (t.find(s) != string::npos) { return true; }
		if (s.length() > t.length()) { return false; }

		for (size_t i = 0; i < s.length(); ++i)
		{
			if (s[i] != t[i]) {
				t.erase(i, 1);
			}
		}

		return s == t;
	};
};

string readFromFile(string filename) {
    std::ifstream file(filename);  // Open the file
    if (!file) {  // Check if the file was opened successfully
        std::cout << "Error opening file!" << std::endl;
        exit(1);
    }

   	std::stringstream buffer;  // Create a string stream
    buffer << file.rdbuf();    // Read the file contents into the buffer
    std::string content = buffer.str();  // Convert buffer to a string/ Close the file

    return content;
}

int main()
{
	Solution solution;
	vector<vector<string>> samples = {
		{"abc", "ahbgdc"}, // true
		{"axc", "ahbgdc"}, // false
		{"acb", "ahbgdc"}, // false
		{"aaaaaa", "bbaaaa"}, // false
		{"ab", "baab"}, // true
		{"leeeeetcode", readFromFile("./data/is_subsequence_heap_1.txt")}, // true
		{"rjufvjafbxnbgriwgokdgqdqewn", readFromFile("./data/is_subsequence_heap_2.txt")} // false
	};

	for (size_t i = 0; i < samples.size(); ++i)
	{
		vector<string> sample = samples[i];
		string s = sample[0];
		string t = sample[1];
		bool result = solution.isSubsequence(s, t);
		std::cout << result << endl;
	};

	return 0;
}