use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateProfileReq {
    pub firstname: String,
    pub lastname: String,
}
