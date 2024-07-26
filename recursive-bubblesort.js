/*
 * Recursive bubblesort version lead to stackoverflow
 */

function swap(arr, i, j) {
  [arr[i], arr[j]] = [arr[j], arr[i]]
}

function maxIndex(arr) {
  if (arr.length === 0) throw new Error('array is empty !!!')
  return arr.reduce((p, v, i) => arr[p] > v ? p : i, 0)
}

function bubblesort(arr) {
  if (arr.length === 1) return arr
  const lastIndex = arr.length - 1
  swap(arr, maxIndex(arr), lastIndex)
  return [...bubblesort(arr.slice(0, lastIndex)), arr[lastIndex]]
}

module.exports = bubblesort
