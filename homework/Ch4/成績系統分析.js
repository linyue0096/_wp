function processStudentGrades(jsonStr) {
  // 1. 處理 JSON
  let students = JSON.parse(jsonStr);
  let result = []; // 準備存放結果的陣列
  
  // 2. 遍歷學生陣列
  for (let i = 0; i < students.length; i++) {
    let student = students[i];
    let scores = student.scores;
    
    // 使用 while 迴圈算總分
    let totalScore = 0;
    let index = 0;
    while (index < scores.length) {
      totalScore += scores[index];
      index++;
    }
    
    let average = totalScore / scores.length;
    
    // 3. 條件判斷
    let grade = "";
    if (average >= 80) {
      grade = "A";
    } else {
      grade = "B";
    }
    
    // 4. 建立新物件並推入陣列
    result.push({
      name: student.name,
      avgScore: average,
      finalGrade: grade
    });
  }
  
  // 5. 轉回 JSON
  return JSON.stringify(result);
}

const inputJSON = '[{"name":"David","scores":[85,90,82]},{"name":"Eve","scores":[60,75,70]}]';
const outputJSON = processStudentGrades(inputJSON);

console.log(outputJSON);
// 輸出: '[{"name":"David","avgScore":85.66666666666667,"finalGrade":"A"},{"name":"Eve","avgScore":68.33333333333333,"finalGrade":"B"}]'