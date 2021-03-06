/*
给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 target，返回 [-1, -1]。

进阶：

你可以设计并实现时间复杂度为 O(log n) 的算法解决此问题吗？


示例 1：

输入：nums = [5,7,7,8,8,10], target = 8
输出：[3,4]
示例 2：

输入：nums = [5,7,7,8,8,10], target = 6
输出：[-1,-1]
示例 3：

输入：nums = [], target = 0
输出：[-1,-1]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

package leetcode

// [lo, hi)
func searchFirstPos(nums []int, target int) int {
	lo, hi := 0, len(nums)
	for lo < hi {
		mid := (lo + hi) / 2
		if target < nums[mid] {
			hi = mid
		} else if target > nums[mid] {
			lo = mid + 1
		} else {
			if nums[mid-1] < nums[mid] {
				return mid
			}
			hi = mid
		}
	}
	return -1
}
