/*
实现获取 下一个排列 的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列（即，组合出下一个更大的整数）。
如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。
必须 原地 修改，只允许使用额外常数空间。

示例 1：
输入：nums = [1,2,3]
输出：[1,3,2]

示例 2：
输入：nums = [3,2,1]
输出：[1,2,3]

示例 3：
输入：nums = [1,1,5]
输出：[1,5,1]

示例 4：
输入：nums = [1]
输出：[1]


提示：
1 <= nums.length <= 100
0 <= nums[i] <= 100

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/next-permutation
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


[(1),5,4,3,(2)]
[1,5,2,(3),(4)]
[1,2,4,(3),(5)]
[(2),(3),1,0,0]
=> 找到右边有比当前值大的最右值
*/
package leetcode

func NextPermutaion(nums []int) {
	if len(nums) <= 1 {
		return
	}
	lo, hi := findValidRightMost(nums)
	swap(&nums, lo, hi)
	if lo == hi {
		lo = -1
	}
	sort(&nums, lo+1, len(nums))
}

// 选择排序
func sort(nums *[]int, lo, hi int) {
	for i := lo; i < hi; i++ {
		min := i
		for j := i; j < hi; j++ {
			if (*nums)[j] < (*nums)[i] {
				min = j
			}
		}
		swap(nums, i, min)
	}
}

func swap(nums *[]int, i, j int) {
	(*nums)[i], (*nums)[j] = (*nums)[j], (*nums)[i]
}

// 找出最右侧满足右边有比当前值大的索引
func findValidRightMost(nums []int) (lo, hi int) {
	for i, num := range nums {
		for j := i + 1; j < len(nums); j++ {
			if nums[j] > num {
				lo, hi = i, j
				// break
			}
		}
	}
	return
}
