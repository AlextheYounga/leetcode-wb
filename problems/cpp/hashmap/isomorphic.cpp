#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

using namespace std; // generally not recommended in larger projects as there could be naming conflicts

/*
Title: Isomorphic Strings
Link: https://leetcode.com/problems/isomorphic-strings/description
Difficulty: Easy

Description:
Given two strings s and t, determine if they are isomorphic.
Two strings s and t are isomorphic if the characters in s can be replaced to get t.
All occurrences of a character must be replaced with another character while preserving the order of characters.
No two characters may map to the same character, but a character may map to itself.


Example 1:
Input: s = "egg", t = "add"
Output: true

Explanation:
The strings s and t can be made identical by:
Mapping 'e' to 'a'.
Mapping 'g' to 'd'.

--

Example 2:
Input: s = "foo", t = "bar"
Output: false

Explanation:
The strings s and t can not be made identical as 'o' needs to be mapped to both 'a' and 'r'.

--

Example 3:
Input: s = "paper", t = "title"
Output: true


Constraints:
1 <= s.length <= 5 * 104
t.length == s.length
s and t consist of any valid ascii character.
*/

class Solution
{
public:
	bool isIsomorphic(string s, string t)
	{
		if (s.size() != t.size())
		{
			return false;
		}

		std::unordered_map<char, char> charMap;

		for (size_t i = 0; i < s.length(); ++i)
		{
			// Check if a value exists in the map by iterating over it
			if (charMap.size() > 0)
			{
				if (charMap.find(s[i]) != charMap.end())
				{ // Check if key exists in the map.
					if (charMap[s[i]] != t[i]) { return false; }
				};
				for (const auto &pair : charMap)
				{ // Check if value exists in the map.
					if (pair.second == t[i] && pair.first != s[i]) { return false; }
				};
			};

			charMap[s[i]] = t[i];
		}
		return true;
	}
};

int main()
{
	Solution solution;
	vector<vector<string>> samples = {
		{"egg", "add"},				  // true
		{"foo", "bar"},				  // false
		{"paper", "title"},			  // true
		{"iijjsndgls", "ppssmuqzam"}, // true
		{"badc", "baba"},			  // false
		{"bookkeeper", "keyboard"}, // false
		{"abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"} // true
	};

	for (size_t i = 0; i < samples.size(); ++i)
	{
		vector<string> sample = samples[i];
		string s = sample[0];
		string t = sample[1];
		bool result = solution.isIsomorphic(s, t);
		std::cout << result << endl;
	};

	return 0;
}