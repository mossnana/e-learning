package main

// กำหนด input
func merge(left, right []int) []int {
	// หา length ของ input
	var lenLeft, lenRight = len(left), len(right)

	// address ตำแหน่งปัจจุบันของ left และ right ในการเช็ค
	var i, j int = 0, 0

	// สร้าง array ใหม่ขนาด i + j
	sorted := make([]int, len(left)+len(right))

	// วนจนกว่า array ใหม่จะหมดที่ใส่
	for index := range sorted {
		// กันการ out of range ของ array
		if i >= lenLeft {
			sorted[index] = right[j]
			j++
			continue
		} else if j >= lenRight {
			sorted[index] = left[i]
			i++
			continue
		}

		// ถ้า right มากกว่า left ให้ใส่ left
		if left[i] < right[j] {
			sorted[index] = left[i]
			i++
		} else { // ถ้า left มากกว่า right ให้ใส่ right
			sorted[index] = right[j]
			j++
		}
	}

	return sorted
}
