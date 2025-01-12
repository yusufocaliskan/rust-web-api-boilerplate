use crate::framework::shared::responser::response_generator::SnarkyResponder;
use actix_web::HttpResponse;
use bson::oid::ObjectId;

use actix_web::http::StatusCode;
pub fn parse_object_id(id: &str) -> Result<ObjectId, HttpResponse> {
    match ObjectId::parse_str(id) {
        Ok(object_id) => Ok(object_id),
        Err(_) => Err(SnarkyResponder::error()
            .message("Invalid Id format")
            .code(StatusCode::BAD_REQUEST)
            .build()),
    }
}
