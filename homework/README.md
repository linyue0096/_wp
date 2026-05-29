# C:\yy\_wp\homework

---

# Ch1：HTML 個人履歷

```
Ch1.html
```

一人兩欄式個人履歷網頁。

**左欄**：大頭照、姓名（陳宜昀）、聯絡電話/Email/地址、教育歷程（金門大學資工系）、能力特質、專業技能（C/C++、Python、嵌入式系統、Linux/WSL）

**右欄**：學習服務志工、幹部經歷（輔導股長、圖書股長、英文小老師、活動組長）、社團（童軍社）、中文檢定證照

---

# Ch2：HTML 會員註冊表單

```
register.html
```

深色主題註冊頁面。練習 HTML 表單各類輸入元件與 CSS 自訂樣式。

| 欄位 | 標籤 |
|------|------|
| 姓名 | `input[type=text]` |
| 電子郵件 | `input[type=email]` |
| 密碼、確認密碼 | `input[type=password]` |
| 電話 | `input[type=tel]` |
| 出生日期 | `input[type=date]` |
| 地址 | `input[type=text]` |
| 性別 | `input[type=radio]`（男/女/其他） |
| 興趣 | `input[type=checkbox]`（閱讀/音樂/運動/旅遊/遊戲） |
| 國家 | `select` + `option` |
| 年齡 | `input[type=range]`（1~100） |
| 喜歡的顏色 | `input[type=color]` |
| 自我介紹 | `textarea` |
| 頭像 | `input[type=file]` |

JavaScript 實現滑桿數值即時顯示、選色即時顯示色碼。

---

# Ch3：JavaScript 入門

```
hello.js
```

```javascript
console.log('Hello 你好')
```

最基本的 `console.log` 輸出。執行：`node hello.js`

---

# Ch4：JavaScript 演算法 10 題

每題一個獨立 `.js` 檔，共同技術文件在 `Readme.md`。

| 檔名 | 函式名稱 | 練習重點 |
|------|----------|----------|
| 圖書館書籍搜尋.js | `findBooksByAuthor()` | 物件陣列線性搜尋、`===` 嚴格比對 |
| 基本條件判斷.js | `checkPass()` | `if/else` 條件分支 |
| 尋找特定目標.js | `findTarget()` | `while` 迴圈、Early Return |
| 建立並回傳物件.js | `createUser()` | 工廠模式、Heap 物件建立 |
| 成績系統分析.js | `processStudentGrades()` | `JSON.parse`、巢狀迴圈、`JSON.stringify` |
| 提款機餘額模擬.js | `withdrawUntilEmpty()` | 傳參考副作用、`while` 迴圈 |
| 物件格式轉換與導出 JSON.js | `exportProductData()` | Zip 合併平行陣列 |
| 解析 JSON 並計算總價.js | `calculateTotal()` | JSON 解析、浮點數運算 |
| 過濾物件陣列.js | `getAdults()` | 手刻 `filter()`、不可變性 |
| 陣列數字加總.js | `sumArray()` | 累加器模式、等同 `reduce()` |

---

# Ch5：Node.js Express 網誌專案

## 目錄

```
Ch5/
├── blog/         完整版（社群網誌，有前端）
│   ├── package.json
│   ├── app.js
│   ├── database.js
│   └── public/index.html
├── blog1/        簡化版（純 API）
│   ├── package.json / app.js / database.js
├── blog2/        簡化版（與 blog1 相同）
└── doc/對話摘要.md
```

## 資料表（blog/database.js）

- **users**：`id, username, password(bcrypt), display_name, bio, avatar, created_at`
- **posts**：`id, user_id(FK), content, image_url, created_at`
- **likes**：`id, user_id(FK), post_id(FK), created_at` — 設 `UNIQUE(user_id, post_id)`
- **follows**：`id, follower_id(FK), following_id(FK), created_at` — 設 `UNIQUE(follower_id, following_id)`

## API（blog/app.js）

| 路由 | 功能 |
|------|------|
| `POST /register` | 註冊 |
| `POST /login` | 登入（session） |
| `POST /logout` | 登出 |
| `GET /me` | 當前用戶資料 |
| `PUT /me` | 更新個人資料 |
| `GET /users/:id` | 查看用戶 |
| `POST /follow/:id` | 追蹤 |
| `DELETE /follow/:id` | 取消追蹤 |
| `GET /feed` | 追蹤動態 |
| `GET /posts` | 全部貼文 |
| `GET /users/:id/posts` | 用戶貼文 |
| `POST /posts` | 發布 |
| `DELETE /posts/:id` | 刪除 |
| `POST /posts/:id/like` | 按讚 |
| `DELETE /posts/:id/like` | 取消按讚 |

## blog vs blog1/blog2

blog1/blog2 有文章 CRUD + 留言系統。blog 改為社群風格（按讚取代留言、新增追蹤系統、個人資料編輯），並附完整前端頁面。

## 前端（blog/public/index.html）

三欄式畫面：左側導航（首頁/探索/個人資料/登入/註冊/發布）、中央貼文串流（發文編輯器 + 貼文列表）、右側個人資訊卡。

前端用 Vanilla JavaScript + Fetch API 串接後端。

## 啟動

```bash
cd blog
npm install
npm start          # 開啟 http://localhost:3000
```

---

# Ch6：JavaScript 進階 10 題

| 檔名 | 主題 | 說明 |
|------|------|------|
| `1.js` | 回呼函式 | 高階函式 `A` 接收回呼 `B`，將兩數傳入執行加/減 |
| `2.js` | IIFE | 立即執行函式，區域變數不汙染全域 |
| `3.js` | map() | 箭頭函式對陣列每個元素打 8 折，回傳新陣列 |
| `4.js` | pop / unshift | 移除末端、插入開頭、示範傳參考 |
| `5.js` | 閉包 Closure | `multiplier(factor)` 回傳函式記住 factor |
| `6.js` | 自訂 filter | 手刻 `myFilter` 用 for 迴圈過濾陣列 |
| `7.js` | filter() 高階 | 內建 `filter` + 箭頭函式過濾成年人 |
| `8.js` | 傳參考 vs 傳值 | `push` 修改原陣列；`=` 指派不影響原變數 |
| `9.js` | setTimeout | 2 秒後非同步執行箭頭函式 |
| `10.js` | reduce + Callback | `reduce` 累加後傳入折扣回呼 |

---

# Ch7：JavaScript 實戰 10 題

| 檔名 | 主題 | 說明 |
|------|------|------|
| `01.js` | 物件屬性存取 | 點記法 `obj.key` vs 括號記法 `obj["key"]` |
| `02.js` | 解構賦值 | `const { title, content } = req.body` |
| `03.js` | forEach 遍歷 | 遍歷陣列組合 HTML 字串 |
| `04.js` | 動態物件鍵 | `params["id"] = 99` 動態新增屬性 |
| `05.js` | Callback 非同步 | Error-First Callback 模式 |
| `06.js` | JSON 解析 | `JSON.parse` 解析巢狀資料 |
| `07.js` | 模擬 DB 查詢 | 模擬 `db.get(sql, params, callback)` |
| `08.js` | 模板字串 | `` `${}` `` 嵌入表達式與三元運算子 |
| `09.js` | 空白 | 預留檔案 |
| `10.js` | 錯誤處理 | `checkAdmin` 用 callback 回傳錯誤或成功 |

---

# 學習順序

```
Ch1 HTML 網頁排版
  ↓
Ch2 HTML 表單 + CSS 自訂樣式
  ↓
Ch3 JS 基礎輸出
  ↓
Ch4 JS 變數/迴圈/條件/物件/陣列/JSON/演算法
  ↓
Ch5 Node.js 後端 + SQLite 資料庫 + 前端 UI（全端整合）
  ↓
Ch6 Callback / IIFE / map / filter / reduce / 閉包
  ↓
Ch7 解構賦值 / 模板字串 / JSON / Error-First Callback 實戰
```
