/*
给你一个字符串 s，找到 s 中最长的回文子串。

示例 1：
输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。

示例 2：
输入：s = "cbbd"
输出："bb"

示例 3：
输入：s = "a"
输出："a"

示例 4：
输入：s = "ac"
输出："a"


提示：

1 <= s.length <= 1000
s 仅由数字和英文字母（大写和/或小写）组成

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-palindromic-substring
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

cbbd
0,0 true
0,1 false
1,1 true
*/
package leetcode

func LongestPalindromic(s string) string {
	// dp[i][j] 表示 i->j 是否为回文字符串
	dp := make([][]bool, len(s))
	for i := 0; i < len(s); i++ {
		dp[i] = make([]bool, len(s))
	}

	var res, ri, rj int
	for j := 0; j < len(s); j++ {
		for i := 0; i <= j; i++ {
			if i == j {
				dp[i][j] = true
			} else if s[i] == s[j] && (j-1 <= i+1 || dp[i+1][j-1]) {
				dp[i][j] = true
			} else {
				dp[i][j] = false
			}

			if dp[i][j] && res < j-i+1 {
				res = j - i + 1
				ri, rj = i, j
			}
		}
	}

	return s[ri : rj+1]
}

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}
