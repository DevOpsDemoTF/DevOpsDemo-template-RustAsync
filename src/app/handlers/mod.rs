use tide::{Error, StatusCode};

pub mod health_check;

type SharedState = super::SharedState;

fn wrap_err<E: std::error::Error>(err: E) -> Error {
    let err_msg = anyhow::Error::msg(err.to_string());
    tide::Error::new(StatusCode::InternalServerError, err_msg)
}
