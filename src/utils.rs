use actix_web::{HttpMessage, HttpRequest};
use uuid::Uuid;

pub fn get_user_id(req: HttpRequest) -> Uuid {
    let ext = req.extensions();
    *ext.get::<Uuid>().unwrap()
}
