use config::Config;

mod app;
mod config;
mod metrics;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = Config::new();
    tide::log::with_level(config.log_level.to_level_filter());

    tide::log::info!("Service has been started");

    let app = app::new(&config);

    futures::try_join!(metrics::init(), app.listen("0.0.0.0:8080"))?;
    Ok(())
}
