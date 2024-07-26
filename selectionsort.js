function swap(arr, i, j) {
  [arr[i], arr[j]] = [arr[j], arr[i]]
}

function selectionsort(arr) {
  for (let i = 0; i < arr.length; i++) {
    // arr.length - 1 - i -> (arr.length - 1) for prevent index error, (arr.length - 1 - i) for not loop in last elements sorted
    for (let j = 0; j < arr.length - 1 - i; j++) {
      if (arr[j] > arr[j + 1]) {
        swap(arr, j, j + 1)
      }
    }
  }
  return arr
}

module.exports = selectionsort
