use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use log::debug;

#[allow(clippy::missing_errors_doc)]
pub async fn guide_secure<B: Send>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    debug!("Middleware: checking if user exists");
    if crate::tyme_config.lock().first_start {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
