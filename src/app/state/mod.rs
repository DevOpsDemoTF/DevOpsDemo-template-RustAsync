pub struct State {
    healthy: bool,
}

impl State {
    pub fn new(_config: &crate::config::Config) -> State {
        State { healthy: true }
    }

    pub fn ping(&self) -> std::io::Result<()> {
        if self.healthy {
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "unhealthy"))
        }
    }
}
