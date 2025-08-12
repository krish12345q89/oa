use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Order {
    #[schema(
        value_type = String,
        example = "550e8400-e29b-41d4-a716-446655440000"
    )]
    pub id: String,

    #[schema(
        example = "shopify",
        max_length = 20,
        pattern = r"^[a-zA-Z0-9_\-]+$"
    )]
    pub marketplace: String,

    #[schema(example = "1234567890", max_length = 20)]
    pub order_id: String,

    #[schema(example = "1234567890", max_length = 20)]
    pub return_order: Option<u64>,

    #[schema(example = "1234567890", max_length = 20)]
    pub shopify_id: Option<String>,

    #[schema(example = "1234567890", max_length = 20)]
    pub market_place_code: Option<String>,

    #[schema(example = "1234567890", max_length = 20)]
    pub returned_sku: Option<String>,

    #[schema(example = "1234567890", max_length = 20)]
    pub offer_sku: Option<String>,

    #[schema(example = "1234567890", max_length = 20)]
    pub matched_sku: Option<String>,

    #[schema(
        example = "automatic",
        max_length = 20
    )]
    pub match_type: Option<String>,

    #[schema(example = "1", maximum = 9999999)]
    pub row_number: Option<u32>,

    #[schema(
        example = "confirmed",
        max_length = 20,
    )]
    pub manual_confirmation: Option<String>,

    #[schema(
        example = "processed",
        max_length = 20,
    )]
    pub status: Option<String>,

    #[schema(example = "1", maximum = 999)]
    pub qty: Option<u32>,

    #[schema(example = "true", max_length = 5)]
    pub main_updated: Option<String>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub date: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub created_at: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    pub updated_at: String,
}


lazy_static::lazy_static! {
    static ref MARKETPLACE_REGEX: regex::Regex = 
        regex::Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();
}