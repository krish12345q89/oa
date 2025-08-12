use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[schema(example = json!({
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "permit_number": "PERMIT-12345",
    "card_ending": 1234,
    "total_paid": 100.0,
    "date": "2023-01-01T00:00:00Z",
    "receipt_no": 987654,
    "address": "123 Main St, City",
    "version": "v1.0.0",
    "created_at": "2023-01-01T00:00:00Z",
    "updated_at": "2023-01-01T00:00:00Z"
}))]
pub struct Application {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,

    #[schema(example = "PERMIT-12345")]
    pub permit_number: String,

    #[schema(example = 1234)]
    pub card_ending: i64,

    #[schema(example = 100.0)]
    pub total_paid: f64,

    #[schema(example = "2023-01-01T00:00:00Z")]
    pub date: DateTime<Utc>,

    #[schema(example = 987654)]
    pub receipt_no: i64,

    #[schema(example = "123 Main St, City")]
    pub address: Option<String>,

    #[schema(example = "v1.0.0")]
    pub version: String,

    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: String,

    #[schema(example = "2023-01-01T00:00:00Z")]
    pub updated_at: String,
}