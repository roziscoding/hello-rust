use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize)]
pub struct Friend {
    pub id: Uuid,
    pub name: String,
    pub pronouns: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct NewFriend {
    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "everybody has pronouns, granpa!"))]
    pub pronouns: String,
    pub notes: Option<String>,
}
