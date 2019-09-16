use tide::App;

#[derive(Default)]
pub struct State {}

pub fn new(_config: &crate::config::Config) -> App<State> {
    let mut app = App::with_state(State::default());
    app.at("/health")
        .get(crate::handlers::health_check::handle_health_check);
    app
}
