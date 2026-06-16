use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewFriend {
    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "everybody has pronouns, granpa!"))]
    pub pronouns: String,
    pub notes: Option<String>,
}
