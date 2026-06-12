# Ch6 JavaScript 練習 — Callback、陣列、閉包

## 1.js — Callback 回呼函數

把一個函式當參數傳給另一個函式，讓它決定什麼時候叫這個函式。

```js
function mathTool(num1, num2, action) {
  return action(num1, num2);
}
let resultAdd  = mathTool(10, 5, (a, b) => a + b);
let resultSub  = mathTool(10, 5, (a, b) => a - b);
console.log(resultAdd);
console.log(resultSub);
```

```
> 15
> 5
```

- `function mathTool(num1, num2, action)` — 宣告 mathTool，接收兩個數字和一個回呼函式 action
- `return action(num1, num2)` — 在 mathTool 內部呼叫 action，把 num1、num2 傳進去並回傳結果
- `(a, b) => a + b` — 傳入加法邏輯，計算 10 + 5
- `(a, b) => a - b` — 傳入減法邏輯，計算 10 - 5
- `console.log(resultAdd)` — 印出 15
- `console.log(resultSub)` — 印出 5

---

## 2.js — IIFE 立即執行函式

函式寫完立刻執行，裡面的變數不會跑出去干擾外面的程式。

```js
(function() {
  let count = 100;
  console.log("Count is: " + count);
})();
```

```
> Count is: 100
```

- `(function() { ... })()` — 定義一個匿名函式並立刻執行
- `let count = 100` — 在 IIFE 內部宣告變數 count，外部無法存取
- `console.log("Count is: " + count)` — 印出 Count is: 100

---

## 3.js — map() 折扣計算

用 map 把陣列每個數字都乘以 0.8，產生一個新陣列。

```js
const prices = [100, 200, 300, 400];
const discounted = prices.map(price => price * 0.8);
console.log(discounted);
```

```
> [80, 160, 240, 320]
```

- `const prices = [100, 200, 300, 400]` — 原始價格陣列
- `prices.map(price => price * 0.8)` — 每個元素乘以 0.8，回傳新陣列
- `console.log(discounted)` — 印出 [80, 160, 240, 320]

---

## 4.js — 陣列變異 (pop / unshift)

pop 砍掉最後一筆，unshift 在開頭加一筆，原陣列會被改到。

```js
function cleanData(arr) {
  arr.pop();
  arr.unshift("Start");
}
let myData = [1, 2, 3];
cleanData(myData);
console.log(myData);
```

```
> ["Start", 1, 2]
```

- `function cleanData(arr)` — 宣告 cleanData，接收一個陣列參數
- `arr.pop()` — 移除陣列最後一個元素（3）
- `arr.unshift("Start")` — 在陣列開頭插入 "Start"
- `let myData = [1, 2, 3]` — 宣告原陣列
- `cleanData(myData)` — 呼叫函式，陣列以參考傳入，myData 會被修改
- `console.log(myData)` — 印出 ["Start", 1, 2]

---

## 5.js — Closure 閉包

外面函式設一個倍數，回傳的裡面包的那個函式會記住這個倍數。

```js
function multiplier(factor) {
  return (n) => n * factor;
}
const double = multiplier(2);
console.log(double(10));
```

```
> 20
```

- `function multiplier(factor)` — 宣告 multiplier，接收 factor 參數
- `return (n) => n * factor` — 回傳一個箭頭函式，它記住了 factor 的值
- `const double = multiplier(2)` — 呼叫 multiplier(2)，double 被賦值為「乘 2」的函式
- `console.log(double(10))` — 執行 double(10)，即 10 * 2，印出 20

---

## 6.js — 自訂 filter 回呼

自己寫一個篩選功能，給一個條件函式，符合的才收進結果陣列。

```js
function myFilter(arr, callback) {
  let result = [];
  for (let i = 0; i < arr.length; i++) {
    if (callback(arr[i])) {
      result.push(arr[i]);
    }
  }
  return result;
}
let data = [1, 5, 8, 12];
let filtered = myFilter(data, function(item) {
  return item > 7;
});
console.log(filtered);
```

```
> [8, 12]
```

- `function myFilter(arr, callback)` — 自訂篩選函式，接收陣列和回呼
- `let result = []` — 建立空陣列準備存放結果
- `for (let i = 0; i < arr.length; i++)` — 遍歷整個陣列
- `if (callback(arr[i]))` — 把每個元素丟給 callback 檢驗
- `result.push(arr[i])` — 符合條件就加入 result
- `return result` — 回傳篩選後的陣列
- `let data = [1, 5, 8, 12]` — 原始資料
- `function(item) { return item > 7 }` — 回呼函式，條件為大於 7
- `console.log(filtered)` — 印出 [8, 12]

---

## 7.js — filter() 過濾物件陣列

從一個物件陣列裡篩出 age 大於等於 18 的。

```js
const users = [
  { name: "Alice", age: 25 },
  { name: "Bob", age: 17 }
];
const adults = users.filter(user => user.age >= 18);
console.log(adults);
```

```
> [{ name: "Alice", age: 25 }]
```

- `const users = [ ... ]` — 使用者陣列，每筆有 name 和 age
- `users.filter(user => user.age >= 18)` — 篩選出 age 大於等於 18 的使用者
- `console.log(adults)` — 印出 [{ name: "Alice", age: 25 }]

---

## 8.js — 傳參考 vs 傳值

a.push 會動到原陣列，b = [100] 不會。

```js
let listA = [1, 2];
let listB = [3, 4];
function process(a, b) {
  a.push(99);
  b = [100];
}
process(listA, listB);
console.log(listA);
console.log(listB);
```

```
> listA → [1, 2, 99]
> listB → [3, 4]
```

- `let listA = [1, 2]` — 宣告陣列 listA
- `let listB = [3, 4]` — 宣告陣列 listB
- `function process(a, b)` — 宣告 process，接收兩個參數
- `a.push(99)` — 修改 a 的內容，因為傳參考會影響原陣列 listA
- `b = [100]` — 將 b 重新指向新陣列，不影響原陣列 listB
- `process(listA, listB)` — 呼叫函式
- `console.log(listA)` — 印出 [1, 2, 99]（被 push 影響）
- `console.log(listB)` — 印出 [3, 4]（不受影響）

---

## 9.js — setTimeout + join

等 2 秒後把陣列用空白黏起來印出來。

```js
const arr = ["Task", "Completed"];
setTimeout(() => {
  console.log(arr.join(" "));
}, 2000);
```

```
> Task Completed   (2 秒後)
```

- `const arr = ["Task", "Completed"]` — 宣告字串陣列
- `setTimeout(() => { ... }, 2000)` — 設定 2 秒後執行箭頭函式
- `arr.join(" ")` — 將陣列以空格連接成 "Task Completed"
- `console.log(...)` — 印出結果

---

## 10.js — reduce() + 折價回呼

把陣列加起來，再把總額丟給折價函式去扣。

```js
function calculateTotal(cart, discountFunc) {
  let sum = cart.reduce((acc, price) => acc + price, 0);
  return discountFunc(sum);
}
const result = calculateTotal([100, 200, 300], function(total) {
  return total - 50;
});
console.log(result);
```

```
> 550   (600 - 50)
```

- `function calculateTotal(cart, discountFunc)` — 宣告計算總額與折扣的函式
- `cart.reduce((acc, price) => acc + price, 0)` — 累加陣列所有價格，起始值 0
- `let sum` — 存加總結果 600
- `return discountFunc(sum)` — 將總額傳給折價函式並回傳結果
- `function(total) { return total - 50 }` — 折價回呼，總額減 50
- `console.log(result)` — 印出 550
