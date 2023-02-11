use crate::error::AppError;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::basic::BasicAuth;

use super::config::AuthConfig;
pub async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let auth = req.app_data::<AuthConfig>().unwrap();
    if auth.username == credentials.user_id()
        && auth.password == credentials.password().unwrap_or("")
    {
        Ok(req)
    } else {
        Err((AppError::Unauthorized.into(), req))
    }
}
