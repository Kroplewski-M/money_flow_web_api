use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}
