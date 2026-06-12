function checkPass(score) {
  if (score >= 60) {
    return "及格";
  } else {
    return "不及格";
  }
}

console.log(checkPass(75)); // 輸出: "及格"
console.log(checkPass(45)); // 輸出: "不及格"