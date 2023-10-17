// https://greyireland.gitbook.io/algorithm-pattern/ru-men-pian/quickstart

package main

// 给定一个 haystack 字符串和一个 needle 字符串，
// 在 haystack 字符串中找出 needle 字符串出现的第一个位置 (从 0 开始)。如果不存在，则返回 -1。
func findStr(haystack, needle string) int {
	// p1 := 0
	// p2 := 0
	// for p1 < len(haystack) {
	// 	t1 := p1
	// 	for t1 < len(haystack) && p2 < len(needle) && haystack[t1] == needle[p2] {
	// 		if p2 == len(needle)-1 {
	// 			return t1 - p2
	// 		}
	// 		t1++
	// 		p2++
	// 	}
	// 	p2 = 0
	// 	p1++
	// }
	// return -1
	if len(needle) == 0 {
		return 0
	}
	var i, j int
	for i = 0; i < len(haystack)-len(needle)+1; i++ {
		for j = 0; j < len(needle); j++ {
			if haystack[i+j] != needle[j] {
				break
			}
		}
		if len(needle) == j {
			return i
		}
	}
	return -1
}

// 给定一组不含重复元素的整数数组 nums，返回该数组所有可能的子集（幂集）
// 回溯法
// result = []
// func backtrack(选择列表,路径):
//
//	if 满足结束条件:
//	    result.add(路径)
//	    return
//	for 选择 in 选择列表:
//	    做选择
//	    backtrack(选择列表,路径)
//	    撤销选择
func subsets(nums []int) [][]int {
	result := make([][]int, 0)
	list := make([]int, 0)
	backtrack(nums, 0, list, &result)
	return result
}

func backtrack(nums []int, pos int, list []int, result *[][]int) {
	if len(list) > 0 {
		ans := make([]int, len(list))
		copy(ans, list)
		*result = append(*result, ans)
	}
	for i := pos; i < len(nums); i++ {
		list = append(list, nums[i])
		backtrack(nums, i+1, list, result)
		list = list[0 : len(list)-1]
	}
}
