use prometheus::{self, Encoder, TextEncoder};
use tide::{Body, Request, Response, Result};

async fn handle_metrics(mut _req: Request<()>) -> Result {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(Response::builder(tide::StatusCode::Ok)
        .body(Body::from(buffer))
        .content_type(encoder.format_type())
        .build())
}

pub async fn init() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("/metrics").get(handle_metrics);
    app.listen("0.0.0.0:9102").await
}
