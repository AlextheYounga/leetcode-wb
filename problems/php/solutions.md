# PHP Solutions

## Length of Last Word
**Question:** Given a string s consisting of words and spaces, return the length of the last word in the string.

### My Answer
```php
class Solution {

    /**
     * @param String $s
     * @return Integer
     */
    function lengthOfLastWord($s) {
        $words = explode(" ", trim($s));
        $lastWord = end($words);
        return strlen($lastWord);
    }
}

$samples = [
    "Hello World",
    "   fly me   to   the moon  ",
    "luffy is still joyboy",
];

foreach($samples as $sample) {
    $solution = new Solution;
    $result = $solution->lengthOfLastWord($sample);
    print($result . "\n");
}
```