use std::env;

use actix_web::{
    Error, HttpMessage,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json::json;

use crate::models::shared::Claims;

pub async fn verified_jwt(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized(json!({"message": "Authorization header is missing"})))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| ErrorUnauthorized(json!({"message": "Authorization header is malformed"})))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(
            json!({"message": "Authorization header is invalid"}),
        ));
    }
    let token = auth_str.strip_prefix("Bearer ").unwrap();

    let key = env::var("JWT_SECRET").expect("JWT_SECRET is missing");
    let key = DecodingKey::from_secret(key.as_bytes());

    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims.sub);
            next.call(req).await
        }
        Err(_) => Err(ErrorUnauthorized(json!({"message":"unauthorized"}))),
    }
}
