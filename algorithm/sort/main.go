// Golang 排序示例
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	id  int
	val int
}

type ExampleSlice []Node

func (e ExampleSlice) Len() int           { return len(e) }
func (e ExampleSlice) Less(i, j int) bool { return e[i].val < e[j].val }
func (e ExampleSlice) Swap(i, j int)      { e[i], e[j] = e[j], e[i] }

func (e ExampleSlice) Show() {
	for _, v := range e {
		if v.val == 1 {
			fmt.Printf("%d-%d ", v.val, v.id)
		} else {
			fmt.Print(".")
		}
	}
	fmt.Println()
}

func main() {
	example := make(ExampleSlice, 0, 20)
	for i := 0; i < 20; i++ {
		n := Node{id: i, val: i % 5}
		example = append(example, n)
	}

	var example1 = make(ExampleSlice, 20)
	copy(example1, example)

	// .1-1 ....1-6 ....1-11 ....1-16 ...
	example.Show()
	sort.Sort(example)
	// Sort 不稳定，相同的值无法保持原始顺序
	// ....1-16 1-11 1-6 1-1 ............
	example.Show()
	sort.Stable(example1)
	// Stable 稳定
	// ....1-1 1-6 1-11 1-16 ............
	example1.Show()
}
