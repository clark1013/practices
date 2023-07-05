package chapter9_test

import (
	"cracking/chapter9"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestUnique(t *testing.T) {
	r := chapter9.IsUnique("abc")
	assert.True(t, r)
	r = chapter9.IsUnique("abca")
	assert.False(t, r)
}
