use state::State;
use std::sync::{Arc, RwLock};
use tide::Server;

mod handlers;
mod state;

#[cfg(test)]
mod test;

pub type SharedState = Arc<RwLock<state::State>>;

pub fn new(config: &crate::config::Config) -> Server<SharedState> {
    let state = State::new(config);
    let mut app = tide::with_state(Arc::new(RwLock::new(state)));
    app.at("/health").get(handlers::health_check::handler);
    app
}
