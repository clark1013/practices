package leetcode

import "testing"

func Test_searchFirstPos(t *testing.T) {
	type args struct {
		nums   []int
		target int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		// TODO: Add test cases.
		{"basic1", args{nums: []int{5, 7, 7, 8, 8, 10}, target: 8}, 3},
		{"basic2", args{nums: []int{5, 7, 7, 8, 8, 10}, target: 10}, 5},
		{"notin", args{nums: []int{5, 7, 7, 8, 8, 10}, target: 11}, -1},
		{"empty", args{nums: []int{}, target: 8}, -1},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := searchFirstPos(tt.args.nums, tt.args.target); got != tt.want {
				t.Errorf("searchFirstPos() = %v, want %v", got, tt.want)
			}
		})
	}
}
