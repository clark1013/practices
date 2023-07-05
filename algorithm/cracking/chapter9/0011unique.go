package chapter9

func IsUnique(s string) bool {
	charSet := make([]bool, 128)
	for _, k := range s {
		if charSet[k] {
			return false
		}
		charSet[k] = true
	}
	return true
}
