use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}
