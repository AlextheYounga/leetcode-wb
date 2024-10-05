# C++ Failed Attempts

## IsSubsequence

Attempt 1
```cpp
class Solution
{
public:
	bool isSubsequence(string s, string t)
	{
		vector<size_t> positions;
		for (size_t i = 0; i < s.length(); ++i)
		{
			// Check if the value doesn't occur more times than in the t stack. 
			int t_count = std::count(t.begin(), t.end(), s[i]);
			int s_count = std::count(s.begin(), s.end(), s[i]);

			if (s_count != t_count) {
				return false;
			};

			// Check positions;
			size_t position = t.find(s[i]);
			int max_p = positions.size() > 0 ? *max_element(positions.begin(), positions.end()) : 0;

			// If character is not found or if this character does not have the highest position in the stack.
			if ((position == string::npos) || (max_p > position))
			{
				return false;
			};

			positions.push_back(position);
		}
		return true;
	};
};
```
Attempt 2
```cpp
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
		if (s.length() > t.length()) { return false; }
		if ((s == t) || t.find(s) != string::npos) { return true; };

		vector<size_t> positions;
		for (size_t i = 0; i < s.length(); ++i)
		{
			// Check positions;
			size_t position = t.find(s[i]);
			// std::cout << s[i] << std::endl;
			int max_p = positions.size() > 0 ? *max_element(positions.begin(), positions.end()) : 0;

			// If character does not have highest position in the stack.
			if ((max_p > position)) { return false; };
			positions.push_back(position);
		}

		return true;
	};
};
```