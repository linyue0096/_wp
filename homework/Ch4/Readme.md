# Ch4 JavaScript 練習 — 陣列、物件、JSON、條件判斷

## 1.js — 圖書館書籍搜尋

從圖書館陣列中找出指定作者的所有書名。

```js
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

console.log(findBooksByAuthor(library, "J.K. Rowling"));
```

```
> [ 'Harry Potter', 'Fantastic Beasts' ]
```

- `const library = [...]` — 宣告圖書館陣列，每筆有 title 和 author
- `function findBooksByAuthor(lib, authorName)` — 宣告函式，接收圖書館陣列和作者名
- `let foundTitles = []` — 空陣列準備存放找到的書名
- `for (let i = 0; i < lib.length; i++)` — 逐本檢查
- `if (lib[i].author === authorName)` — 如果作者符合
- `foundTitles.push(lib[i].title)` — 把書名加入結果陣列
- `return foundTitles` — 回傳所有符合的書名

---

## 2.js — 基本條件判斷

根據分數判斷及格或不及格。

```js
function checkPass(score) {
  if (score >= 60) {
    return "及格";
  } else {
    return "不及格";
  }
}

console.log(checkPass(75));
console.log(checkPass(45));
```

```
> 及格
> 不及格
```

- `function checkPass(score)` — 宣告函式，接收分數
- `if (score >= 60)` — 如果分數大於等於 60
- `return "及格"` — 回傳及格
- `else { return "不及格" }` — 否則回傳不及格
- `console.log(checkPass(75))` — 印出 及格
- `console.log(checkPass(45))` — 印出 不及格

---

## 3.js — 尋找特定目標

在陣列中找出目標值的索引位置，找不到回傳 -1。

```js
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

console.log(findTarget([5, 12, 8, 130, 44], 8));
console.log(findTarget([5, 12, 8], 99));
```

```
> 2
> -1
```

- `function findTarget(arr, target)` — 宣告函式，接收陣列和目標值
- `let i = 0` — 設定起始索引
- `while (i < arr.length)` — 當 i 小於陣列長度就繼續找
- `if (arr[i] === target)` — 如果第 i 個元素等於目標
- `return i` — 回傳索引位置
- `i++` — 索引加 1 繼續找下一個
- `return -1` — 找不到就回傳 -1
- `console.log(findTarget([5, 12, 8, 130, 44], 8))` — 8 在索引 2，印出 2
- `console.log(findTarget([5, 12, 8], 99))` — 99 不在陣列中，印出 -1

---

## 4.js — 建立並回傳物件

傳入姓名、年齡、Email，回傳一個使用者物件。

```js
function createUser(name, age, email) {
  let user = {
    userName: name,
    userAge: age,
    userEmail: email
  };
  return user;
}

console.log(createUser("Alice", 25, "alice@example.com"));
```

```
> { userName: 'Alice', userAge: 25, userEmail: 'alice@example.com' }
```

- `function createUser(name, age, email)` — 宣告函式，接收三個參數
- `let user = { userName: name, ... }` — 建立一個物件，把參數分別存入屬性
- `return user` — 回傳這個物件
- `console.log(createUser("Alice", 25, "alice@example.com"))` — 印出使用者物件

---

## 5.js — 成績系統分析

解析 JSON 字串，計算每個學生的平均分數與等第，再轉回 JSON 輸出。

```js
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

    let grade = "";
    if (average >= 80) {
      grade = "A";
    } else {
      grade = "B";
    }

    result.push({
      name: student.name,
      avgScore: average,
      finalGrade: grade
    });
  }

  return JSON.stringify(result);
}

const inputJSON = '[{"name":"David","scores":[85,90,82]},{"name":"Eve","scores":[60,75,70]}]';
const outputJSON = processStudentGrades(inputJSON);
console.log(outputJSON);
```

```
> [{"name":"David","avgScore":85.66666666666667,"finalGrade":"A"},{"name":"Eve","avgScore":68.33333333333333,"finalGrade":"B"}]
```

- `let students = JSON.parse(jsonStr)` — 把 JSON 字串解析成陣列
- `let result = []` — 準備存放結果
- `for (let i = 0; i < students.length; i++)` — 逐個學生處理
- `let scores = student.scores` — 取出該學生的分數陣列
- `while (index < scores.length)` — 用 while 把分數全部加總
- `totalScore += scores[index]` — 累加每一科分數
- `let average = totalScore / scores.length` — 算平均
- `if (average >= 80) { grade = "A" }` — 平均 80 以上拿 A，否則 B
- `result.push({ name, avgScore, finalGrade })` — 把結果物件加入陣列
- `return JSON.stringify(result)` — 把結果陣列轉回 JSON 字串回傳

---

## 6.js — 提款機餘額模擬

從帳戶中持續扣款直到餘額不足，回傳提款次數與剩餘金額。

```js
function withdrawUntilEmpty(account, amount) {
  let count = 0;
  while (account.balance >= amount) {
    account.balance -= amount;
    count++;
  }
  console.log(`成功提款 ${count} 次，帳戶剩餘: ${account.balance}`);
  return account;
}

let myAccount = { owner: "Bob", balance: 1000 };
withdrawUntilEmpty(myAccount, 300);
```

```
> 成功提款 3 次，帳戶剩餘: 100
```

- `function withdrawUntilEmpty(account, amount)` — 宣告函式，接收帳戶物件和每次提款金額
- `let count = 0` — 記錄提款次數
- `while (account.balance >= amount)` — 只要餘額夠就繼續提
- `account.balance -= amount` — 從餘額扣掉金額
- `count++` — 提款次數加 1
- `console.log(...)` — 印出提款次數和剩餘金額
- `let myAccount = { owner: "Bob", balance: 1000 }` — 宣告帳戶，餘額 1000
- `withdrawUntilEmpty(myAccount, 300)` — 每次提 300，最多提 3 次剩 100

---

## 7.js — 物件格式轉換與導出 JSON

將兩個平行陣列（產品名稱、價格）結合成物件陣列，再轉成 JSON 字串。

```js
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

console.log(exportProductData(["Pen", "Notebook"], [15, 50]));
```

```
> [{"id":1,"name":"Pen","price":15},{"id":2,"name":"Notebook","price":50}]
```

- `function exportProductData(names, prices)` — 宣告函式，接收名稱陣列和價格陣列
- `let products = []` — 空陣列準備放轉換後的物件
- `for (let i = 0; i < names.length; i++)` — 用同一個索引走訪兩個陣列
- `products.push({ id: i + 1, name: names[i], price: prices[i] })` — 合併成物件加入陣列
- `return JSON.stringify(products)` — 轉成 JSON 字串回傳

---

## 8.js — 解析 JSON 並計算總價

解析 JSON 購物車字串，計算所有商品的價格 × 數量的總和。

```js
function calculateTotal(jsonString) {
  const cart = JSON.parse(jsonString);
  let total = 0;

  for (let i = 0; i < cart.length; i++) {
    total += cart[i].price * cart[i].quantity;
  }
  return total;
}

const jsonInput = '[{"item":"Apple","price":20,"quantity":3},{"item":"Banana","price":10,"quantity":5}]';
console.log(calculateTotal(jsonInput));
```

```
> 110
```

- `const cart = JSON.parse(jsonString)` — 把 JSON 字串解析成陣列
- `let total = 0` — 總額從 0 開始
- `for (let i = 0; i < cart.length; i++)` — 逐項計算
- `total += cart[i].price * cart[i].quantity` — 各項金額累加
- `return total` — 回傳總額 110（20×3 + 10×5）

---

## 9.js — 過濾物件陣列

從一群人當中篩選出年齡大於等於 18 歲的成年人。

```js
function getAdults(people) {
  let adults = [];
  for (let i = 0; i < people.length; i++) {
    if (people[i].age >= 18) {
      adults.push(people[i]);
    }
  }
  return adults;
}

const users = [
  { name: "Tom", age: 15 },
  { name: "Jerry", age: 20 },
  { name: "Mickey", age: 35 }
];
console.log(getAdults(users));
```

```
> [ { name: 'Jerry', age: 20 }, { name: 'Mickey', age: 35 } ]
```

- `function getAdults(people)` — 宣告函式，接收人員陣列
- `let adults = []` — 空陣列準備放成年人
- `for (let i = 0; i < people.length; i++)` — 逐個檢查
- `if (people[i].age >= 18)` — 如果年齡大於等於 18
- `adults.push(people[i])` — 加入結果陣列
- `return adults` — 回傳篩選後的成年人陣列
- Tom 15 歲未滿 18，被排除；Jerry 20、Mickey 35 符合條件

---

## 10.js — 陣列數字加總

將陣列中所有數字加總後回傳。

```js
function sumArray(numbers) {
  let total = 0;
  for (let i = 0; i < numbers.length; i++) {
    total += numbers[i];
  }
  return total;
}

console.log(sumArray([10, 20, 30]));
```

```
> 60
```

- `function sumArray(numbers)` — 宣告函式，接收數字陣列
- `let total = 0` — 總和從 0 開始
- `for (let i = 0; i < numbers.length; i++)` — 從頭跑到尾
- `total += numbers[i]` — 把每個數字加進 total
- `return total` — 回傳總和 60
