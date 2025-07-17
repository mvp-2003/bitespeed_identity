use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct IdentifyRequest {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default, rename = "phoneNumber")]
    pub phone_number: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct IdentifyResponse {
    pub contact: ContactSummary,
}

#[derive(Serialize, Debug)]
pub struct ContactSummary {
    pub primaryContactId: i64,
    pub emails: Vec<String>,
    pub phoneNumbers: Vec<String>,
    pub secondaryContactIds: Vec<i64>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ContactRow {
    pub id: i64,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub linked_id: Option<i64>,
    pub link_precedence: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}