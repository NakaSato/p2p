use serde::{Deserialize, Serialize};

// User models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub role: String, // "producer", "consumer", "prosumer"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

// Meter models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMeterRequest {
    pub user_id: String,
    pub meter_type: String, // "smart_meter", "consumption_meter", "production_meter"
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateMeterRequest {
    pub location: Option<String>,
    pub status: Option<String>, // "active", "inactive", "maintenance"
}

// Market/Order models
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderRequest {
    pub user_id: String,
    pub order_type: String, // "buy", "sell"
    pub amount: String, // e.g., "10.5 kWh"
    pub price: String, // e.g., "0.12 USD/kWh"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateOrderRequest {
    pub amount: Option<String>,
    pub price: Option<String>,
}
