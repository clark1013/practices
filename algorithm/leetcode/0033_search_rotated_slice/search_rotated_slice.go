/*
整数数组 nums 按升序排列，数组中的值 互不相同 。
在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。
给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。

示例 1：
输入：nums = [4,5,6,7,0,1,2], target = 0
输出：4

示例 2：
输入：nums = [4,5,6,7,0,1,2], target = 3
输出：-1

示例 3：
输入：nums = [1], target = 0
输出：-1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/search-in-rotated-sorted-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


[4,5|6,7,0,1,2]
[4,5,6,7|0,1,2]
[4,5,6,7,0|1,2]
*/
package leetcode

func SearchRortatedSlice(nums []int, target int) int {
	return rSearch(nums, 0, len(nums), target)
}

func rSearch(nums []int, lo, hi, target int) int {
	if lo >= hi {
		return -1
	}

	mid := (lo + hi) / 2
	if nums[mid] > nums[mid+1] {
		// 左右都为升序列
		if target == nums[lo] {
			return lo
		} else if target < nums[lo] {
			return binarySearch(nums, mid+1, hi, target)
		} else {
			return binarySearch(nums, lo+1, mid+1, target)
		}
	} else {
		if nums[mid] < nums[lo] {
			// 左侧为旋转数组，右侧为升序
			if target >= nums[mid+1] && target <= nums[hi] {
				return binarySearch(nums, mid+1, hi, target)
			} else {
				return rSearch(nums, lo, mid, target)
			}
		} else {
			// 左侧为升序，右侧为旋转数组
			if target >= nums[lo] && target <= nums[mid] {
				return binarySearch(nums, lo, mid, target)
			} else {
				return rSearch(nums, mid+1, hi, target)
			}
		}
	}
}

func BinarySearch(nums []int, target int) int {
	return binarySearch(nums, 0, len(nums), target)
}

func binarySearch(nums []int, lo, hi, target int) int {
	if lo >= hi {
		return -1
	}

	mid := (lo + hi) / 2
	if nums[mid] == target {
		return mid
	} else if nums[mid] > target {
		return binarySearch(nums, lo, mid, target)
	} else {
		return binarySearch(nums, mid+1, hi, target)
	}
}
