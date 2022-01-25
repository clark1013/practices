package leetcode

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNextPermutaion(t *testing.T) {
	nums := []int{1, 2, 3}
	NextPermutaion(nums)
	assert.Equal(t, []int{1, 3, 2}, nums)

	nums = []int{1, 2, 4, 3, 5}
	NextPermutaion(nums)
	assert.Equal(t, []int{1, 2, 4, 5, 3}, nums)

	nums = []int{1, 5, 4, 3, 2}
	NextPermutaion(nums)
	assert.Equal(t, []int{2, 1, 3, 4, 5}, nums)
}
