use crate::app;
use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_health() -> tide::Result<()> {
    let config = crate::Config::new();
    let app = app::new(&config);

    let resp = app.client().get("/health").await?;
    assert_eq!(resp.status(), 200);

    Ok(())
}
