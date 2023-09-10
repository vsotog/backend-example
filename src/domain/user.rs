use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub url: String,
}