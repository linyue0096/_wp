use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:ecommerce.db?mode=rwc")
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'buyer',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"#,
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            slug TEXT UNIQUE NOT NULL
        )"#,
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            seller_id TEXT NOT NULL REFERENCES users(id),
            category_id TEXT NOT NULL REFERENCES categories(id),
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0,
            image_url TEXT NOT NULL DEFAULT '',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"#,
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS cart_items (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id),
            product_id TEXT NOT NULL REFERENCES products(id),
            quantity INTEGER NOT NULL DEFAULT 1,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"#,
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            buyer_id TEXT NOT NULL REFERENCES users(id),
            status TEXT NOT NULL DEFAULT 'pending',
            payment_method TEXT NOT NULL,
            total REAL NOT NULL,
            shipping_address TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"#,
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS order_items (
            id TEXT PRIMARY KEY,
            order_id TEXT NOT NULL REFERENCES orders(id),
            product_id TEXT NOT NULL REFERENCES products(id),
            quantity INTEGER NOT NULL,
            price REAL NOT NULL
        )"#,
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS reviews (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id),
            product_id TEXT NOT NULL REFERENCES products(id),
            rating INTEGER NOT NULL CHECK(rating >= 1 AND rating <= 5),
            comment TEXT NOT NULL DEFAULT '',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"#,
    )
    .execute(&db)
    .await
    .unwrap();

    // Seed default categories if empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
        .fetch_one(&db)
        .await
        .unwrap_or((0,));

    if count.0 == 0 {
        let categories = vec![
            ("Electronics", "electronics"),
            ("Clothing", "clothing"),
            ("Home & Garden", "home-garden"),
            ("Books", "books"),
            ("Sports", "sports"),
            ("Toys", "toys"),
            ("Food & Beverages", "food-beverages"),
            ("Beauty", "beauty"),
        ];

        for (name, slug) in categories {
            sqlx::query("INSERT INTO categories (id, name, slug) VALUES (?, ?, ?)")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(name)
                .bind(slug)
                .execute(&db)
                .await
                .unwrap();
        }

        // Seed a demo seller
        let seller_id = uuid::Uuid::new_v4().to_string();
        let hashed = bcrypt::hash("seller123", 10).unwrap();
        sqlx::query(
            "INSERT OR IGNORE INTO users (id, username, email, password, role) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&seller_id)
        .bind("demoseller")
        .bind("seller@demo.com")
        .bind(&hashed)
        .bind("seller")
        .execute(&db)
        .await
        .unwrap();

        // Seed a demo buyer
        let buyer_id = uuid::Uuid::new_v4().to_string();
        let hashed = bcrypt::hash("buyer123", 10).unwrap();
        sqlx::query(
            "INSERT OR IGNORE INTO users (id, username, email, password, role) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&buyer_id)
        .bind("demobuyer")
        .bind("buyer@demo.com")
        .bind(&hashed)
        .bind("buyer")
        .execute(&db)
        .await
        .unwrap();

        // Seed demo products
        let categories: Vec<(String, String)> = sqlx::query_as::<_, (String, String)>(
            "SELECT id, slug FROM categories",
        )
        .fetch_all(&db)
        .await
        .unwrap_or_default();

        let cat_map: std::collections::HashMap<String, String> =
            categories.into_iter().map(|(id, slug)| (slug, id)).collect();

        let demo_products = vec![
            ("Wireless Headphones", "electronics", 89.99, 50, "High-quality wireless headphones with noise cancellation", "https://picsum.photos/seed/headphones/400/400"),
            ("Cotton T-Shirt", "clothing", 24.99, 100, "Comfortable 100% cotton t-shirt, available in multiple colors", "https://picsum.photos/seed/tshirt/400/400"),
            ("Indoor Plant Pot", "home-garden", 34.99, 30, "Modern ceramic plant pot with drainage system", "https://picsum.photos/seed/plantpot/400/400"),
            ("Rust Programming Book", "books", 49.99, 20, "Comprehensive guide to Rust programming language", "https://picsum.photos/seed/rustbook/400/400"),
            ("Yoga Mat", "sports", 29.99, 75, "Non-slip yoga mat with carrying strap", "https://picsum.photos/seed/yogamat/400/400"),
            ("Building Blocks Set", "toys", 39.99, 60, "500-piece building blocks set for creative kids", "https://picsum.photos/seed/blocks/400/400"),
            ("Organic Green Tea", "food-beverages", 19.99, 200, "Premium organic green tea from Japan", "https://picsum.photos/seed/greentea/400/400"),
            ("Moisturizing Face Cream", "beauty", 44.99, 40, "Natural moisturizing face cream with vitamin E", "https://picsum.photos/seed/facecream/400/400"),
            ("Smart Watch", "electronics", 199.99, 35, "Feature-rich smartwatch with health tracking", "https://picsum.photos/seed/smartwatch/400/400"),
            ("Denim Jacket", "clothing", 79.99, 45, "Classic denim jacket with modern fit", "https://picsum.photos/seed/denim/400/400"),
        ];

        for (name, cat_slug, price, stock, desc, img) in demo_products {
            if let Some(cat_id) = cat_map.get(cat_slug) {
                let id = uuid::Uuid::new_v4().to_string();
                let slug = name.to_lowercase().replace(' ', "-") + "-" + &uuid::Uuid::new_v4().to_string()[..8];
                sqlx::query(
                    "INSERT INTO products (id, seller_id, category_id, name, slug, description, price, stock, image_url) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&id)
                .bind(&seller_id)
                .bind(cat_id)
                .bind(name)
                .bind(&slug)
                .bind(desc)
                .bind(price)
                .bind(stock)
                .bind(img)
                .execute(&db)
                .await
                .unwrap();
            }
        }
    }

    db
}
