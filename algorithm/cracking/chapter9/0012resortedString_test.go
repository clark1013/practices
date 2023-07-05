package chapter9_test

import (
	"cracking/chapter9"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestResortedString(t *testing.T) {
	r := chapter9.IsResortedString("abc", "cba")
	assert.True(t, r)
	r = chapter9.IsResortedString("abc", "cbb")
	assert.False(t, r)
}
