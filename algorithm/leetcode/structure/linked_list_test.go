package leetcode_test

import (
	structure "leetcode/structure"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_NewLinkedList(t *testing.T) {
	l := structure.NewLinkedList(1, 2, 3)
	r := l.Traverse()
	assert.Equal(t, 3, len(r))
	assert.Equal(t, []int{1, 2, 3}, r)
}
