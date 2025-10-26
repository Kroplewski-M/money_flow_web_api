use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}
