package leetcode

type ListNode struct {
	val  int
	Next *ListNode
}

func (n *ListNode) Traverse() []int {
	res := make([]int, 0)
	t := n
	for t != nil {
		res = append(res, t.val)
		t = t.Next
	}
	return res
}

func NewLinkedList(vals ...int) *ListNode {
	var head *ListNode
	var pre *ListNode
	for i, val := range vals {
		elem := &ListNode{val: val}
		if i == 0 {
			head = elem
		} else {
			pre.Next = elem
		}
		pre = elem
	}
	return head
}
