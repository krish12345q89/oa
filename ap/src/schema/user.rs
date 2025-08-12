use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use utoipa::{ ToSchema };
use secrecy::SecretString;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[schema(rename = "admin")]
    Admin,
    #[schema(rename = "user")]
    User,
    #[schema(rename = "guest")]
    Guest,
}

#[allow(dead_code)]
#[derive(Debug, Clone, ToSchema, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000", value_type = String)]
    id: Uuid,

    #[schema(example = "john_doe", pattern = r"^[a-zA-Z0-9_]{3,30}$", max_length = 30)]
    username: String,

    #[schema(example = "john@example.com", pattern = r"^[^@\s]+@[^@\s]+\.[^@\s]+$")]
    email: String,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    created_at: DateTime<Utc>,

    #[schema(value_type = String, example = "2023-01-01T00:00:00Z")]
    updated_at: DateTime<Utc>,

    #[schema(ignore)] // Never expose password in API docs
    #[serde(skip_serializing)]
    password_hash: SecretString,

    #[schema(example = "user")]
    role: UserRole,

    #[schema(example = "v1.0.0", max_length = 20)]
    version: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, ToSchema, Deserialize)]
pub struct CreateUserRequest {
    #[schema(example = "john_doe")]
    pub username: String,

    #[schema(example = "john@example.com")]
    pub email: String,

    #[schema(example = "strongPassword123!")]
    pub password: String,

    #[schema(default = "UserRole::User")]
    pub role: Option<UserRole>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, ToSchema, Deserialize)]
pub struct UpdateUserRequest {
    #[schema(example = "new_username")]
    pub username: Option<String>,

    #[schema(example = "new@example.com")]
    pub email: Option<String>,

    #[schema(example = "newStrongPassword123!")]
    pub password: Option<String>,
}
