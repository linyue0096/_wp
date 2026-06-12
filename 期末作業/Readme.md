# 網頁設計 — 期末學習報告

## 目錄

1. [學習進程](#學習進程)
2. [AI 使用說明](#ai-使用說明)
3. [各作業說明](#各作業說明)
   - [Ch1：HTML 個人簡歷](#ch1-html-個人簡歷)
   - [Ch2：HTML 會員註冊表單](#ch2-html-會員註冊表單)
   - [Ch3：Node.js 環境設定](#ch3-nodejs-環境設定)
   - [Ch4：JavaScript 基礎演算法](#ch4-javascript-基礎演算法)
   - [Ch5：Express 網誌系統](#ch5-express-網誌系統)
   - [Ch6：JavaScript 進階函式](#ch6-javascript-進階函式)
   - [Ch7：Express 後端開發實務](#ch7-express-後端開發實務)
   - [期中專題：E-Shop 電商平台](#期中專題-e-shop-電商平台)
4. [學習總結](#學習總結)

---

## 學習進程

### 第一階段：HTML 基礎（Ch1 ~ Ch2）
* 皆使用opencode
* Ch1將我的簡歷給opencode讓他幫我排版產出
* Ch2使用老師上課的提示詞給opencode讓他幫我做出這份作品
學習 HTML 語意標籤、CSS 排版與響應式設計。從個人簡歷頁面開始，掌握 flexbox 雙欄佈局；接著實作會員註冊表單，熟悉各類 input 元素（文字、密碼、日期、顏色、檔案上傳、range 滑桿）與表單驗證。

### 第二階段：Node.js 與 JavaScript（Ch3 ~ Ch4）
* Ch3 查找且參考JavaScript程式碼 : https://www.runoob.com/jsref/met-console-log.html
* Ch4 將老師給的提示詞丟給opencode讓他幫我撰寫,且讓opencode幫我寫一份readme,讓我了解這些程式是做甚麼、執行出來的結果是甚麼樣
安裝 Node.js 環境，撰寫第一支 Hello.js 確認執行環境正常。進入 JavaScript 核心語法，包含陣列操作、物件處理、JSON 解析、條件判斷、迴圈與演算法實作，並探討時間/空間複雜度、Pass by Sharing、IEEE 754 浮點數等底層觀念。

### 第三階段：後端框架與資料庫（Ch5 ~ Ch7）
* Ch5 參考老師上課的提示詞,使用opencode幫我撰寫,且讓opencode幫我保留對話紀錄
* Ch6 使用opencode幫我寫出答案,閱讀後自行嘗試練習後,修正錯誤
* Ch7 使用opencode幫我寫出答案,閱讀後自行嘗試練習後,修正錯誤
以 Express + SQLite3 建構完整後端系統，從基本 CRUD 部落格逐步迭代到 Threads 風格社群網誌（含按讚、追蹤功能）。Ch6 深入高階函式（map、filter、reduce、closure、callback），Ch7 以 Express 實務情境整合所有觀念（路由、中介層、JSON 處理、模板渲染、資料庫查詢）。

### 第四階段：期中專題 — Rust + Axum 電商平台
* 全使用opencode幫我製作,請opencode幫我保留開發過程
使用 Rust 語言搭配 Axum 框架開發全端電商網站，涵蓋買家購物流程、賣家後台管理、JWT 認證、SQLite 資料庫設計、Tera 模板引擎，並透過 GitHub Pages 部署靜態展示頁面。
---

## 各作業說明
* 請opencode幫我主要將程式碼抓出,我補說明,跟一部分程式碼的解釋,和執行結果
### Ch1：HTML 個人簡歷

**檔案：** `homework/Ch1/Ch1.html`

個人履歷頁面，使用 flexbox 雙欄佈局（左欄個人資訊、右欄經歷）：
- 左欄：大頭照、姓名、聯絡資訊、教育背景、軟實力、專業技能標籤（C/C++、Python、嵌入式系統、Linux/WSL）
- 右欄：學習服務經歷、幹部經歷（輔導股長、圖書股長）、社團參與（童軍社）、證照檢定
- 響應式設計：螢幕小於 768px 時切換為單欄

### Ch2：HTML 會員註冊表單

**檔案：** `homework/Ch2/register.html`

暗色系風格（黑金配色）註冊頁面，包含：
- 文字輸入：姓名、Email、密碼、確認密碼、電話、地址
- 日期選擇器、性別 radio、興趣 checkbox
- 國家下拉選單、年齡 range 滑桿（即時顯示數值）
- 顏色選擇器（即時顯示色碼）、檔案上傳（自訂樣式）、自我介紹 textarea
- JavaScript 即時更新 range 數值與色碼

### Ch3：Node.js 環境設定

**檔案：** `homework/Ch3/hello.js`

第一行程式碼，確認 Node.js 環境正常運作：
```javascript
console.log('Hello 你好');
```
```
> Hello 你好
```
用意：確認 Node.js 已正確安裝，可以執行 JavaScript 程式。

### Ch4：JavaScript 基礎演算法

**目錄：** `homework/Ch4/`（10 個 .js 檔案 + Readme.md）

本階段老師提供提示詞（prompt），我將提示詞交給 opencode 協助撰寫程式碼，並由 opencode 產生 Readme，幫助我理解每支程式在做什麼、執行結果是什麼。

透過這 10 個練習，我學到了以下核心觀念：

| 概念 | 說明 |
|------|------|
| **陣列操作** | 建立陣列、`for` / `while` 迴圈走訪、`push` 新增元素 |
| **物件處理** | 建立物件、存取屬性（點記號 / 括號記號）、物件陣列混合使用 |
| **JSON 雙向轉換** | `JSON.parse()` 字串轉物件、`JSON.stringify()` 物件轉字串 |
| **條件判斷** | `if / else` 條件分支、三元運算子簡化寫法 |
| **Pass by Sharing** | 物件/陣列以參考傳遞，函式內修改會直接影響原資料 |
| **時間複雜度** | `for` / `while` 的 O(n) 線性搜尋，認識演算法效率 |
| **IEEE 754 浮點數** | 分數計算有小數誤差（如 85.666...），了解浮點數精確度限制 |

| # | 檔案 | 功能 | 核心概念 |
|---|------|------|---------|
| 1 | 圖書館書籍搜尋.js | 依作者查詢書籍 | 線性搜尋 O(N) |
| 2 | 基本條件判斷.js | 成績及格判斷 | 條件分支 O(1) |
| 3 | 尋找特定目標.js | 陣列搜尋目標值 | while 迴圈 + Early Return |
| 4 | 建立並回傳物件.js | 工廠模式建立使用者 | 物件建立 O(1) |
| 5 | 成績系統分析.js | JSON 解析 → 平均 → 評等 | 資料管線 O(N×M)、IEEE 754 |
| 6 | 提款機餘額模擬.js | 重複扣款直到餘額不足 | Pass by Sharing |
| 7 | 物件格式轉換與導出.js | 合併平行陣列為物件陣列 | Zip 運算 O(N) |
| 8 | 解析 JSON 並計算總額.js | 購物車總價計算 | 浮點數精度 |
| 9 | 過濾物件陣列.js | 篩選成年人 | filter 底層實作 |
| 10 | 陣列數字加總.js | 陣列總和 | Accumulator 模式 |

**Ch4 學習總結：** 這 10 個練習讓我建立了 JavaScript 的扎實基礎。從最簡單的條件判斷（O(1)）到陣列搜尋（O(n)），再到 JSON 資料的解析與序列化，每一個練習都環環相扣。最重要的是我理解了 Pass by Sharing（物件傳參考）和 IEEE 754（浮點數精度）這些底層觀念，這對於日後除錯和寫出高效程式碼非常有幫助。

### Ch5：Express 網誌系統

**目錄：** `homework/Ch5/blog/`

Express + SQLite3 社群網誌，session 認證、19 條 RESTful API、Threads 黑金風格前端。

```js
const express = require('express');
const session = require('express-session');
const db = require('./database');
const app = express();
app.use(express.json());
app.use(session({ secret: 'threads_secret_key', resave: false, saveUninitialized: false }));

app.get('/api/posts', (req, res) => {
  db.all('SELECT p.*, u.username FROM posts p JOIN users u ON p.user_id = u.id ORDER BY p.created_at DESC', [], (err, rows) => {
    res.json(rows);
  });
});

app.post('/api/posts', (req, res) => {
  const { title, content } = req.body;
  db.run('INSERT INTO posts (user_id, title, content) VALUES (?, ?, ?)', [req.session.userId, title, content], function () {
    res.json({ id: this.lastID });
  });
});
```

另含 blog1、blog2 簡化版（僅文章 CRUD + 留言板），展示逐步演進過程。

```
> 啟動於 http://localhost:3000
> GET /api/posts → [ { id:1, title:"...", content:"...", username:"..." }, ... ]
```
用意：瀏覽器開啟後看到 Threads 黑金風格頁面，API 回傳 JSON 貼文列表。

### Ch6：JavaScript 進階函式

**目錄：** `homework/Ch6/`（10 個 .js 檔案）

```js
// 1. Callback — 把函式當參數傳遞
function A(num1, num2, B) { return B(num1, num2); }
A(10, 5, (a, b) => a + b); // 15

// 2. IIFE — 寫完立刻執行，變數不汙染外部
(function() { let count = 100; console.log(count); })();

// 3. map() — 不修改原陣列，產生新陣列
const prices = [100, 200, 300, 400];
prices.map(p => p * 0.8); // [80, 160, 240, 320]

// 4. 副作用 — 函式內修改外部陣列
function cleanData(arr) { arr.pop(); arr.unshift("Start"); }

// 5. Closure — 記住外部變數的函式工廠
const multiplier = factor => n => n * factor;
multiplier(2)(10); // 20

// 6. 自訂 myFilter — 自己實作 filter
function myFilter(arr, callback) {
  let result = [];
  for (let i = 0; i < arr.length; i++)
    if (callback(arr[i])) result.push(arr[i]);
  return result;
}

// 7. Array.filter() — 選出成年人
const adults = users.filter(user => user.age >= 18);

// 8. 引用 vs 重新賦值 — push 改原陣列，賦值不影響
function process(a, b) { a.push(99); b = [100]; } // listA 被改，listB 不變

// 9. setTimeout — 非同步延遲執行
setTimeout(() => console.log("2 秒後執行"), 2000);

// 10. reduce + callback — 聚合後再套優惠
function total(cart, fn) { return fn(cart.reduce((a, p) => a + p, 0)); }
total([100, 200, 300], t => t - 50); // 550
```
```
> 15
> 5
> Count is: 100
> [ 80, 160, 240, 320 ]
> [ "Start", 2 ]
> 20
> [ 8, 12 ]
> [ { name: "Alice", age: 25 } ]
> [ 1, 2, 99 ]
> [ 3, 4 ]
> Task Completed  (延遲 2 秒後出現)
> 550
```
用意：每個練習獨立執行，輸出結果驗證函式行為 — callback 傳遞邏輯、map/filter 陣列轉換、閉包記住外部變數、setTimeout 非同步等待。

### Ch7：Express 後端開發實務

**目錄：** `homework/Ch7/`（10 個 .js 檔案）

```js
// 01. 物件屬性存取 — . 和 [] 等價
post.title    post["title"]     // → "Hello World"

// 02. 解構賦值 — 從 req.body 取出資料
const { title, content } = req.body;

// 03. forEach 組合 HTML — 模板渲染的雛形
posts.forEach(p => html += `<div>${p.t}</div>`);

// 04. 動態物件鍵 — URL 參數處理
const params = {};  params["id"] = 99;

// 05. callback 非同步 — 模擬 db.get
function fetchData(id, callback) { callback(null, { id, status: "success" }); }

// 06. JSON.parse — API 請求解析
const obj = JSON.parse('{"title":"Post 1","tags":["js","node"]}');

// 07. 模擬 SQL 查詢 — callback 模式
function fakeGet(sql, params, callback) { callback(null, { title: "Fake Title" }); }

// 08. 模板字串條件渲染
`<h1>Welcome, ${user ? user : "Stranger"}</h1>`

// 09. （空檔案）

// 10. Error-first callback — Express 錯誤處理慣例
function checkAdmin(role, callback) {
  if (role !== "admin") callback("Access Denied");
  else callback(null, "Welcome");
}
```
```
> Hello World
> Hello World
> JS教學
> 內容在此
> <div>A</div><div>B</div>
> { id: 99 }
> 取得資料: { id: 123, status: "success" }
> node
> 文章標題: Fake Title
> <h1>Welcome, Guest</h1>
> 測試 1 錯誤攔截: Access Denied
> 測試 2 成功: Welcome
```
用意：每支程式模擬 Express 開發中的一個實務情境 — 物件屬性存取對應 req.body、解構賦值提取資料、forEach 組合 HTML 等於模板渲染、callback 模擬 db.query、error-first 模式處理錯誤。

### 期中專題：E-Shop 電商平台

**目錄：** `期中/ecommerce/`

Rust + Axum + SQLite + Tera 全端電商，支援買家購物、賣家管理、JWT 認證、7 張關聯表。

```rust
use axum::{routing::get, Router};
use sqlx::SqlitePool;
use tera::Tera;
use tower_http::services::ServeDir;

struct AppState { db: SqlitePool, tmpl: Tera, jwt_secret: String }

#[tokio::main]
async fn main() {
    let db = init_db().await;
    let app = Router::new()
        .route("/", get(|| async { "Hello E-Shop" }))
        .route("/products", get(product_list))
        .route("/products/:id", get(product_detail))
        .route("/cart/add", post(cart_add))
        .route("/cart", get(cart_view))
        .route("/checkout", post(checkout))
        .route("/admin/products", get(admin_products))
        .route("/admin/products/new", post(admin_product_new))
        .route("/orders/:id/status", post(admin_order_update))
        .nest_service("/static", ServeDir::new("static"));
}
```
```
> cargo run
> 編譯中...
> 啟動於 http://localhost:3000
> 瀏覽器開啟可見：商品列表、購物車、賣家儀表板
```
用意：Rust 編譯成功後啟動 Axum 伺服器，提供完整電商功能（商品瀏覽、加入購物車、結帳、賣家管理）。

## 學習總結

### 技術收穫

1. **前端基礎（Ch1 ~ Ch2）：**
   - 從零開始建立第一個 HTML 頁面，學會語意標籤（header、section、article、footer）的正確用法
   - 掌握 flexbox 雙欄佈局與 `@media` 響應式設計，理解如何在不同螢幕尺寸下調整版面
   - 熟悉 HTML 表單所有輸入類型（text、email、password、date、color、file、range、tel），並實作即時顯示 range 數值與色碼的 JavaScript 互動
   - 學會 CSS 變數、漸層、transition 動畫、自定義 radio/checkbox 樣式等進階切版技巧

2. **JavaScript 核心（Ch3 ~ Ch4、Ch6）：**
   - 從 `console.log` 第一行程式碼出發，逐步建立完整的 JavaScript 基礎
   - 掌握陣列操作（建立、走訪、搜尋、過濾、累加）與物件處理（建立屬性、點記號/括號記號存取、物件陣列混合使用）
   - 理解 JSON 雙向轉換：`JSON.parse()` 與 `JSON.stringify()`，這是前後端溝通的基礎
   - 深入底層觀念：時間複雜度（O(1)、O(n)、O(N×M)）、Pass by Sharing（物件傳參考）、IEEE 754 浮點數精度問題
   - 進階函式概念：callback 回呼、IIFE 立即執行函式、Closure 閉包（函式工廠）、高階陣列方法（map、filter、reduce）

3. **後端開發（Ch5、Ch7）：**
   - 使用 Node.js + Express 建立 RESTful API，理解路由設定、中介層（middleware）、請求處理流程
   - 串接 SQLite3 資料庫，學會 db.all / db.get / db.run 的 CRUD 操作與 SQL 語法
   - 實作 session-based 認證（express-session + bcryptjs 密碼加密），理解無狀態與有狀態認證的差異
   - 從 Ch7 的 10 個練習中掌握 Express 實務模式：解構賦值提取 req.body、模板字串渲染、error-first callback 錯誤處理、動態物件鍵處理 URL 參數

4. **全端整合與系統級語言（期中專題）：**
   - 使用 Rust + Axum 框架開發完整電商網站，涵蓋買家購物流程（註冊、瀏覽、購物車、結帳）與賣家後台管理（商品 CRUD、訂單處理）
   - 學習 Rust 的所有權系統、借用檢查器、async/await 非同步程式設計
   - 使用 Tera 模板引擎進行伺服器端渲染，JWT + Cookie 實現無狀態認證
   - 設計 7 張 SQLite 關聯表（users、categories、products、cart_items、orders、order_items、reviews），理解 foreign key 與 JOIN 查詢

### 學習方法

- **教材為主、AI 為輔：** 每一章從閱讀教材（W3Schools、ccc114b/html2server）開始，先理解觀念再動手實作。遇到瓶頸時向 AI 提問，但會先自行思考問題根因，確保是真正理解而非盲目複製
- **由淺入深、逐步迭代：** 課程設計從 HTML/CSS 靜態頁面 → JavaScript 基礎語法 → 後端框架 → 全端專題，每一階段都建立在前一階段的基礎上。例如 Ch5 的 blog1 → blog2 → blog 三版本逐步疊代，先有基本 CRUD 再加入認證與社交功能
- **每個練習都寫註解與 Readme：** 每題程式碼不僅寫出正確功能，還附上逐行註解與 Readme 說明，強迫自己真正理解每一行的作用。這個習慣在除錯時特別有幫助 — 能快速定位問題來源
- **跨語言學習驗證：** 從 JavaScript（動態型別、直譯式）跨到 Rust（靜態型別、編譯式），雖然語法差異大，但因為已經理解程式設計的底層邏輯（變數、函式、物件、陣列、迴圈），換語言時只需要學習語法差異與生態系工具

### 自我反思
- **待加強：** 前端 JavaScript 動態互動（DOM 操作、事件處理）的練習相對較少，Ch1 ~ Ch2 主要是靜態頁面設計，後續可強化這部分。另外單元測試（unit test）尚未接觸，這是後續可以補強的方向
- **心得：** 程式設計的關鍵在於「理解底層原理」— 懂了 Pass by Sharing 就不會被函式修改原陣列的行為嚇到；懂了 IEEE 754 就不會對 0.1 + 0.2 ≠ 0.3 感到困惑；懂了所有權概念就能寫出安全的 Rust 程式。原理一通，換語言或框架都很快
