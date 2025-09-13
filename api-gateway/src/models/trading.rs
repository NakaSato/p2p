use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::schema::types::{OrderType, OrderSide, OrderStatus};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TradingOrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub energy_amount: rust_decimal::Decimal,
    pub price_per_kwh: rust_decimal::Decimal,
    pub filled_amount: rust_decimal::Decimal,
    pub status: OrderStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub filled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub energy_amount: rust_decimal::Decimal,
    pub price_per_kwh: rust_decimal::Decimal,
    pub order_type: OrderType,
    pub expiry_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub current_epoch: u64,
    pub epoch_start_time: DateTime<Utc>,
    pub epoch_end_time: DateTime<Utc>,
    pub status: String,
    pub order_book: OrderBook,
    pub recent_trades: Vec<TradeExecution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBook {
    pub sell_orders: Vec<TradingOrder>,
    pub buy_orders: Vec<TradingOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeExecution {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub energy_amount: rust_decimal::Decimal,
    pub price_per_kwh: rust_decimal::Decimal,
    pub total_price: rust_decimal::Decimal,
    pub executed_at: DateTime<Utc>,
}