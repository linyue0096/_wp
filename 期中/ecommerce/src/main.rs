use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tera::{Context, Tera};
use tower_http::{cors::CorsLayer, services::ServeDir};
use uuid::Uuid;

mod db;
use db::init_db;

mod auth;
use auth::*;

mod models;
use models::*;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
    tmpl: Arc<Tera>,
    jwt_secret: Arc<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = init_db().await;
    let mut tera = Tera::new("templates/**/*.html").expect("Tera templates");
    tera.autoescape_on(Vec::new());
    let jwt_secret = "ecommerce-secret-key-2024-change-in-production".to_string();

    let state = AppState {
        db,
        tmpl: Arc::new(tera),
        jwt_secret: Arc::new(jwt_secret),
    };

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
        .route("/cart/remove/{id}", post(remove_from_cart))
        .route("/cart/update/{id}", post(update_cart))
        .route("/checkout", get(checkout_page).post(place_order))
        .route("/orders", get(orders_page))
        .route("/order/{id}", get(order_detail))
        .route("/seller/dashboard", get(seller_dashboard))
        .route("/seller/products", get(seller_products))
        .route("/seller/products/new", get(new_product_form).post(create_product))
        .route("/seller/products/{id}/edit", get(edit_product_form))
        .route("/seller/products/{id}", post(update_product))
        .route("/seller/products/{id}/delete", post(delete_product))
        .route("/seller/orders", get(seller_orders))
        .route("/seller/order/{id}", get(seller_order_detail))
        .route("/seller/order/{id}/status", post(update_order_status))
        .route("/api/products", get(api_products))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn render(state: &AppState, template: &str, ctx: Context) -> Html<String> {
    let html = state.tmpl.render(template, &ctx).unwrap_or_else(|e| {
        format!("<h1>Template error: {}</h1><pre>{:?}</pre>", template, e)
    });
    Html(html)
}

async fn home_page(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = get_user_from_jar(&jar, &state).await;
    let categories: Vec<Category> = sqlx::query_as("SELECT * FROM categories ORDER BY name")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    let products: Vec<ProductWithSeller> = sqlx::query_as(
        r#"SELECT p.*, u.username as seller_name, c.name as category_name
           FROM products p
           JOIN users u ON p.seller_id = u.id
           JOIN categories c ON p.category_id = c.id
           WHERE p.stock > 0
           ORDER BY p.created_at DESC
           LIMIT 20"#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("categories", &categories);
    ctx.insert("products", &products);
    render(&state, "home.html", ctx)
}

async fn product_detail(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let user = get_user_from_jar(&jar, &state).await;
    let product: Option<ProductWithSeller> = sqlx::query_as(
        r#"SELECT p.*, u.username as seller_name, c.name as category_name
           FROM products p
           JOIN users u ON p.seller_id = u.id
           JOIN categories c ON p.category_id = c.id
           WHERE p.slug = ?"#,
    )
    .bind(&slug)
    .fetch_optional(&state.db)
    .await
    .unwrap_or_default();

    match product {
        Some(p) => {
            let mut ctx = Context::new();
            ctx.insert("user", &user);
            ctx.insert("product", &p);
            ctx.insert("in_cart", &false);
            render(&state, "product.html", ctx)
        }
        None => Html("<h1>Product not found</h1>".to_string()),
    }
}

async fn search_page(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let user = get_user_from_jar(&jar, &state).await;
    let q = params.get("q").cloned().unwrap_or_default();
    let cat = params.get("category").cloned().unwrap_or_default();
    let categories: Vec<Category> = sqlx::query_as("SELECT * FROM categories ORDER BY name")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    let products = if !q.is_empty() && !cat.is_empty() {
        sqlx::query_as::<_, ProductWithSeller>(
            r#"SELECT p.*, u.username as seller_name, c.name as category_name
               FROM products p
               JOIN users u ON p.seller_id = u.id
               JOIN categories c ON p.category_id = c.id
               WHERE (p.name LIKE ? OR p.description LIKE ?) AND c.slug = ? AND p.stock > 0
               ORDER BY p.created_at DESC"#,
        )
        .bind(format!("%{}%", q))
        .bind(format!("%{}%", q))
        .bind(&cat)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default()
    } else if !q.is_empty() {
        sqlx::query_as::<_, ProductWithSeller>(
            r#"SELECT p.*, u.username as seller_name, c.name as category_name
               FROM products p
               JOIN users u ON p.seller_id = u.id
               JOIN categories c ON p.category_id = c.id
               WHERE (p.name LIKE ? OR p.description LIKE ?) AND p.stock > 0
               ORDER BY p.created_at DESC"#,
        )
        .bind(format!("%{}%", q))
        .bind(format!("%{}%", q))
        .fetch_all(&state.db)
        .await
        .unwrap_or_default()
    } else if !cat.is_empty() {
        sqlx::query_as::<_, ProductWithSeller>(
            r#"SELECT p.*, u.username as seller_name, c.name as category_name
               FROM products p
               JOIN users u ON p.seller_id = u.id
               JOIN categories c ON p.category_id = c.id
               WHERE c.slug = ? AND p.stock > 0
               ORDER BY p.created_at DESC"#,
        )
        .bind(&cat)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default()
    } else {
        vec![]
    };

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("products", &products);
    ctx.insert("categories", &categories);
    ctx.insert("q", &q);
    ctx.insert("selected_category", &cat);
    render(&state, "search.html", ctx)
}

async fn category_page(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let user = get_user_from_jar(&jar, &state).await;
    let category: Option<Category> = sqlx::query_as("SELECT * FROM categories WHERE slug = ?")
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .unwrap_or_default();

    match category {
        Some(cat) => {
            let products: Vec<ProductWithSeller> = sqlx::query_as(
                r#"SELECT p.*, u.username as seller_name, c.name as category_name
                   FROM products p
                   JOIN users u ON p.seller_id = u.id
                   JOIN categories c ON p.category_id = c.id
                   WHERE c.slug = ? AND p.stock > 0
                   ORDER BY p.created_at DESC"#,
            )
            .bind(&slug)
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();

            let categories: Vec<Category> =
                sqlx::query_as("SELECT * FROM categories ORDER BY name")
                    .fetch_all(&state.db)
                    .await
                    .unwrap_or_default();

            let mut ctx = Context::new();
            ctx.insert("user", &user);
            ctx.insert("products", &products);
            ctx.insert("categories", &categories);
            ctx.insert("category", &cat);
            render(&state, "category.html", ctx)
        }
        None => Html("<h1>Category not found</h1>".to_string()),
    }
}

async fn login_page(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = get_user_from_jar(&jar, &state).await;
    if user.is_some() {
        return Redirect::to("/").into_response();
    }
    let mut ctx = Context::new();
    ctx.insert("user", &None::<UserPublic>);
    render(&state, "login.html", ctx).into_response()
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login_action(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE username = ?")
        .bind(&form.username)
        .fetch_optional(&state.db)
        .await
        .unwrap_or_default();

    match user {
        Some(u) if bcrypt::verify(&form.password, &u.password).unwrap_or(false) => {
            let token = create_jwt(&u, &state.jwt_secret);
            let cookie = Cookie::build(("token", token))
                .path("/")
                .max_age(time::Duration::days(7));
            let jar = jar.add(cookie);
            (jar, Redirect::to("/")).into_response()
        }
        _ => {
            let mut ctx = Context::new();
            ctx.insert("user", &None::<UserPublic>);
            ctx.insert("error", "Invalid username or password");
            render(&state, "login.html", ctx).into_response()
        }
    }
}

async fn register_page(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = get_user_from_jar(&jar, &state).await;
    if user.is_some() {
        return Redirect::to("/").into_response();
    }
    let mut ctx = Context::new();
    ctx.insert("user", &None::<UserPublic>);
    render(&state, "register.html", ctx).into_response()
}

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
    role: String,
}

async fn register_action(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    if form.password != form.confirm_password {
        let mut ctx = Context::new();
        ctx.insert("user", &None::<UserPublic>);
        ctx.insert("error", "Passwords do not match");
        return render(&state, "register.html", ctx).into_response();
    }

    let existing = sqlx::query::<sqlx::Sqlite>(
        "SELECT id FROM users WHERE username = ? OR email = ?",
    )
    .bind(&form.username)
    .bind(&form.email)
    .fetch_optional(&state.db)
    .await
    .unwrap_or_default();

    if existing.is_some() {
        let mut ctx = Context::new();
        ctx.insert("user", &None::<UserPublic>);
        ctx.insert("error", "Username or email already exists");
        return render(&state, "register.html", ctx).into_response();
    }

    let hashed = bcrypt::hash(&form.password, 10).unwrap();
    let id = Uuid::new_v4().to_string();
    let role = if form.role == "seller" { "seller" } else { "buyer" };

    sqlx::query(
        "INSERT INTO users (id, username, email, password, role) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&form.username)
    .bind(&form.email)
    .bind(&hashed)
    .bind(role)
    .execute(&state.db)
    .await
    .unwrap_or_default();

    let user = UserPublic {
        id,
        username: form.username,
        email: form.email,
        role: role.to_string(),
    };
    let token = create_jwt_raw(&user, &state.jwt_secret);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .max_age(time::Duration::days(7));
    let jar = jar.add(cookie);
    (jar, Redirect::to("/")).into_response()
}

async fn logout(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build(("token", "")).path("/");
    let jar = jar.remove(cookie);
    (jar, Redirect::to("/"))
}

async fn cart_page(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    let items: Vec<CartItemWithProduct> = sqlx::query_as(
        r#"SELECT ci.*, p.name as product_name, p.price, p.image_url, p.stock, p.slug as product_slug
           FROM cart_items ci
           JOIN products p ON ci.product_id = p.id
           WHERE ci.user_id = ?
           ORDER BY ci.created_at DESC"#,
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let categories: Vec<Category> = sqlx::query_as("SELECT * FROM categories ORDER BY name")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    let total: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("items", &items);
    ctx.insert("categories", &categories);
    ctx.insert("total", &total);
    render(&state, "cart.html", ctx).into_response()
}

#[derive(Deserialize)]
struct AddToCartForm {
    product_id: String,
    quantity: i32,
}

async fn add_to_cart(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<AddToCartForm>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    let existing: Option<CartItem> = sqlx::query_as(
        "SELECT * FROM cart_items WHERE user_id = ? AND product_id = ?",
    )
    .bind(&user.id)
    .bind(&form.product_id)
    .fetch_optional(&state.db)
    .await
    .unwrap_or_default();

    if let Some(item) = existing {
        sqlx::query("UPDATE cart_items SET quantity = quantity + ? WHERE id = ?")
            .bind(form.quantity)
            .bind(&item.id)
            .execute(&state.db)
            .await
            .unwrap_or_default();
    } else {
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO cart_items (id, user_id, product_id, quantity) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&user.id)
            .bind(&form.product_id)
            .bind(form.quantity)
            .execute(&state.db)
            .await
            .unwrap_or_default();
    }

    Redirect::to("/cart").into_response()
}

async fn remove_from_cart(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    sqlx::query("DELETE FROM cart_items WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user.id)
        .execute(&state.db)
        .await
        .unwrap_or_default();

    Redirect::to("/cart").into_response()
}

async fn update_cart(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
    Form(form): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    if let Some(qty_str) = form.get("quantity") {
        if let Ok(qty) = qty_str.parse::<i32>() {
            if qty <= 0 {
                sqlx::query("DELETE FROM cart_items WHERE id = ? AND user_id = ?")
                    .bind(&id)
                    .bind(&user.id)
                    .execute(&state.db)
                    .await
                    .unwrap_or_default();
            } else {
                sqlx::query("UPDATE cart_items SET quantity = ? WHERE id = ? AND user_id = ?")
                    .bind(qty)
                    .bind(&id)
                    .bind(&user.id)
                    .execute(&state.db)
                    .await
                    .unwrap_or_default();
            }
        }
    }

    Redirect::to("/cart").into_response()
}

async fn checkout_page(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    let items: Vec<CartItemWithProduct> = sqlx::query_as(
        r#"SELECT ci.*, p.name as product_name, p.price, p.image_url, p.stock, p.slug as product_slug
           FROM cart_items ci
           JOIN products p ON ci.product_id = p.id
           WHERE ci.user_id = ?
           ORDER BY ci.created_at DESC"#,
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    if items.is_empty() {
        return Redirect::to("/cart").into_response();
    }

    let total: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("items", &items);
    ctx.insert("total", &total);
    render(&state, "checkout.html", ctx).into_response()
}

#[derive(Deserialize)]
struct CheckoutForm {
    shipping_address: String,
    payment_method: String,
}

async fn place_order(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<CheckoutForm>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    let items: Vec<CartItemWithProduct> = sqlx::query_as(
        r#"SELECT ci.*, p.name as product_name, p.price, p.image_url, p.stock, p.slug as product_slug
           FROM cart_items ci
           JOIN products p ON ci.product_id = p.id
           WHERE ci.user_id = ?
           ORDER BY ci.created_at DESC"#,
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    if items.is_empty() {
        return Redirect::to("/cart").into_response();
    }

    let total: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
    let order_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO orders (id, buyer_id, status, payment_method, total, shipping_address) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&order_id)
    .bind(&user.id)
    .bind("paid")
    .bind(&form.payment_method)
    .bind(total)
    .bind(&form.shipping_address)
    .execute(&state.db)
    .await
    .unwrap_or_default();

    for item in &items {
        sqlx::query(
            "INSERT INTO order_items (id, order_id, product_id, quantity, price) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&order_id)
        .bind(&item.product_id)
        .bind(item.quantity)
        .bind(item.price)
        .execute(&state.db)
        .await
        .unwrap_or_default();

        sqlx::query("UPDATE products SET stock = stock - ? WHERE id = ?")
            .bind(item.quantity)
            .bind(&item.product_id)
            .execute(&state.db)
            .await
            .unwrap_or_default();
    }

    sqlx::query("DELETE FROM cart_items WHERE user_id = ?")
        .bind(&user.id)
        .execute(&state.db)
        .await
        .unwrap_or_default();

    Redirect::to(&format!("/order/{}", order_id)).into_response()
}

async fn orders_page(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    let orders: Vec<OrderDetail> = sqlx::query_as(
        r#"SELECT o.*, u.username as buyer_name
           FROM orders o
           JOIN users u ON o.buyer_id = u.id
           WHERE o.buyer_id = ?
           ORDER BY o.created_at DESC"#,
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("orders", &orders);
    render(&state, "orders.html", ctx).into_response()
}

async fn order_detail(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };

    let order: Option<OrderDetail> = sqlx::query_as(
        r#"SELECT o.*, u.username as buyer_name
           FROM orders o
           JOIN users u ON o.buyer_id = u.id
           WHERE o.id = ? AND o.buyer_id = ?"#,
    )
    .bind(&id)
    .bind(&user.id)
    .fetch_optional(&state.db)
    .await
    .unwrap_or_default();

    match order {
        Some(o) => {
            let items: Vec<OrderItemWithProduct> = sqlx::query_as(
                r#"SELECT oi.*, p.name as product_name, p.image_url, p.slug as product_slug
                   FROM order_items oi
                   JOIN products p ON oi.product_id = p.id
                   WHERE oi.order_id = ?"#,
            )
            .bind(&o.id)
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();

            let mut ctx = Context::new();
            ctx.insert("user", &user);
            ctx.insert("order", &o);
            ctx.insert("items", &items);
            render(&state, "order_detail.html", ctx).into_response()
        }
        None => Html("<h1>Order not found</h1>".to_string()).into_response(),
    }
}

async fn seller_dashboard(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    let product_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM products WHERE seller_id = ?",
    )
    .bind(&user.id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0,));

    let order_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM orders o
           JOIN order_items oi ON o.id = oi.order_id
           JOIN products p ON oi.product_id = p.id
           WHERE p.seller_id = ?"#,
    )
    .bind(&user.id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0,));

    let revenue: Option<(f64,)> = sqlx::query_as(
        r#"SELECT COALESCE(SUM(oi.price * oi.quantity), 0) FROM orders o
           JOIN order_items oi ON o.id = oi.order_id
           JOIN products p ON oi.product_id = p.id
           WHERE p.seller_id = ? AND o.status = 'paid'"#,
    )
    .bind(&user.id)
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("product_count", &product_count.0);
    ctx.insert("order_count", &order_count.0);
    ctx.insert("revenue", &revenue.unwrap_or((0.0,)).0);
    render(&state, "seller/dashboard.html", ctx).into_response()
}

async fn seller_products(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    let products: Vec<ProductWithCategory> = sqlx::query_as(
        r#"SELECT p.*, c.name as category_name
           FROM products p
           JOIN categories c ON p.category_id = c.id
           WHERE p.seller_id = ?
           ORDER BY p.created_at DESC"#,
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("products", &products);
    render(&state, "seller/products.html", ctx).into_response()
}

async fn new_product_form(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    let categories: Vec<Category> = sqlx::query_as("SELECT * FROM categories ORDER BY name")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("categories", &categories);
    ctx.insert("product", &None::<Product>);
    render(&state, "seller/product_form.html", ctx).into_response()
}

#[derive(Deserialize)]
struct ProductForm {
    name: String,
    category_id: String,
    description: String,
    price: f64,
    stock: i32,
    image_url: String,
}

async fn create_product(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<ProductForm>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    let id = Uuid::new_v4().to_string();
    let slug = form.name.to_lowercase().replace(' ', "-") + "-" + &Uuid::new_v4().to_string()[..8];

    sqlx::query(
        "INSERT INTO products (id, seller_id, category_id, name, slug, description, price, stock, image_url) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&user.id)
    .bind(&form.category_id)
    .bind(&form.name)
    .bind(&slug)
    .bind(&form.description)
    .bind(form.price)
    .bind(form.stock)
    .bind(&form.image_url)
    .execute(&state.db)
    .await
    .unwrap_or_default();

    Redirect::to("/seller/products").into_response()
}

async fn edit_product_form(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    let product: Option<Product> = sqlx::query_as(
        "SELECT * FROM products WHERE id = ? AND seller_id = ?",
    )
    .bind(&id)
    .bind(&user.id)
    .fetch_optional(&state.db)
    .await
    .unwrap_or_default();

    let categories: Vec<Category> = sqlx::query_as("SELECT * FROM categories ORDER BY name")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    match product {
        Some(p) => {
            let mut ctx = Context::new();
            ctx.insert("user", &user);
            ctx.insert("categories", &categories);
            ctx.insert("product", &Some(p));
            render(&state, "seller/product_form.html", ctx).into_response()
        }
        None => Html("<h1>Product not found</h1>".to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct UpdateProductForm {
    name: String,
    category_id: String,
    description: String,
    price: f64,
    stock: i32,
    image_url: String,
}

async fn update_product(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
    Form(form): Form<UpdateProductForm>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    sqlx::query(
        "UPDATE products SET name=?, category_id=?, description=?, price=?, stock=?, image_url=? WHERE id=? AND seller_id=?",
    )
    .bind(&form.name)
    .bind(&form.category_id)
    .bind(&form.description)
    .bind(form.price)
    .bind(form.stock)
    .bind(&form.image_url)
    .bind(&id)
    .bind(&user.id)
    .execute(&state.db)
    .await
    .unwrap_or_default();

    Redirect::to("/seller/products").into_response()
}

async fn delete_product(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    sqlx::query("DELETE FROM products WHERE id = ? AND seller_id = ?")
        .bind(&id)
        .bind(&user.id)
        .execute(&state.db)
        .await
        .unwrap_or_default();

    Redirect::to("/seller/products").into_response()
}

async fn seller_orders(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    let orders: Vec<SellerOrder> = sqlx::query_as(
        r#"SELECT DISTINCT o.id, o.buyer_id, o.status, o.payment_method, o.total, o.shipping_address, o.created_at, u.username as buyer_name
           FROM orders o
           JOIN order_items oi ON o.id = oi.order_id
           JOIN products p ON oi.product_id = p.id
           JOIN users u ON o.buyer_id = u.id
           WHERE p.seller_id = ?
           ORDER BY o.created_at DESC"#,
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("orders", &orders);
    render(&state, "seller/orders.html", ctx).into_response()
}

async fn seller_order_detail(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    let order: Option<SellerOrder> = sqlx::query_as(
        r#"SELECT DISTINCT o.id, o.buyer_id, o.status, o.payment_method, o.total, o.shipping_address, o.created_at, u.username as buyer_name
           FROM orders o
           JOIN order_items oi ON o.id = oi.order_id
           JOIN products p ON oi.product_id = p.id
           JOIN users u ON o.buyer_id = u.id
           WHERE o.id = ? AND p.seller_id = ?"#,
    )
    .bind(&id)
    .bind(&user.id)
    .fetch_optional(&state.db)
    .await
    .unwrap_or_default();

    match order {
        Some(o) => {
            let items: Vec<OrderItemWithProduct> = sqlx::query_as(
                r#"SELECT oi.*, p.name as product_name, p.image_url, p.slug as product_slug
                   FROM order_items oi
                   JOIN products p ON oi.product_id = p.id
                   WHERE oi.order_id = ?"#,
            )
            .bind(&o.id)
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();

            let mut ctx = Context::new();
            ctx.insert("user", &user);
            ctx.insert("order", &o);
            ctx.insert("items", &items);
            render(&state, "seller/order_detail.html", ctx).into_response()
        }
        None => Html("<h1>Order not found</h1>".to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct UpdateStatusForm {
    status: String,
}

async fn update_order_status(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
    Form(form): Form<UpdateStatusForm>,
) -> impl IntoResponse {
    let user = match get_user_from_jar(&jar, &state).await {
        Some(u) if u.role == "seller" => u,
        _ => return Redirect::to("/auth/login").into_response(),
    };

    sqlx::query(
        r#"UPDATE orders SET status = ? WHERE id = ? AND id IN (
            SELECT DISTINCT oi.order_id FROM order_items oi
            JOIN products p ON oi.product_id = p.id
            WHERE p.seller_id = ?
        )"#,
    )
    .bind(&form.status)
    .bind(&id)
    .bind(&user.id)
    .execute(&state.db)
    .await
    .unwrap_or_default();

    Redirect::to(&format!("/seller/order/{}", id)).into_response()
}

async fn api_products(State(state): State<AppState>) -> impl IntoResponse {
    let products: Vec<ProductWithSeller> = sqlx::query_as(
        r#"SELECT p.*, u.username as seller_name, c.name as category_name
           FROM products p
           JOIN users u ON p.seller_id = u.id
           JOIN categories c ON p.category_id = c.id
           WHERE p.stock > 0
           ORDER BY p.created_at DESC"#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    axum::Json(products)
}
