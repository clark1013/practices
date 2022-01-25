package leetcode

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_LongestPalindromic(t *testing.T) {
	res := LongestPalindromic("cbbd")
	assert.Equal(t, "bb", res)

	res = LongestPalindromic("babad")
	assert.Equal(t, "bab", res)
}
