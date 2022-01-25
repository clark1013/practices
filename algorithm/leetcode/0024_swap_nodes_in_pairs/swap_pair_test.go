package leetcode

import (
	structure "leetcode/structure"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_SwapPair(t *testing.T) {
	l1 := structure.NewLinkedList(1, 2, 3, 4)
	res := SwapPair(l1)
	assert.Equal(t, []int{2, 1, 4, 3}, res.Traverse())

	l1 = structure.NewLinkedList()
	res = SwapPair(l1)
	assert.Equal(t, []int{}, res.Traverse())

	l1 = structure.NewLinkedList(1)
	res = SwapPair(l1)
	assert.Equal(t, []int{1}, res.Traverse())

	l1 = structure.NewLinkedList(1, 2, 3)
	res = SwapPair(l1)
	assert.Equal(t, []int{2, 1, 3}, res.Traverse())
}
