package chapter9

func IsResortedString(s1, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	charSet := make([]int, 128)
	for _, k1 := range s1 {
		charSet[k1] += 1
	}
	for _, k2 := range s2 {
		charSet[k2] -= 1
		if charSet[k2] < 0 {
			return false
		}
	}
	return true
}
