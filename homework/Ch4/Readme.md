# JavaScript 基礎演算法與資料處理實作

本專案收錄了 10 個核心的 JavaScript 實作範例。本文件不僅提供函式的 I/O 規格，更深入探討演算法複雜度、記憶體參照特性（Pass by Sharing）、以及 JavaScript 引擎在處理 JSON 解析與浮點數運算時的底層邏輯。

## 目錄
1. [圖書館書籍搜尋.js](#1-圖書館書籍搜尋js)
2. [基本條件判斷.js](#2-基本條件判斷js)
3. [尋找特定目標.js](#3-尋找特定目標js)
4. [建立並回傳物件.js](#4-建立並回傳物件js)
5. [成績系統分析.js](#5-成績系統分析js)
6. [提款機餘額模擬.js](#6-提款機餘額模擬js)
7. [物件格式轉換與導出.js](#7-物件格式轉換與導出js)
8. [解析 JSON 並計算總額.js](#8-解析-json-並計算總額js)
9. [過濾物件陣列.js](#9-過濾物件陣列js)
10. [陣列數字加總.js](#10-陣列數字加總js)

---

### 1. 圖書館書籍搜尋.js

*   對應函式: `findBooksByAuthor(lib, authorName)`
*   時間複雜度: $O(N)$，其中 N 為圖書館陣列的長度。
*   空間複雜度: $O(K)$，其中 K 為符合條件的書籍數量。
*   說明: 
    採用線性搜尋 (Linear Search)。物件陣列的儲存實際上是參照 (Reference) 的集合。每次迭代透過 `.` 運算子存取 `author` 屬性時，引擎會進行屬性查找。使用嚴格相等 (`===`) 可避免 JavaScript 隱式型別轉換 (Type Coercion) 帶來的效能耗損與潛在的邏輯錯誤。

```
const library = [
  { title: "Harry Potter", author: "J.K. Rowling" },
  { title: "The Hobbit", author: "J.R.R. Tolkien" },
  { title: "Fantastic Beasts", author: "J.K. Rowling" }
];

function findBooksByAuthor(lib, authorName) {
  let foundTitles = [];
  for (let i = 0; i < lib.length; i++) { 
    if (lib[i].author === authorName) { 
      foundTitles.push(lib[i].title);
    }
  }
  return foundTitles;
}
```
2. 基本條件判斷.js
* 對應函式: checkPass(score)
* 時間複雜度: $O(1)$空間複雜度: $O(1)$
* 說明:常數時間的條件分支 (Conditional Branching)。在底層架構中 if...else 結構會交由 CPU 的分支預測器 (Branch Predictor) 處理。不涉及任何迴圈或記憶體動態配置，所以執行成本極低。

```
function checkPass(score) {
  if (score >= 60) {
    return "及格";
  } else {
    return "不及格";
  }
}
```

3. 尋找特定目標.js
* 對應函式: findTarget(arr, target)
* 時間複雜度: 最佳 $O(1)$，最差 $O(N)$
* 空間複雜度: $O(1)$
* 說明:以 while 迴圈實作的循序搜尋。此函式利用了提早回傳 (Early Return) 機制：一旦命中目標 (arr[i] === target) 便立即將控制權與結果交還給呼叫者（彈出呼叫堆疊 Call Stack），有效減少不必要的迭代次數，提升平均情況下的執行效率。
```
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
```
4. 建立並回傳物件.js
* 對應函式: createUser(name, age, email)
* 時間/空間複雜度: $O(1)$
* 深度解析:實作了基礎的工廠模式 (Factory Pattern)。當執行此函式時， 會在Heap (堆積) 記憶體中動態配置一塊空間來儲存這個新物件。為了優化屬性存取速度，引擎會在背景建立隱藏類別 (Hidden Classes)，將屬性佈局固定下來。
```
function createUser(name, age, email) {
  let user = {
    userName: name,
    userAge: age,
    userEmail: email
  };
  return user;
}
```
5. 成績系統分析.js
* 對應函式: processStudentGrades(jsonStr)
* 時間複雜度: $O(N \times M)$，N 為學生人數，M 為單一學生的分數科目數。
* 空間複雜度: $O(N)$
* 說明:此函式模擬了微型的資料管線 (Pipeline)。
  1. 反序列化 (Deserialize): JSON.parse 是一個高成本的同步操作，它會掃描字串並在記憶體中建立對應的樹狀結構（類似 AST）。
  2. 聚合運算: 透過巢狀迴圈進行資料聚合 (Aggregation)。
  3. 序列化 (Serialize): 將處理完的 JS 物件陣列透過 JSON.stringify 重新轉換為字串。這過程牽涉到記憶體的釋放與新字串的配置。
```
function processStudentGrades(jsonStr) {
  let students = JSON.parse(jsonStr);
  let result = [];
  for (let i = 0; i < students.length; i++) {
    let student = students[i];
    let scores = student.scores;
    
    let totalScore = 0;
    let index = 0;
    while (index < scores.length) {
      totalScore += scores[index];
      index++;
    }
    
    let average = totalScore / scores.length;
    let grade = average >= 80 ? "A" : "B";
    
    result.push({
      name: student.name,
      avgScore: average,
      finalGrade: grade
    });
  }
  return JSON.stringify(result);
}
```

6. 提款機餘額模擬.js
* 對應函式: withdrawUntilEmpty(account, amount)
* 時間複雜度: $O(\lfloor \text{balance} / \text{amount} \rfloor)
* 空間複雜度: $O(1)$
* 說明:此函式展示了 JavaScript 中的 Pass by Sharing (傳址/共享傳遞) 特性。傳入的 account 是一個物件參考，因此迴圈內對 account.balance 的修改會直接產生副作用 (Side Effect)，變更全域/外部記憶體中的原始物件狀態。這與純函式 (Pure Function) 的設計理念不同，在併發或大型系統中需謹慎處理狀態變更。
```
function withdrawUntilEmpty(account, amount) {
  let count = 0;
  while (account.balance >= amount) {
    account.balance -= amount;
    count++;
  }
  console.log(`成功提款 ${count} 次，帳戶剩餘: ${account.balance}`);
  return account;
}
```
7. 物件格式轉換與導出.js
* 對應函式: exportProductData(names, prices)
* 時間複雜度: $O(N)$空間複雜度: $O(N)$ (需配置新陣列儲存物件)
* 說明:實作了類似函數式程式設計中的 Zip 運算。利用共用的索引變數 i，將記憶體中兩個獨立且平行的陣列 (Parallel Arrays) 同步走訪，並將其壓縮封裝為單一物件陣列，解決了原本資料結構鬆散的問題。
```
function exportProductData(names, prices) {
  let products = [];
  for (let i = 0; i < names.length; i++) {
    products.push({
      id: i + 1,
      name: names[i],
      price: prices[i]
    });
  }
  return JSON.stringify(products);
}
```
8. 解析 JSON 並計算總額.js
* 對應函式: calculateTotal(jsonString)
* 時間複雜度: $O(N)$
* 空間複雜度: $O(N)$ JSON parsing 配置的記憶體 說明:注意 IEEE 754 浮點數誤差：JavaScript 的 Number 型別底層全面採用 IEEE 754 雙精度浮點數 (Double-precision 64-bit)。在此計算 price * quantity 的累加過程中，若涉及小數運算，極易產生浮點數精度遺失的誤差。在實際電商系統實作中，通常需將數值先放大為整數運算後再縮小，以避免算術誤差。
```
function calculateTotal(jsonString) {
  const cart = JSON.parse(jsonString); 
  let total = 0;
  for (let i = 0; i < cart.length; i++) {
    total += cart[i].price * cart[i].quantity;
  }
  return total;
}
```
9. 過濾物件陣列.js
* 對應函式: getAdults(people)
* 時間複雜度: $O(N)$空間複雜度: $O(K)$ (K 為成年人數)
* 說明:此實作為原生 Array.prototype.filter() 的底層邏輯重現。以一個獨立的陣列作為緩衝區 (Buffer) 來接收滿足謂詞 (Predicate) age >= 18 的元素參考。這確保了原始陣列 people 的不可變性 (Immutability)。
```
function getAdults(people) {
  let adults = [];
  for (let i = 0; i < people.length; i++) {
    if (people[i].age >= 18) {
      adults.push(people[i]);
    }
  }
  return adults;
}
```
10. 陣列數字加總.js
* 對應函式: sumArray(numbers)
* 時間複雜度: $O(N)$空間複雜度: $O(1)$
* 深度解析:經典的 Accumulator (累加器) 模式。宣告於迴圈外部的 total 變數在每次迭代中更新狀態。此設計思維與原生 Array.prototype.reduce() 以及摺疊運算 (Fold) 的概念完全一致，是用於陣列降維 (從一維陣列收斂至單一純量值) 的最基礎演算法。
```
function sumArray(numbers) {
  let total = 0;
  for (let i = 0; i < numbers.length; i++) {
    total += numbers[i];
  }
  return total;
}
```