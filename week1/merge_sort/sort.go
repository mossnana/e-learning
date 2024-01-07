package main

// กำหนด input
func sort(arr []int) []int {
	l := len(arr)

	// sub-problem operation
	if l == 1 {
		// arr เดียวไม่ต้อง sort
		return arr
	} else if l == 2 {
		// arr 2 ตัว เช็ค มากกว่าน้อยกว่า
		if arr[0] > arr[1] {
			arr[1], arr[0] = arr[0], arr[1]
		}
		return arr
	}

	//  หาค่ากลาง
	mid := l / 2

	// recursive
	left := sort(arr[:mid])
	right := sort(arr[mid:])

	// merge
	return merge(left, right)
}
