use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EnergyReading {
    pub id: Option<Uuid>,
    pub meter_id: String,
    pub timestamp: DateTime<Utc>,
    pub energy_generated: f64,
    pub energy_consumed: f64,
    pub solar_irradiance: Option<f64>,
    pub temperature: Option<f64>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyReadingSubmission {
    pub meter_id: String,
    pub timestamp: DateTime<Utc>,
    pub energy_generated: f64,
    pub energy_consumed: f64,
    pub solar_irradiance: Option<f64>,
    pub temperature: Option<f64>,
    pub engineering_authority_signature: String,
    pub metadata: Option<EnergyMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyMetadata {
    pub location: String,
    pub device_type: String,
    pub weather_conditions: Option<String>,
}