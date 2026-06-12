function findTarget(arr, target) {
  let i = 0;
  while (i < arr.length) {
    if (arr[i] === target) {
      return i;
    }
    i++;
  }
  return -1;
}

console.log(findTarget([5, 12, 8, 130, 44], 8)); // 輸出: 2
console.log(findTarget([5, 12, 8], 99)); // 輸出: -1