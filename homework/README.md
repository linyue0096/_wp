# C:\yy\_wp\homework 作業總覽

## 目錄結構

```
C:\yy\_wp\homework\
├── Ch1\          HTML 個人履歷
├── Ch2\          HTML 會員註冊表單
├── Ch3\          JavaScript 入門
├── Ch4\          JavaScript 基礎演算法（10 題）
├── Ch5\          Node.js 網誌全端專案
├── Ch6\          JavaScript 進階練習（10 題）
└── Ch7\          JavaScript 實戰練習（10 題）
```

---

# Ch1 — HTML 個人履歷

**檔案**：`Ch1.html`

## 說明

個人簡歷網頁，使用 Flexbox 實作雙欄式排版。左欄放置個人基本資訊，右欄放置經歷與社團活動。

### 左欄內容

```
大頭照（圓形）
姓名：陳宜昀
英文名：Chen Yi-Yun
─────────────────────
聯絡資訊：手機 0979-733-685 / Email / 地址
教育背景：金門大學 資訊工程學系 → 港明高中 → 港明國中 → 安慶國小
能力特質：自主學習、實作解決問題、軟硬體整合、團隊合作
專業技能：C/C++、Python、嵌入式系統、Linux/WSL
```

### 右欄內容

```
學習服務：校慶志工、自治大隊、英文小老師、童軍志工
幹部經歷：輔導股長（高一上/下）→ 圖書股長（高二上）→ 英文小老師（高二下）→ 活動組長（社團）
社團參與：童軍社（高一/高二）
證照檢定：全民中文檢定（中等/中高等）
```

## 關鍵 CSS 技術

| 技術 | 說明 |
|------|------|
| `display: flex` | 左右兩欄並排 |
| `border-radius: 50%` | 大頭照圓形裁切 |
| `.skill-tag` | 圓角標籤 `border-radius: 20px` |
| `@media (max-width: 768px)` | 小螢幕自動切換單欄 |

---

# Ch2 — HTML 會員註冊表單

**檔案**：`register.html`

## 說明

深色主題的會員註冊頁面。涵蓋多種 HTML 表單輸入元件，搭配自訂 CSS 樣式與少量 JavaScript 即時互動。

### 表單欄位

```
姓名            input[type=text]
電子郵件        input[type=email]
密碼            input[type=password]
確認密碼        input[type=password]
電話            input[type=tel]
出生日期        input[type=date]
地址            input[type=text]
性別            input[type=radio]   男 / 女 / 其他
興趣            input[type=checkbox] 閱讀/音樂/運動/旅遊/遊戲
國家            select               台灣/香港/日本/韓國/新加坡/馬來西亞/其他
年齡            input[type=range]   1~100 滑桿
喜歡的顏色      input[type=color]
自我介紹        textarea
上傳頭像        input[type=file]    虛線拖曳區
```

### JavaScript 互動

```javascript
// 年齡滑桿即時顯示
const ageRange = document.getElementById('ageRange');
const ageValue = document.getElementById('ageValue');
ageRange.addEventListener('input', function() {
    ageValue.textContent = this.value;
});

// 顏色選取即時顯示色碼
const colorPicker = document.getElementById('colorPicker');
const colorValue = document.getElementById('colorValue');
colorPicker.addEventListener('input', function() {
    colorValue.textContent = this.value;
});
```

## CSS 設計重點

```
CSS 變數（--bg-dark, --accent-gold）統一管理色系
appearance: none  自訂 radio / checkbox
linear-gradient   漸層按鈕
border: 2px dashed  虛線上傳區
.form-row grid  雙欄並排
```

---

# Ch3 — JavaScript 入門

**檔案**：`hello.js`

```javascript
console.log('Hello 你好')
```

## 說明

最基本的 JavaScript 程式。`console.log()` 是開發者最常用的除錯工具，會在終端機輸出文字。

## 執行

```
node hello.js
```

## 預期輸出

```
Hello 你好
```

---

# Ch4 — JavaScript 基礎演算法（10 題）

## 4-1 圖書館書籍搜尋.js

```javascript
function findBooksByAuthor(lib, authorName) { ... }
```

### 說明

從圖書館陣列中找出指定作者的所有書籍。使用 `for` 迴圈線性搜尋（Linear Search），對每個物件的 `author` 屬性進行 `===` 嚴格比對。

### 流程

```
library = [
  { title: "Harry Potter",     author: "J.K. Rowling" },
  { title: "The Hobbit",       author: "J.R.R. Tolkien" },
  { title: "Fantastic Beasts", author: "J.K. Rowling" }
]

呼叫 findBooksByAuthor(library, "J.K. Rowling")
  → 第 0 筆：author === "J.K. Rowling" ✅ 加入
  → 第 1 筆：author !== "J.K. Rowling" ❌ 跳過
  → 第 2 筆：author === "J.K. Rowling" ✅ 加入
  → 回傳 ["Harry Potter", "Fantastic Beasts"]
```

### 預期輸出

```
[ 'Harry Potter', 'Fantastic Beasts' ]
```

## 4-2 基本條件判斷.js

```javascript
function checkPass(score) {
    if (score >= 60) { return "及格"; }
    else { return "不及格"; }
}
```

### 說明

根據分數回傳「及格」或「不及格」。60 分為合格門檻。

### 預期輸出

```
checkPass(75) → "及格"
checkPass(45) → "不及格"
```

## 4-3 尋找特定目標.js

```javascript
function findTarget(arr, target) {
    let i = 0;
    while (i < arr.length) {
        if (arr[i] === target) return i;
        i++;
    }
    return -1;
}
```

### 說明

在陣列中搜尋特定值。使用 `while` 迴圈遍歷，找到目標立即回傳索引（Early Return），找不到回傳 -1。

### 預期輸出

```
findTarget([5, 12, 8, 130, 44], 8)  → 2
findTarget([5, 12, 8], 99)          → -1
```

## 4-4 建立並回傳物件.js

```javascript
function createUser(name, age, email) {
    let user = { userName: name, userAge: age, userEmail: email };
    return user;
}
```

### 說明

工廠模式（Factory Pattern）：函式內建立一個新物件並回傳。每次呼叫都在 Heap 記憶體中配置新空間。

### 預期輸出

```
createUser("Alice", 25, "alice@example.com")
→ { userName: 'Alice', userAge: 25, userEmail: 'alice@example.com' }
```

## 4-5 成績系統分析.js

```javascript
function processStudentGrades(jsonStr) {
    let students = JSON.parse(jsonStr);
    let result = [];
    for (let i = 0; i < students.length; i++) {
        let student = students[i];
        // 用 while 算總分
        let totalScore = 0, index = 0;
        while (index < student.scores.length) {
            totalScore += student.scores[index];
            index++;
        }
        let average = totalScore / student.scores.length;
        let grade = average >= 80 ? "A" : "B";
        result.push({ name: student.name, avgScore: average, finalGrade: grade });
    }
    return JSON.stringify(result);
}
```

### 流程

```
JSON 字串 → JSON.parse → 學生陣列
  → 遍歷每位學生
    → while 迴圈加總分數
    → 計算平均
    → 條件判斷等級（80↑為 A）
    → 推入結果陣列
  → JSON.stringify → JSON 字串輸出
```

### 預期輸出

```
processStudentGrades('[{"name":"David","scores":[85,90,82]},{"name":"Eve","scores":[60,75,70]}]')
→ '[{"name":"David","avgScore":85.67,"finalGrade":"A"},{"name":"Eve","avgScore":68.33,"finalGrade":"B"}]'
```

## 4-6 提款機餘額模擬.js

```javascript
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

### 說明

`while` 迴圈持續扣款直到餘額不足。傳入的 `account` 是**物件參考**，直接修改原物件的 `balance`（Side Effect）。

### 預期輸出

```
let myAccount = { owner: "Bob", balance: 1000 };
withdrawUntilEmpty(myAccount, 300);
→ 成功提款 3 次，帳戶剩餘: 100
```

## 4-7 物件格式轉換與導出 JSON.js

```javascript
function exportProductData(names, prices) {
    let products = [];
    for (let i = 0; i < names.length; i++) {
        products.push({ id: i + 1, name: names[i], price: prices[i] });
    }
    return JSON.stringify(products);
}
```

### 說明

將兩個平行陣列（名稱、價格）合併為一個結構化的物件陣列（Zip 運算），並加上自動編號 `id`。

### 預期輸出

```
exportProductData(["Pen", "Notebook"], [15, 50])
→ '[{"id":1,"name":"Pen","price":15},{"id":2,"name":"Notebook","price":50}]'
```

## 4-8 解析 JSON 並計算總價.js

```javascript
function calculateTotal(jsonString) {
    const cart = JSON.parse(jsonString);
    let total = 0;
    for (let i = 0; i < cart.length; i++) {
        total += cart[i].price * cart[i].quantity;
    }
    return total;
}
```

### 說明

解析購物車 JSON 字串，將每項商品的 `price × quantity` 累加計算總額。

### 預期輸出

```
const jsonInput = '[{"item":"Apple","price":20,"quantity":3},{"item":"Banana","price":10,"quantity":5}]';
calculateTotal(jsonInput) → 110  （20×3 + 10×5）
```

## 4-9 過濾物件陣列.js

```javascript
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

### 說明

手動實作 `Array.filter()` 的底層邏輯。用新陣列存放符合 `age >= 18` 條件的元素，不修改原始陣列。

### 預期輸出

```
const users = [
    { name: "Tom", age: 15 },
    { name: "Jerry", age: 20 },
    { name: "Mickey", age: 35 }
];
getAdults(users) → [ { name: 'Jerry', age: 20 }, { name: 'Mickey', age: 35 } ]
```

## 4-10 陣列數字加總.js

```javascript
function sumArray(numbers) {
    let total = 0;
    for (let i = 0; i < numbers.length; i++) {
        total += numbers[i];
    }
    return total;
}
```

### 說明

累加器模式（Accumulator）：用 `for` 迴圈逐項累加，將一維陣列降維收斂為單一數值。等同 `reduce()` 的基礎原理。

### 預期輸出

```
sumArray([10, 20, 30]) → 60
```

---

# Ch5 — Node.js + Express 網誌全端專案

## 目錄

```
Ch5/
├── blog/
│   ├── package.json      express / sqlite3 / bcryptjs / express-session / marked
│   ├── app.js            主程式（14 支 API）
│   ├── database.js       SQLite3 資料庫初始化
│   └── public/
│       └── index.html    前端頁面（Threads 風格）
├── blog1/                簡化版（文章 CRUD + 留言）
│   ├── package.json / app.js / database.js
├── blog2/                與 blog1 相同
└── doc/對話摘要.md        開發記錄
```

---

## database.js — SQLite3 資料庫

```javascript
const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./blog.db', (err) => {
    if (err) { console.log('資料庫連線失敗:', err.message); }
    else { console.log('資料庫連線成功'); initTables(); }
});
```

### 資料表設計

```
users
├── id            INTEGER PK AUTOINCREMENT
├── username      TEXT UNIQUE NOT NULL   （登入帳號）
├── password      TEXT NOT NULL          （bcrypt 加密）
├── display_name  TEXT                   （顯示名稱）
├── bio           TEXT                   （自我介紹）
├── avatar        TEXT                   （頭像網址，預設 DiceBear）
└── created_at    DATETIME              （註冊時間）

posts
├── id            INTEGER PK
├── user_id       INTEGER FK → users(id)
├── content       TEXT NOT NULL
├── image_url     TEXT
└── created_at    DATETIME

likes
├── id            INTEGER PK
├── user_id       FK → users(id)
├── post_id       FK → posts(id)
├── UNIQUE(user_id, post_id)     ← 防止重複按讚
└── created_at

follows
├── id            INTEGER PK
├── follower_id   FK → users(id)
├── following_id  FK → users(id)
├── UNIQUE(follower_id, following_id)  ← 防止重複追蹤
└── created_at
```

---

## app.js — Express 主程式

```javascript
const express = require('express');
const session = require('express-session');
const bcrypt = require('bcryptjs');
const db = require('./database');
```

### 中介層設定

```javascript
app.use(express.json());                    // 解析 JSON body
app.use(express.urlencoded({ extended: true }));  // 解析表單
app.use(express.static('public'));          // 靜態檔案
app.use(session({                           // Session 管理
    secret: 'threads_secret_key',
    resave: false,
    saveUninitialized: false,
    cookie: { maxAge: 7 * 24 * 60 * 60 * 1000 }  // 7 天
}));
```

### 認證中介層

```javascript
function isAuthenticated(req, res, next) {
    if (req.session.userId) return next();
    res.status(401).json({ error: '請先登入' });
}
```

### API 路由

```
POST /register
  請求: { username, password, display_name }
  流程: 密碼 bcrypt.hashSync(10) → INSERT INTO users
  回傳: { message: "註冊成功", userId: id }

POST /login
  請求: { username, password }
  流程: 查詢使用者 → bcrypt.compareSync → 寫入 session
  回傳: { message: "登入成功", user: { id, username, display_name } }

POST /logout
  流程: req.session.destroy()
  回傳: { message: "已登出" }

GET /me  (需登入)
  流程: SQL 子查詢 stat（貼文數、追蹤數、粉絲數）→ 回傳用戶資料

PUT /me  (需登入)
  請求: { display_name, bio, avatar }
  流程: UPDATE users SET ... COALESCE
  回傳: { message: "更新成功" }

GET /users/:id
  流程: 查詢用戶 + stat + 是否已追蹤
  回傳: user（不含 password）

POST /follow/:id  (需登入)
  流程: INSERT OR IGNORE INTO follows
  回傳: { message: "已追蹤" }

DELETE /follow/:id  (需登入)
  流程: DELETE FROM follows
  回傳: { message: "已取消追蹤" }

GET /feed
  流程: SELECT posts WHERE user_id IN（追蹤的人）OR 自己
        含子查詢 likes_count + liked
  回傳: posts[]

GET /posts
  流程: SELECT 全部貼文（探索頁面）
  回傳: posts[]

GET /users/:id/posts
  流程: SELECT WHERE p.user_id = :id
  回傳: posts[]

POST /posts  (需登入)
  請求: { content, image_url }
  流程: INSERT INTO posts
  回傳: { message: "發布成功", id }

DELETE /posts/:id  (需登入)
  流程: 驗證作者 → DELETE likes → DELETE posts
  回傳: { message: "刪除成功" }

POST /posts/:id/like  (需登入)
  流程: INSERT OR IGNORE INTO likes
  回傳: { message: "已按讚" }

DELETE /posts/:id/like  (需登入)
  流程: DELETE FROM likes
  回傳: { message: "已取消按讚" }
```

---

## index.html — 前端頁面

### 版面

```
┌─────────────┬─────────────────────┬────────────┐
│  側欄導航    │    中央貼文串流      │  右側資訊   │
│             │                     │            │
│  𝕏 Logo     │  追蹤動態 │ 探索     │  大頭照     │
│  首頁       │  ────────────────   │  姓名      │
│  探索       │  發文編輯器          │  貼文 0    │
│  個人資料   │  ────────────────   │  追蹤中 0  │
│  登入/註冊  │  貼文 1（頭像+內容） │  粉絲 0    │
│  發布按鈕   │  貼文 2             │  自我介紹   │
│             │  貼文 3             │  追蹤按鈕   │
└─────────────┴─────────────────────┴────────────┘
```

### 前端 API 串接

```javascript
async function api(endpoint, options = {}) {
    const res = await fetch(endpoint, {
        ...options,
        credentials: 'include',                    // 傳送 session cookie
        headers: { 'Content-Type': 'application/json' }
    });
    return res.json();
}

async function checkAuth() {     // GET /me 檢查登入狀態
    const user = await api('/me');
    if (user.id) {
        currentUser = user;
        // 更新 UI：顯示登出、更新個人資訊卡
    }
}

function renderPosts(posts) {    // 渲染貼文串流
    document.getElementById('feed').innerHTML = posts.map(post => `
        <div class="post">
            <img src="${post.avatar}">
            <div class="post-content">
                <div>${post.display_name} @${post.username}</div>
                <div>${post.content}</div>
                <div class="action-btn ${post.liked ? 'liked' : ''}"
                     onclick="toggleLike(${post.id}, ${post.liked})">
                    ♥ ${post.likes_count}
                </div>
            </div>
        </div>
    `).join('');
}
```

---

## blog1 / blog2（簡化版）

### app.js 差異

相較於 blog（社群風格），blog1/blog2 的 API：

```
POST   /posts             建立文章（需 title + content）
GET    /posts             列出所有文章（含作者 username）
GET    /posts/:id         單一文章（含留言）
PUT    /posts/:id         更新文章（需登入）
DELETE /posts/:id         刪除文章（需登入）
POST   /posts/:id/comments 留言（不需登入，需 author_name）
```

### 資料表差異

```
blog1/users:  id, username, password（無 display_name / bio / avatar）
blog1/posts:  id, title, content, author_id, created_at, updated_at
blog1/comments: id, post_id, author_name, content, created_at
```

blog 加入了 **likes** 與 **follows** 表，去掉了 comments 表。

---

## 啟動方式

```bash
cd C:\yy\_wp\homework\Ch5\blog
npm install
npm start
# 開啟 http://localhost:3000
```

---

# Ch6 — JavaScript 進階練習（10 題）

## 6-1 回呼函式.js

```javascript
function A(num1, num2, B) { return B(num1, num2); }
let resultAdd = A(10, 5, function(a, b) { return a + b; });
let resultSub = A(10, 5, function(a, b) { return a - b; });
```

### 說明

高階函式 `A` 接收兩個數字與一個回呼函式 `B`，將數字傳入 `B` 執行。示範 JavaScript 的 First-class Function——函式可以當作參數傳遞。

### 預期輸出

```
15   (10 + 5)
5    (10 - 5)
```

---

## 6-2 IIFE.js

```javascript
(function() {
    let count = 100;
    console.log("Count is: " + count);
})();
```

### 說明

IIFE（Immediately Invoked Function Expression）：函式定義後立即執行。內部變數 `count` 只存在於函式作用域內，不會汙染全域命名空間。

### 預期輸出

```
Count is: 100
```

---

## 6-3 map().js

```javascript
const prices = [100, 200, 300, 400];
const discounted = prices.map(price => price * 0.8);
```

### 說明

`map()` 對陣列每個元素套用箭頭函式 `price => price * 0.8`，回傳一個**新陣列**（原陣列不變）。

### 預期輸出

```
[80, 160, 240, 320]
```

---

## 6-4 陣列操作.js

```javascript
function cleanData(arr) {
    arr.pop();                // 移除末端的 3
    arr.unshift("Start");     // 開頭插入 "Start"
}
let myData = [1, 2, 3];
cleanData(myData);
```

### 說明

`pop()` 移除陣列最後一個元素，`unshift()` 在陣列開頭插入新元素。陣列是傳參考，函式內的修改會直接影響原陣列。

### 預期輸出

```
["Start", 1, 2]
```

---

## 6-5 閉包.js

```javascript
function multiplier(factor) {
    return (n) => n * factor;
}
const double = multiplier(2);
```

### 說明

閉包（Closure）：`multiplier(2)` 回傳的函式「記住」了 `factor = 2`，即使 `multiplier` 已執行完畢。每個呼叫產生獨立的執行環境。

### 預期輸出

```
double(10) → 20
triple(10) → 30  （若 multiplier(3)）
```

---

## 6-6 自訂 filter.js

```javascript
function myFilter(arr, callback) {
    let result = [];
    for (let i = 0; i < arr.length; i++) {
        if (callback(arr[i])) result.push(arr[i]);
    }
    return result;
}
let filtered = myFilter(data, function(item) { return item > 7; });
```

### 說明

手動實作 `Array.prototype.filter()` 底層機制。用 `for` 迴圈遍歷，當 callback 回傳 `true` 時將元素加入結果陣列。

### 預期輸出

```
[8, 12]
```

---

## 6-7 filter().js

```javascript
const users = [{ name: "Alice", age: 25 }, { name: "Bob", age: 17 }];
const adults = users.filter(user => user.age >= 18);
```

### 說明

使用內建 `Array.filter()` 搭配箭頭函式。`filter` 回傳一個新陣列，原陣列不變。只保留條件為 `true` 的元素。

### 預期輸出

```
[{ name: "Alice", age: 25 }]
```

---

## 6-8 傳參考.js

```javascript
let listA = [1, 2], listB = [3, 4];
function process(a, b) {
    a.push(99);       // push 修改原陣列
    b = [100];        // 指派新陣列，不影響原變數
}
process(listA, listB);
```

### 說明

`push(99)` 透過參考直接修改 `listA`。`b = [100]` 將參數 `b` 重新指向新陣列，不影響外層的 `listB`。

### 預期輸出

```
[1, 2, 99]    (listA)
[3, 4]        (listB)
```

---

## 6-9 setTimeout.js

```javascript
const arr = ["Task", "Completed"];
setTimeout(() => { console.log(arr.join(" ")); }, 2000);
```

### 說明

`setTimeout` 設定 2000 毫秒後執行箭頭函式。示範 JavaScript 的非同步特性——程式會繼續往下執行，時間到才執行回呼內的程式碼。

### 流程

```
執行 setTimeout（註冊 2 秒後執行）
  → 繼續執行後續程式（如果有的話）
  → 2 秒後 → 箭頭函式被呼叫 → arr.join(" ") → 輸出
```

### 預期輸出（2 秒後）

```
Task Completed
```

---

## 6-10 reduce + Callback.js

```javascript
function calculateTotal(cart, discountFunc) {
    let sum = cart.reduce((acc, price) => acc + price, 0);
    return discountFunc(sum);
}
const result = calculateTotal([100, 200, 300], function(total) {
    return total - 50;
});
```

### 說明

`reduce()` 將購物車金額陣列累加為單一總和（初始值 0），再將總和傳入折扣回呼函式計算最終金額。

### 流程

```
[100, 200, 300]  →  reduce(acc + price)  →  600
600  →  discountFunc(600)  →  600 - 50  →  550
```

### 預期輸出

```
550
```

---

# Ch7 — JavaScript 實戰練習（10 題）

## 7-1 物件屬性存取.js

```javascript
let post = { id: 1, title: "Hello World", content: "Markdown content" };
console.log(post.title);      // 點記法
console.log(post["title"]);   // 括號記法
```

### 說明

JavaScript 提供兩種存取物件屬性的方式：點記法（`obj.key`）靜態、簡潔；括號記法（`obj["key"]`）可接受變數或動態字串。

### 預期輸出

```
Hello World
Hello World
```

---

## 7-2 解構賦值.js

```javascript
const req = { body: { title: "JS教學", content: "內容在此", author: "Gemini" } };
const { title, content } = req.body;
```

### 說明

解構賦值（Destructuring）直接從物件中提取屬性命名为變數，取代傳統的 `const title = req.body.title` 寫法，常見於 Express 路由處理。

### 預期輸出

```
JS教學
內容在此
```

---

## 7-3 forEach.js

```javascript
const posts = [{id: 1, t: "A"}, {id: 2, t: "B"}];
let html = "";
posts.forEach(post => { html += `<div>${post.t}</div>`; });
```

### 說明

`forEach()` 遍歷陣列每個元素，用模板字串逐步組合 HTML。這是前端渲染（template rendering）的基本模式。

### 預期輸出

```
<div>A</div><div>B</div>
```

---

## 7-4 動態物件鍵.js

```javascript
const params = {};
params["id"] = 99;
```

### 說明

使用括號記法動態新增物件屬性。等同於 `params.id = 99`。差別在於括號記法可以使用變數作為鍵名，常見於 `req.params` 處理。

### 預期輸出

```
{ id: 99 }
```

---

## 7-5 Callback 非同步.js

```javascript
function fetchData(id, callback) {
    const data = { id: id, status: "success" };
    callback(null, data);          // Error-First：第一個參數是 error
}

fetchData(123, (err, result) => {
    if (err) return console.log("發生錯誤:", err);
    console.log("取得資料:", result);
});
```

### 說明

Error-First Callback 是 Node.js 的標準非同步慣例。回呼函式的第一個參數保留給錯誤物件（無錯誤時為 `null`），第二個參數才是實際資料。

### 預期輸出

```
取得資料: { id: 123, status: "success" }
```

---

## 7-6 JSON 解析.js

```javascript
const jsonStr = '{"title": "Post 1", "tags": ["js", "node"]}';
const obj = JSON.parse(jsonStr);
console.log(obj.tags[1]);
```

### 說明

`JSON.parse()` 將 JSON 字串轉換為 JavaScript 物件。可以透過點記法或索引存取巢狀結構中的資料。

### 預期輸出

```
node
```

---

## 7-7 模擬 DB 查詢.js

```javascript
function fakeGet(sql, params, callback) {
    callback(null, { title: "Fake Title" });
}

fakeGet("SELECT * FROM posts WHERE id = ?", [1], (err, row) => {
    if (err) return console.log("資料庫錯誤:", err);
    console.log("文章標題:", row.title);
});
```

### 說明

模擬 SQLite 的 `db.get(sql, params, callback)` 模式。第一個參數是 SQL 語句，第二個是參數陣列，第三個是 Error-First Callback。此模式也常見於其他 Node.js 資料庫套件。

### 預期輸出

```
文章標題: Fake Title
```

---

## 7-8 模板字串.js

```javascript
const user = "Guest";
const htmlStr = `<h1>Welcome, ${user ? user : "Stranger"}</h1>`;
```

### 說明

模板字串（Template Literals）使用反引號`` ` ``與`${}`語法嵌入表達式。這裡使用三元運算子：若 `user` 有值則顯示，否則顯示 "Stranger"。

### 預期輸出

```
<h1>Welcome, Guest</h1>
```

若 `user = null`：

```
<h1>Welcome, Stranger</h1>
```

---

## 7-9 空白.js

空檔案，無內容。可能為預留位置。

---

## 7-10 錯誤處理.js

```javascript
function checkAdmin(role, callback) {
    if (role !== "admin") { callback("Access Denied"); }
    else { callback(null, "Welcome"); }
}

checkAdmin("user", (err, message) => {
    if (err) return console.log("測試 1 錯誤攔截:", err);
    console.log("測試 1 成功:", message);
});

checkAdmin("admin", (err, message) => {
    if (err) return console.log("測試 2 錯誤攔截:", err);
    console.log("測試 2 成功:", message);
});
```

### 說明

完整 Error-First Callback 實作。第一個呼叫傳入 `"user"`（非 admin），callback 收到 `"Access Denied"` 作為錯誤。第二個呼叫傳入 `"admin"`，callback 收到 `null` 錯誤 + `"Welcome"` 資料。

### 預期輸出

```
測試 1 錯誤攔截: Access Denied
測試 2 成功: Welcome
```

---

# 執行方式

```bash
# Ch3
node "C:\yy\_wp\homework\Ch3\hello.js"

# Ch4（任一題）
node "C:\yy\_wp\homework\Ch4\陣列數字加總.js"

# Ch5
cd "C:\yy\_wp\homework\Ch5\blog"
npm install
npm start

# Ch6（任一題）
node "C:\yy\_wp\homework\Ch6\1.js"

# Ch7（任一題）
node "C:\yy\_wp\homework\Ch7\01.js"
```

---

# 學習路徑

```
Ch1 HTML 網頁排版（Flexbox、RWD）
  ↓
Ch2 HTML 表單 + CSS 自訂樣式（13 種輸入元件）
  ↓
Ch3 JavaScript 基本輸出（console.log）
  ↓
Ch4 JavaScript 演算法基礎
    （陣列/物件/JSON/迴圈/條件/演算法）
  ↓
Ch5 Node.js 全端專案整合
    （Express + SQLite + Session + 前端 UI）
  ↓
Ch6 JavaScript 進階技巧
    （Callback / IIFE / map / filter / reduce / 閉包）
  ↓
Ch7 JavaScript 實戰應用
    （解構賦值 / 模板字串 / JSON / Error-First Callback）
```
