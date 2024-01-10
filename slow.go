package longestpalindromesubstring

import (
	strings
)

func SlowestLongestPalindromeSubstring(S string) {
	s := strings.Split(S, "")
    maxLength, maxLeft, maxRight := 0, 0 ,0

    for i := range s {
        // odd solution
        left, right := i, i
        for left >= 0 && right < len(s) && s[left] == s[right] {
            length := right - left + 1
            if length > maxLength {
                maxLength, maxLeft, maxRight = length, left, right
            }
            left--
            right++
        }

        // even solution
        left, right = i, i+1
        for left >= 0 && right < len(s) && s[left] == s[right] {
            length := right - left + 1
            if length > maxLength {
                maxLength, maxLeft, maxRight = length, left, right
            }
            left--
            right++
        }
    }
    
    return strings.Join(s[maxLeft:maxRight+1], "")
}
