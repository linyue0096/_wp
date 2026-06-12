#![allow(dead_code)]
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPublic {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
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
}

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
    pub seller_name: String,
    pub category_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductWithCategory {
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
    pub category_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartItem {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartItemWithProduct {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: Option<NaiveDateTime>,
    pub product_name: String,
    pub price: f64,
    pub image_url: String,
    pub stock: i32,
    pub product_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: String,
    pub buyer_id: String,
    pub status: String,
    pub payment_method: String,
    pub total: f64,
    pub shipping_address: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderDetail {
    pub id: String,
    pub buyer_id: String,
    pub status: String,
    pub payment_method: String,
    pub total: f64,
    pub shipping_address: String,
    pub created_at: Option<NaiveDateTime>,
    pub buyer_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItemWithProduct {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub price: f64,
    pub product_name: String,
    pub image_url: String,
    pub product_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SellerOrder {
    pub id: String,
    pub buyer_id: String,
    pub status: String,
    pub payment_method: String,
    pub total: f64,
    pub shipping_address: String,
    pub created_at: Option<NaiveDateTime>,
    pub buyer_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub rating: i32,
    pub comment: String,
    pub created_at: Option<NaiveDateTime>,
}
