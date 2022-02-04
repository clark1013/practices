package leetcode

import "testing"

func TestSearchRortatedSlice(t *testing.T) {
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
		{"1", args{[]int{4, 5, 6, 7, 0, 1, 2}, 4}, 0},
		{"2", args{[]int{4, 5, 6, 7, 0, 1, 2}, 6}, 2},
		{"3", args{[]int{}, 6}, -1},
		{"4", args{[]int{4, 5, 6, 7, 0, 1, 2}, 1}, 5},
		{"5", args{[]int{4, 5, 6, 7, 0, 1, 2}, 3}, -1},
		{"6", args{[]int{4}, 3}, -1},
		{"7", args{[]int{1, 3}, 3}, 1},
		{"8", args{[]int{3, 1}, 3}, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := SearchRortatedSlice(tt.args.nums, tt.args.target); got != tt.want {
				t.Errorf("SearchRortatedSlice() = %v, want %v", got, tt.want)
			}
		})
	}
}
