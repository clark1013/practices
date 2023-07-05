package chapter9

func Rotate(matrix [][]int) [][]int {
	n := len(matrix)
	result := make([][]int, 0, n)
	for k := 0; k < n; k++ {
		temp := make([]int, n)
		result = append(result, temp)
	}
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			result[j][n-1-i] = matrix[i][j]
		}
	}
	return result
}
