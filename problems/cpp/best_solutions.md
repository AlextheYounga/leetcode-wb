# Best CPP Solutions from LeetCode Users

## IsSubsequence
### My Answer:
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
```
### Best Solution
**Intuition**
The intuition behind this problem is simple: a subsequence retains the relative order of characters, so the task is to check if all characters in string s appear in string t in the same order, but not necessarily consecutively. We can achieve this by iterating through t and checking if each character in s appears in t in the correct order.

**Approach**
We maintain a pointer i to track the position in string s.
Iterate through each character c in string t:
If the current character c in t matches the character at position i in s, increment i to move to the next character in s.
If the pointer i reaches the length of s, it means all characters of s have been found in t in the correct order, so we return true.
If the loop completes and i hasn't reached the length of s, return false, meaning not all characters in s were found in the correct order in t.
Complexity
Time complexity: O(n), where n is the length of string t. We iterate through each character of t exactly once. The comparison with string s takes constant time since we only track the pointer i.

**Space complexity**
O(1), as the space used by the algorithm is constant, regardless of the size of the input strings. We only use a few variables (i) and no additional data structures.

```cpp
class BestSolution {
public:
    bool isSubsequence(string s, string t) {
        int i=0;
        for(auto c: t)
            s[i]==c ? i++:i;
        
        if(i==s.size())
            return true;
    return false;
    }
};
```

## Isomorphic Strings
### My Answer
```cpp
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
```

### Best Solution
**My Notes**
It looks like here he uses two arrays instead of a hashmap, and that's still far more efficient than a hashmap.
My approach took 18ms while his took 0ms. Mine also uses a nested loop, not ideal.

**Approach**
Imagine two secret messages that use different letters for the same words:

Message 1: "apple"
Message 2: "bbnbm"

These messages are like isomorphic strings—they have the same structure, just different codes.

Here's how the code checks if two strings are isomorphic:

Prepare two counting tables:

One for string s (like a tally for "apple")
One for string t (like a tally for "bbnbm")
Measure the length:

If the messages are different lengths, they can't be the same secret message.
Compare each letter, one by one:

If a letter appears for the first time in both strings at the same position (like "a" and "b" both appearing first at position 0), make a tally mark for both strings in their respective tables.
If a letter appears again, check if its previous position matches in both tally tables. If not, the messages have different codes and aren't the same secret message.
Keep going until the end:

If you finish comparing all letters and every letter matched up in both tally tables, the messages use the same code and are isomorphic!
It's like playing a matching game with letters and their positions to see if the two messages have the same secret code!

**Complexity**
**Time complexity:**
O(n)

**Space complexity**:
O(n)

```cpp
class Solution {
public:
    bool isIsomorphic(string s, string t) {
        vector<int> indexS(200, 0); // Stores index of characters in string s
        vector<int> indexT(200, 0); // Stores index of characters in string t
        
        int len = s.length(); // Get the length of both strings
        
        if(len != t.length()) { // If the lengths of the two strings are different, they can't be isomorphic
            return false;
        }
        
        for(int i = 0; i < len; i++) { // Iterate through each character of the strings
            if(indexS[s[i]] != indexT[t[i]]) { // Check if the index of the current character in string s is different from the index of the corresponding character in string t
                return false; // If different, strings are not isomorphic
            }
            
            indexS[s[i]] = i + 1; // updating position of current character
            indexT[t[i]] = i + 1;
        }
        
        return true; // If the loop completes without returning false, strings are isomorphic
    }
};
```