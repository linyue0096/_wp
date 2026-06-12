# E-Shop 電商平台 — 期中專題

## 目錄

1. [專案概述](#專案概述)
2. [開發過程](#開發過程)
3. [技術架構總覽](#技術架構總覽)
4. [後端說明 (Rust + Axum)](#後端說明-rust--axum)
   - [4.1 依賴套件與用途](#41-依賴套件與用途)
   - [4.2 專案結構與各檔案功能](#42-專案結構與各檔案功能)
   - [4.3 資料庫設計（完整欄位與關聯）](#43-資料庫設計完整欄位與關聯)
   - [4.4 API 路由大全](#44-api-路由大全)
   - [4.5 認證機制（JWT + Cookie）](#45-認證機制jwt--cookie)
   - [4.6 種子資料與初始化](#46-種子資料與初始化)
   - [4.7 付款流程實作細節](#47-付款流程實作細節)
5. [前端說明](#前端說明)
   - [5.1 Tera 模板引擎](#51-tera-模板引擎)
   - [5.2 頁面功能詳解](#52-頁面功能詳解)
   - [5.3 共用版型 (base.html)](#53-共用版型-basehtml)
   - [5.4 買家流程頁面](#54-買家流程頁面)
   - [5.5 賣家管理頁面](#55-賣家管理頁面)
   - [5.6 GitHub Pages 靜態前端](#56-github-pages-靜態前端)
6. [本地執行步驟](#本地執行步驟)
7. [Demo 帳號](#demo-帳號)
8. [開發工具與環境](#開發工具與環境)
9. [版本紀錄](#版本紀錄)

---

## 專案概述

本專案為一款全功能電子商務網站，採用 Rust 作為後端語言，以 Axum 框架提供高效能、非同步的 Web 服務。前端以伺服器端渲染（SSR）方式透過 Tera 模板引擎產生 HTML 頁面，搭配 Tailwind CSS 實現美觀、響應式的使用者介面。

本系統涵蓋完整的電子商務流程：

- **買家端：** 註冊 → 瀏覽商品 → 分類篩選 → 關鍵字搜尋 → 加入購物車 → 修改數量 → 結帳（選擇付款方式） → 送出訂單 → 查詢訂單狀態
- **賣家端：** 註冊賣家帳號 → 儀表板統計 → 新增商品 → 編輯/刪除商品 → 查看訂單 → 更新出貨狀態
- **付款方式：** 支援貨到付款（COD）與信用卡付款兩種選擇

---

## 開發過程

### 第一步：需求分析
使用者最初提出建立一個電商網頁版，要求包含以下功能：
- 貨到付款與信用卡付款
- 商品分類瀏覽
- 商品搜尋
- 買家與賣家角色分離

### 第二步：技術選型
最初規劃使用 **Next.js 14**（React 全端框架 + TypeScript + Tailwind CSS + Prisma ORM），因為它能夠同時處理前後端，開發效率高。

但使用者要求更進階的後端技術，因此重新評估後選擇：

| 技術 | 選擇原因 |
|------|---------|
| **Rust** | 系統級程式語言，零成本抽象，記憶體安全，執行效能極高 |
| **Axum** | 基於 Tower 生態系，擁有極佳的非同步效能與型別安全路由 |
| **SQLite + sqlx** | 零配置資料庫，編譯期 SQL 檢查（型別安全） |
| **Tera** | Rust 生態中最成熟的模板引擎，語法類似 Jinja2 |
| **JWT + bcrypt** | 輕量級認證方案，無需 Session 儲存 |

### 第三步：開發與測試
開發過程中遇到了一些問題並逐一解決：
1. **Tera 模板語法錯誤：** 使用了 Python 風格的 `[:80]` 切片與 `"%.0f"|format`，Tera 不支援，需改用 `truncate` filter 與直接輸出數值
2. **種子資料未正確寫入：** `cat_map` 的 HashMap 鍵值順序錯誤（id→slug 應為 slug→id），導致商品無法對應到分類
3. **Cookie 型別問題：** `axum-extra` 的 Cookie 型別與 `time::Duration` 的搭配需正確引用

### 第四步：部署
- 完整原始碼推送至 GitHub 倉庫
- 建立 `docs/index.html` 靜態前端版本，透過 GitHub Pages 部署
- 靜態頁面 URL：`https://linyue0096.github.io/E-shop/`

---

## 技術架構總覽

```
┌─────────────────────────────────────────────────────┐
│                  使用者瀏覽器                         │
│       (https://linyue0096.github.io/E-shop/)         │
└────────────────────┬────────────────────────────────┘
                     │ HTTP Request
                     ▼
┌─────────────────────────────────────────────────────┐
│              Rust Axum Web Server                     │
│              (http://localhost:3000)                   │
│                                                       │
│  ┌─────────┐  ┌──────────┐  ┌──────────────────┐    │
│  │ 路由層   │→ │ Handler  │→ │ Tera 模板渲染    │    │
│  │ (Router) │  │ (業務邏輯)│  │ (前端頁面輸出)   │    │
│  └─────────┘  └────┬─────┘  └──────────────────┘    │
│                     │                                 │
│                     ▼                                 │
│  ┌─────────────────────────────────────────────┐    │
│  │            SQLite 資料庫                      │    │
│  │  (users, categories, products, cart_items,   │    │
│  │   orders, order_items, reviews)              │    │
│  └─────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────┘
```

### 請求處理流程

```
瀏覽器發送請求
    │
    ▼
Axum Router 匹配 URL 路徑
    │
    ▼
執行對應的 Handler 函式（async fn）
    │
    ├── 靜態檔案（/static/*）→ tower-http ServeDir 直接回傳
    │
    ├── API 請求（/api/*）→ 查詢資料庫 → 回傳 JSON
    │
    └── 頁面請求（/、/product/*、/cart 等）
        │
        ├── 從 Cookie 讀取 JWT Token → 解析使用者身份
        ├── 查詢 SQLite 資料庫取得資料
        ├── 建立 Tera Context（注入資料）
        ├── 渲染 HTNL 模板
        └── 回傳 HTML Response
    │
    ▼
瀏覽器顯示渲染完畢的頁面
```

---

## 後端說明 (Rust + Axum)

### 4.1 依賴套件與用途

```toml
[dependencies]
axum = { version = "0.8", features = ["macros"] }   # Web 框架，提供路由、Handler 宏
tokio = { version = "1", features = ["full"] }       # 非同步執行環境（async/await）
serde = { version = "1", features = ["derive"] }     # 序列化/反序列化（JSON、表單）
serde_json = "1"                                      # JSON 格式處理
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "chrono"] }  # SQL 操作
tera = "1"                                            # 模板引擎（HTML 渲染）
tower-http = { version = "0.6", features = ["fs", "cors"] }  # 靜態檔案服務、CORS
tower = "0.5"                                         # 中介層抽象層
jsonwebtoken = "9"                                    # JWT 認證 Token 產生與驗證
bcrypt = "0.16"                                       # 密碼雜湊（hash + verify）
uuid = { version = "1", features = ["v4"] }           # 產生唯一識別碼
chrono = { version = "0.4", features = ["serde"] }    # 日期時間處理
tracing = "0.1"                                       # 日誌記錄
tracing-subscriber = "0.3"                            # 日誌輸出格式
axum-extra = { version = "0.10", features = ["cookie"] }  # Cookie 操作擴充
time = { version = "0.3", features = ["macros"] }     # Cookie 有效期 Duration
```

### 4.2 專案結構與各檔案功能

```
ecommerce/
│
├── Cargo.toml                          # Rust 專案設定 + 相依套件
├── Cargo.lock                          # 依賴鎖定檔案（自動產生）
│
├── src/
│   ├── main.rs                         # 主程式：路由註冊、Handler 實作、伺服器啟動
│   ├── db.rs                           # 資料庫初始化：建立表格、寫入種子資料
│   ├── models.rs                       # 資料結構定義：User、Product、Order 等 struct
│   └── auth.rs                         # 認證模組：JWT 產生、驗證、Cookie 讀取
│
├── templates/                          # Tera 模板檔案
│   ├── base.html                       # 共用版型（header、nav、footer）
│   ├── home.html                       # 首頁（分類標籤 + 商品卡片）
│   ├── product.html                    # 商品詳情頁
│   ├── search.html                     # 搜尋結果頁
│   ├── category.html                   # 分類瀏覽頁
│   ├── login.html                      # 登入頁
│   ├── register.html                   # 註冊頁
│   ├── cart.html                       # 購物車頁
│   ├── checkout.html                   # 結帳頁（付款選擇）
│   ├── orders.html                     # 我的訂單列表
│   ├── order_detail.html               # 訂單明細
│   └── seller/                         # 賣家功能模板
│       ├── dashboard.html              # 賣家儀表板
│       ├── products.html               # 商品管理列表
│       ├── product_form.html           # 新增/編輯商品表單
│       ├── orders.html                 # 訂單管理列表
│       └── order_detail.html           # 訂單明細 + 狀態更新
│
├── docs/
│   └── index.html                      # GitHub Pages 靜態版本
│
├── static/                             # 靜態資源目錄（圖片、CSS、JS）
│   ├── css/
│   └── js/
│
└── ecommerce.db                        # SQLite 資料庫檔案（執行後自動產生）
```

#### `src/main.rs` — 主程式核心

此檔案包含以下關鍵部分：

**1. 應用狀態 (AppState)**
```rust
#[derive(Clone)]
struct AppState {
    db: SqlitePool,        // 資料庫連線池
    tmpl: Arc<Tera>,       // 模板引擎（Arc 共享所有權）
    jwt_secret: Arc<String>, // JWT 簽章密鑰
}
```
`AppState` 透過 Axum 的 `State` 提取器傳遞給每個 Handler，讓所有請求處理函式都能存取資料庫與模板。

**2. 路由註冊**
```rust
let app = Router::new()
    .route("/", get(home_page))
    .route("/product/{slug}", get(product_detail))
    .route("/search", get(search_page))
    .route("/category/{slug}", get(category_page))
    .route("/auth/login", get(login_page).post(login_action))
    .route("/auth/register", get(register_page).post(register_action))
    .route("/auth/logout", get(logout))
    .route("/cart", get(cart_page))
    .route("/cart/add", post(add_to_cart))
    // ... 共 30+ 條路由
    .nest_service("/static", ServeDir::new("static"))
    .layer(CorsLayer::permissive())
    .with_state(state);
```
Axum 支援在同一路由上註冊多個 HTTP 方法（如 `get(login_page).post(login_action)`），
使用 `{slug}` 路徑參數進行動態匹配。

**3. Handler 模式**
每個 Handler 都是一個 `async fn`，透過參數注入需要的提取器：
```rust
async fn home_page(
    State(state): State<AppState>,  // 應用狀態
    jar: CookieJar,                 // Cookie（含 JWT）
) -> impl IntoResponse {
    // 1. 從 Cookie 解析使用者身份
    let user = get_user_from_jar(&jar, &state).await;
    // 2. 查詢資料庫
    let categories = sqlx::query_as("SELECT * FROM categories ...").fetch_all(&state.db).await;
    // 3. 建立模板上下文
    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("categories", &categories);
    // 4. 渲染 HTML
    render(&state, "home.html", ctx)
}
```

#### `src/db.rs` — 資料庫初始化

此檔案負責：

1. **連線到 SQLite 資料庫**（自動建立檔案）
2. **建立所有資料表**（使用 `CREATE TABLE IF NOT EXISTS`）
3. **寫入種子資料**（分類、展示商品、Demo 帳號）

#### `src/models.rs` — 資料結構

定義所有與資料庫對應的 Rust 結構體，使用 `sqlx::FromRow` 自動將查詢結果映射為 Rust 型別：

```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductWithSeller {
    pub id: String,
    pub seller_id: String,
    pub category_id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub image_url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub seller_name: String,      // JOIN users 而來
    pub category_name: String,    // JOIN categories 而來
}
```
共用 12 個資料結構，處理不同查詢組合的結果型別。

#### `src/auth.rs` — 認證模組

實作 JWT 的產生與驗證，包含：
- `Claims` 結構：儲存使用者 ID、名稱、Email、角色、到期時間
- `create_jwt()`：登入成功時產生 Token（有效期 7 天）
- `verify_jwt()`：驗證 Token 是否有效
- `get_user_from_jar()`：從請求的 Cookie 中解析使用者身份

### 4.3 資料庫設計（完整欄位與關聯）

系統使用 SQLite 作為資料庫，共有 7 個資料表。

#### users — 使用者
```sql
CREATE TABLE users (
    id          TEXT PRIMARY KEY,           -- UUID 唯一識別
    username    TEXT UNIQUE NOT NULL,       -- 登入名稱
    email       TEXT UNIQUE NOT NULL,       -- 電子郵件
    password    TEXT NOT NULL,              -- bcrypt 雜湊密碼
    role        TEXT NOT NULL DEFAULT 'buyer',  -- 'buyer' 或 'seller'
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### categories — 商品分類
```sql
CREATE TABLE categories (
    id    TEXT PRIMARY KEY,             -- UUID
    name  TEXT UNIQUE NOT NULL,         -- 顯示名稱（如 "Electronics"）
    slug  TEXT UNIQUE NOT NULL          -- URL 用名稱（如 "electronics"）
);
```
預設種子分類：Electronics、Clothing、Home & Garden、Books、Sports、Toys、Food & Beverages、Beauty

#### products — 商品
```sql
CREATE TABLE products (
    id          TEXT PRIMARY KEY,           -- UUID
    seller_id   TEXT NOT NULL REFERENCES users(id),  -- 賣家（外部鍵）
    category_id TEXT NOT NULL REFERENCES categories(id), -- 分類（外部鍵）
    name        TEXT NOT NULL,              -- 商品名稱
    slug        TEXT NOT NULL,              -- URL 用名稱
    description TEXT NOT NULL,              -- 商品描述
    price       REAL NOT NULL,              -- 價格
    stock       INTEGER NOT NULL DEFAULT 0, -- 庫存數量
    image_url   TEXT NOT NULL DEFAULT '',   -- 商品圖片網址
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```
外部鍵約束確保 seller_id 必須存在於 users 表，category_id 必須存在於 categories 表。

#### cart_items — 購物車
```sql
CREATE TABLE cart_items (
    id          TEXT PRIMARY KEY,
    user_id     TEXT NOT NULL REFERENCES users(id),      -- 買家
    product_id  TEXT NOT NULL REFERENCES products(id),   -- 商品
    quantity    INTEGER NOT NULL DEFAULT 1,              -- 數量
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### orders — 訂單
```sql
CREATE TABLE orders (
    id               TEXT PRIMARY KEY,
    buyer_id         TEXT NOT NULL REFERENCES users(id),
    status           TEXT NOT NULL DEFAULT 'pending',  -- 'paid'|'shipped'|'delivered'|'cancelled'
    payment_method   TEXT NOT NULL,                    -- 'cod' 或 'credit_card'
    total            REAL NOT NULL,                    -- 訂單總額
    shipping_address TEXT NOT NULL,                    -- 收貨地址
    created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### order_items — 訂單明細
```sql
CREATE TABLE order_items (
    id         TEXT PRIMARY KEY,
    order_id   TEXT NOT NULL REFERENCES orders(id),    -- 所屬訂單
    product_id TEXT NOT NULL REFERENCES products(id),  -- 商品
    quantity   INTEGER NOT NULL,                       -- 數量
    price      REAL NOT NULL                           -- 下單時價格（避免商品調價影響）
);
```

#### reviews — 商品評價
```sql
CREATE TABLE reviews (
    id         TEXT PRIMARY KEY,
    user_id    TEXT NOT NULL REFERENCES users(id),
    product_id TEXT NOT NULL REFERENCES products(id),
    rating     INTEGER NOT NULL CHECK(rating >= 1 AND rating <= 5),  -- 1~5 星
    comment    TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 表格關聯圖

```
users ──┬──< products (seller_id)      -- 賣家擁有多個商品
        ├──< cart_items (user_id)       -- 買家擁有多個購物車項目
        ├──< orders (buyer_id)          -- 買家擁有多個訂單
        └──< reviews (user_id)          -- 使用者可以評價

categories ──< products                -- 一個分類有多個商品
products ──< cart_items                -- 商品可出現在多個購物車
products ──< order_items               -- 商品可出現在多個訂單
products ──< reviews                   -- 商品可有多個評價
orders ──< order_items                 -- 訂單包含多個明細
```

### 4.4 API 路由大全

#### 公開頁面（無需登入）

| 路由 | 方法 | Handler | 說明 |
|------|------|---------|------|
| `/` | GET | `home_page` | 首頁，顯示所有分類與 20 筆商品 |
| `/product/{slug}` | GET | `product_detail` | 商品詳情，含描述、價格、庫存 |
| `/search?q=&category=` | GET | `search_page` | 關鍵字搜尋（LIKE 比對名稱與描述）+ 分類篩選 |
| `/category/{slug}` | GET | `category_page` | 分類瀏覽，顯示該分類所有商品 |

#### 認證相關

| 路由 | 方法 | Handler | 說明 |
|------|------|---------|------|
| `/auth/login` | GET | `login_page` | 顯示登入表單 |
| `/auth/login` | POST | `login_action` | 驗證帳密→產生 JWT→寫入 Cookie→重新導向 |
| `/auth/register` | GET | `register_page` | 顯示註冊表單 |
| `/auth/register` | POST | `register_action` | 建立帳號→自動登入→重新導向 |
| `/auth/logout` | GET | `logout` | 清除 Cookie→導回首頁 |

#### 買家功能（需登入）

| 路由 | 方法 | Handler | 說明 |
|------|------|---------|------|
| `/cart` | GET | `cart_page` | 購物車列表，含數量調整與價格計算 |
| `/cart/add` | POST | `add_to_cart` | 加入商品（若已存在則增加數量） |
| `/cart/remove/{id}` | POST | `remove_from_cart` | 刪除購物車項目 |
| `/cart/update/{id}` | POST | `update_cart` | 修改數量（設為 0 則自動刪除） |
| `/checkout` | GET | `checkout_page` | 結帳頁面，顯示訂單摘要與付款選擇 |
| `/checkout` | POST | `place_order` | 建立訂單、扣庫存、清空購物車、重新導向到訂單頁 |
| `/orders` | GET | `orders_page` | 我的訂單列表 |
| `/order/{id}` | GET | `order_detail` | 訂單明細（商品、金額、狀態、地址） |

#### 賣家功能（需 seller 角色）

| 路由 | 方法 | Handler | 說明 |
|------|------|---------|------|
| `/seller/dashboard` | GET | `seller_dashboard` | 儀表板：商品數、訂單數、總收入 |
| `/seller/products` | GET | `seller_products` | 商品管理列表（該賣家的商品） |
| `/seller/products/new` | GET | `new_product_form` | 新增商品表單 |
| `/seller/products/new` | POST | `create_product` | 儲存新商品 |
| `/seller/products/{id}/edit` | GET | `edit_product_form` | 編輯商品表單（預填資料） |
| `/seller/products/{id}` | POST | `update_product` | 更新商品資訊 |
| `/seller/products/{id}/delete` | POST | `delete_product` | 刪除商品 |
| `/seller/orders` | GET | `seller_orders` | 訂單列表（含有該賣家商品的訂單） |
| `/seller/order/{id}` | GET | `seller_order_detail` | 訂單詳情 |
| `/seller/order/{id}/status` | POST | `update_order_status` | 更新出貨狀態（paid→shipped→delivered） |

#### API 端點

| 路由 | 方法 | Handler | 說明 |
|------|------|---------|------|
| `/api/products` | GET | `api_products` | 回傳 JSON 格式所有商品列表 |

### 4.5 認證機制（JWT + Cookie）

本系統採用 **無狀態認證** 方式，不使用 Session，而是使用 JWT（JSON Web Token）。

```
登入流程：
┌───────┐         ┌───────────┐         ┌──────┐
│ 使用者 │──帳密──→│ POST /    │──查詢──→│資料庫│
│        │         │ auth/login│         │      │
│        │←─JWT───│ (bcrypt   │←─驗證───│      │
│        │  Cookie │  驗證)    │  通過    │      │
└───────┘         └───────────┘         └──────┘

後續請求：
┌───────┐──Cookie(JWT)──→│ 伺服器驗證 JWT    │
│ 使用者 │                │ • 檢查簽章        │
│        │←──頁面/資料───│ • 檢查是否過期     │
└───────┘                │ • 解析使用者身份   │
                          └──────────────────┘
```

**Token 內容：**
```json
{
  "sub": "使用者UUID",
  "username": "demobuyer",
  "email": "buyer@demo.com",
  "role": "buyer",
  "exp": 1700000000
}
```

**安全措施：**
- 密碼使用 bcrypt 雜湊（成本因子 10）
- JWT 使用 HMAC-SHA256 簽章
- Token 有效期設為 7 天
- Cookie 設定 `path=/` 全域有效

### 4.6 種子資料與初始化

伺服器首次啟動時自動執行以下初始化：

1. 建立 7 個資料表（若不存在）
2. 檢查 categories 是否為空
3. 若為空則寫入 8 個分類 + 2 個 Demo 使用者 + 10 個展示商品

**Demo 帳號：**
- 買家：`demobuyer` / `buyer123`（role: buyer）
- 賣家：`demoseller` / `seller123`（role: seller）

**展示商品（10 筆）：**
| 商品 | 分類 | 價格 | 庫存 |
|------|------|------|------|
| Wireless Headphones | Electronics | NT$ 89.99 | 50 |
| Cotton T-Shirt | Clothing | NT$ 24.99 | 100 |
| Indoor Plant Pot | Home & Garden | NT$ 34.99 | 30 |
| Rust Programming Book | Books | NT$ 49.99 | 20 |
| Yoga Mat | Sports | NT$ 29.99 | 75 |
| Building Blocks Set | Toys | NT$ 39.99 | 60 |
| Organic Green Tea | Food & Beverages | NT$ 19.99 | 200 |
| Moisturizing Face Cream | Beauty | NT$ 44.99 | 40 |
| Smart Watch | Electronics | NT$ 199.99 | 35 |
| Denim Jacket | Clothing | NT$ 79.99 | 45 |

### 4.7 付款流程實作細節

#### 貨到付款（COD — Cash on Delivery）

```
使用者選擇「貨到付款」
    │
    ▼
系統建立訂單（payment_method = "cod"）
    │
    ▼
訂單狀態設為 "paid"（已付款狀態）
    │
    ▼
賣家出貨後，狀態更新為 "shipped"
    │
    ▼
買家收到商品時以現金付款
    │
    ▼
賣家確認送達，狀態更新為 "delivered"
```

**實作方式：** 選擇 COD 時，系統記錄 `payment_method = 'cod'`，
訂單狀態設為 `'paid'` 表示已成立待出貨，實際金流在送貨時發生。

#### 信用卡付款（Credit Card）

```
使用者選擇「信用卡付款」
    │
    ▼
系統建立訂单（payment_method = "credit_card"）
    │
    ▼
（模擬）信用卡授權成功
    │
    ▼
訂單狀態設為 "paid"
    │
    ▼
賣家出貨 → 送達
```

**實作方式：** 目前為模擬刷卡，系統記錄 `payment_method = 'credit_card'`，
與 COD 的差異僅在付款方式標記不同，實際金流串接需整合第三方支付 SDK（如綠界、藍新）。

#### 訂單狀態流程

```
paid ──→ shipped ──→ delivered
  │
  └──→ cancelled（賣家取消）
```

**訂單完成後的庫存扣減邏輯：**
```rust
// 在 place_order handler 中
for item in &items {
    // 扣減庫存
    sqlx::query("UPDATE products SET stock = stock - ? WHERE id = ?")
        .bind(item.quantity)
        .bind(&item.product_id)
        .execute(&state.db)
        .await
        .unwrap_or_default();
}
// 清空購物車
sqlx::query("DELETE FROM cart_items WHERE user_id = ?")
    .bind(&user.id)
    .execute(&state.db)
    .await
    .unwrap_or_default();
```

---

## 前端說明

### 5.1 Tera 模板引擎

Tera 是 Rust 生態中最成熟的模板引擎，語法源自 Jinja2 / Django Templates。

**基礎語法：**
```html
<!-- 變數輸出 -->
{{ user.username }}

<!-- 條件判斷 -->
{% if user %}
  <p>歡迎回來，{{ user.username }}</p>
{% else %}
  <a href="/login">登入</a>
{% endif %}

<!-- 迴圈 -->
{% for product in products %}
  <div class="product-card">{{ product.name }}</div>
{% endfor %}

<!-- Filter -->
{{ product.description | truncate(length=80) }}

<!-- 註解 -->
{# 這是註解，不會輸出 #}
```

**模板繼承：**
所有頁面繼承自 `base.html`，透過 `{% block content %}{% endblock %}` 定義可替換區塊，
達到 DRY（Don't Repeat Yourself）原則。

### 5.2 頁面功能詳解

每個頁面都包含以下層次：

1. **模板渲染：** 後端 Handler 查詢資料庫 → 注入 Context → Tera 渲染 → 回傳 HTML
2. **響應式設計：** 使用 Tailwind CSS 的響應式斷點（`sm:`、`md:`、`lg:`）
3. **圖示系統：** Font Awesome 6（免費圖示庫）
4. **互動效果：** 純 CSS 實作 hover 動畫（無 JavaScript）

### 5.3 共用版型 (base.html)

`base.html` 是整個網站的外殼，所有頁面共用：

**導覽列（Navbar）：**
```html
<nav class="bg-white shadow-md sticky top-0 z-50">
  <!-- 左側：Logo + 搜尋列 -->
  <a href="/"><i class="fas fa-store"></i> E-Shop</a>
  <form action="/search"><input name="q" placeholder="搜尋商品..."></form>

  <!-- 右側：使用者功能 -->
  {% if user %}
    <!-- 已登入：購物車圖示 + 下拉選單（我的訂單、賣家中心、登出） -->
  {% else %}
    <!-- 未登入：登入 + 註冊按鈕 -->
  {% endif %}
</nav>
```

**頁尾（Footer）：**
包含購物指南、客戶服務、聯絡資訊三欄 + 版權聲明。

### 5.4 買家流程頁面

#### 首頁（home.html）
- 上方漸層橫幅：歡迎文字 + 行動呼籲按鈕
- 分類標籤列：所有分類以圓角標籤顯示，點選即跳轉至分類頁
- 商品網格：使用 CSS Grid 響應式排列（1~4 欄），每張卡片包含：
  - 商品圖片（16:9 裁切）
  - 分類標籤（小圓角標）
  - 商品名稱 + 截斷描述（80 字元）
  - 價格（紅色粗體）+ 庫存數字
  - 「加入購物車」按鈕（POST 表單）

#### 商品詳情（product.html）
- 麵包屑導航：首頁 > 分類 > 商品名稱
- 雙欄布局：
  - 左欄：商品圖片
  - 右欄：分類標籤、名稱、賣家、價格（大字）、描述、庫存狀態
- 數量選擇 + 加入購物車按鈕

#### 搜尋（search.html）
- 搜尋表單：文字輸入框 + 分類下拉選單
- 搜尋結果：同首頁商品卡片網格
- 支援無結果提示

#### 分類瀏覽（category.html）
- 分類標籤列（當前分類高亮）
- 該分類商品列表

#### 購物車（cart.html）
- 左欄：購物車項目列表
  - 商品縮圖、名稱、單價
  - 數量輸入框（可修改或設為 0 刪除）
  - 小計金額
  - 刪除按鈕
- 右欄：訂單摘要
  - 各項金額明細
  - 總計（大字紅色）
  - 前往結帳按鈕
  - 繼續購物連結

#### 結帳（checkout.html）
- 左欄：商品明細摘要
- 右欄：結帳表單
  - 收貨地址（textarea）
  - 付款方式選擇（radio button 搭配卡片式 UI）：
    ```
    ┌─────────────────────────────────────────┐
    │ ◎ 貨到付款 (COD)                        │
    │   收到商品時以現金付款                    │
    ├─────────────────────────────────────────┤
    │ ○ 信用卡付款                            │
    │   支援 VISA / Mastercard / JCB          │
    └─────────────────────────────────────────┘
    ```
  - 確認下單按鈕

#### 我的訂單（orders.html）
- 訂卡列表，每張卡片顯示：
  - 訂單編號（前 8 碼）
  - 下單時間
  - 付款方式標籤（綠色 COD / 藍色 信用卡）
  - 訂單狀態標籤（黃色待出貨 / 藍色已出貨 / 綠色已送達 / 紅色已取消）
  - 總金額

#### 訂單明細（order_detail.html）
- 訂單資訊：編號、時間、付款方式、收貨地址
- 商品列表：縮圖、名稱、單價、數量、小計
- 狀態標籤

### 5.5 賣家管理頁面

#### 賣家儀表板（seller/dashboard.html）
- 三欄統計卡片：
  - 商品數量（indigo 配色）
  - 訂單數量（綠色配色）
  - 總收入（紅色配色）
- 快捷按鈕：商品管理、新增商品、訂單管理

#### 商品管理（seller/products.html）
- 表格布局：商品圖片、名稱、分類、價格、庫存、操作（編輯/刪除）
- 頂部「新增商品」按鈕
- 刪除需確認（JavaScript confirm）

#### 新增/編輯商品（seller/product_form.html）
- 表單欄位：名稱、分類（下拉）、描述（textarea）、價格、庫存、圖片網址
- 編輯模式：預填現有資料
- 提交按鈕：建立 / 更新

#### 賣家訂單管理（seller/orders.html + seller/order_detail.html）
- 訂單列表（與買家端類似）
- 訂單明細 + 狀態更新表單：
  ```
  訂單狀態： [已付款 ▼] [更新狀態]
  ```
  可切換：已付款 → 已出貨 → 已送達 → 已取消

### 5.6 GitHub Pages 靜態前端

`docs/index.html` 是一個自包含的靜態網頁，特色：

**技術：**
- 單一 HTML 檔案（無需建置工具）
- Tailwind CSS（CDN 載入）
- Font Awesome（CDN 載入）
- 原生 JavaScript（無框架）

**內容：**
- 導覽列（含學號 111310562 標示）
- 漸層橫幅（技術棧介紹）
- 8 筆展示商品卡片網格（硬編碼資料）
- 頁尾（學號 + 版權）

**限制：**
- 無購物車功能（需要後端）
- 無登入/註冊（需要後端）
- 僅供展示商品列表與網站視覺設計

---

## 本地執行步驟

### 前置需求

1. **安裝 Rust：** 在 [rustup.rs](https://rustup.rs/) 下載安裝
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   或 Windows 下載 [rustup-init.exe](https://win.rustup.rs/)

2. **確認安裝成功：**
   ```bash
   rustc --version   # 應顯示 rustc 1.x.x
   cargo --version   # 應顯示 cargo 1.x.x
   ```

### 啟動伺服器

```bash
# 進入專案目錄
cd C:\yy\_wp\期中\ecommerce

# 編譯並執行（首次編譯需要下載依賴，約 2~5 分鐘）
cargo run

# 看到以下訊息表示啟動成功：
# Server running on http://0.0.0.0:3000
```

### 使用方式

1. 開啟瀏覽器，前往 `http://localhost:3000`
2. 使用 Demo 帳號登入體驗完整功能
3. 或註冊新帳號（可選買家或賣家角色）

### 重新設置資料庫

若想恢復原始資料，刪除資料庫檔案後重新啟動即可：
```bash
# 在專案目錄下執行
rm ecommerce.db
cargo run
```
伺服器會自動重新建立資料表並寫入種子資料。

---

## Demo 帳號

| 角色 | 用戶名 | 密碼 | 說明 |
|------|--------|------|------|
| 🛒 買家 | `demobuyer` | `buyer123` | 可瀏覽商品、購物車、結帳、查詢訂單 |
| 📦 賣家 | `demoseller` | `seller123` | 可管理商品、檢視訂單、更新出貨狀態 |

---

## 開發工具與環境

| 類別 | 工具 | 版本 |
|------|------|------|
| 作業系統 | Windows 11 + WSL2 (Ubuntu) | - |
| 編輯器 | VS Code | 最新版 |
| 語言 | Rust | 1.93.1 |
| 建置工具 | Cargo | 1.93.1 |
| Web 框架 | Axum | 0.8 |
| 資料庫 | SQLite | 3.x |
| 版控 | Git + GitHub | - |
| API 測試 | curl / HTTPie | - |
| 前端樣式 | Tailwind CSS (CDN) | 3.x |
| 前端圖示 | Font Awesome (CDN) | 6.5.1 |

---

## 版本紀錄

| 日期 | 版本 | 內容 |
|------|------|------|
| 2024-06-05 | v0.1.0 | 初始開發：建立 Rust Axum 專案、SQLite 資料庫、Tera 模板、JWT 認證 |
| 2024-06-05 | v0.1.1 | Bug 修正：Tera 模板語法相容性修正、種子資料 HashMap 方向修正 |
| 2024-06-05 | v0.1.2 | 功能完成：所有買家/賣家頁面測試通過 |
| 2024-06-05 | v0.2.0 | 部署：推送 GitHub、建立 GitHub Pages 靜態前端 |

---