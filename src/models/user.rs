use chrono::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(
    Debug, 
    Deserialize, 
    Serialize, 
    Clone, 
    PartialEq, 
    FromSqlRow, 
    AsExpression)]
#[sql_type = "Text"]
pub enum UserRole {
    Admin,
    BetaTester,
    User,
}

impl UserRole {
    pub fn to_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::BetaTester => "betatester",
        }
    }
}

#[allow(non_snake_case)]
#[derive(
    Debug, 
    Deserialize, 
    Serialize, 
    Clone
)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub role: UserRole,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}