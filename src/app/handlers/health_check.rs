use super::*;
use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};
use tide::{Request, Response, Result, StatusCode};

lazy_static! {
    static ref HEALTH_CHECK_COUNTER: IntCounter = register_int_counter!(
        "health_counter",
        "Number of times the health endpoint has been called"
    )
    .unwrap();
}

pub async fn handler(req: Request<SharedState>) -> Result {
    HEALTH_CHECK_COUNTER.inc();
    let state = req.state().read().map_err(wrap_err)?;
    if state.ping().is_ok() {
        Ok(Response::new(StatusCode::Ok))
    } else {
        Ok(Response::new(StatusCode::InternalServerError))
    }
}
