package main

import (
	"testing"

	"github.com/tj/assert"
)

func TestFindStr(t *testing.T) {
	assert.Equal(t, 0, findStr("aaa", "a"))
	assert.Equal(t, 1, findStr("baa", "a"))
	assert.Equal(t, -1, findStr("aaa", "aaaab"))
	assert.Equal(t, 1, findStr("abcd", "bc"))
	assert.Equal(t, 2, findStr("abbc", "bc"))
	assert.Equal(t, 0, findStr("abbc", ""))
}
