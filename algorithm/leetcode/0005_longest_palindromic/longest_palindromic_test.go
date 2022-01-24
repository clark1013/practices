package leetcode_test

import (
	leetcode "leetcode/0005_longest_palindromic"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_LongestPalindromic(t *testing.T) {
	res := leetcode.LongestPalindromic("cbbd")
	assert.Equal(t, "bb", res)

	res = leetcode.LongestPalindromic("babad")
	assert.Equal(t, "bab", res)
}
