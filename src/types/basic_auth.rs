use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BasicAuth{
    pub username: String, 
    pub password: String
}