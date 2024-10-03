# C++ Failed Attempts

## IsSubsequence
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
