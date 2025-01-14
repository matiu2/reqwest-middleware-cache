use anyhow::Result;
use reqwest::{Client, Method, Request, Url};
use reqwest_middleware::ClientBuilder;
use reqwest_middleware_cache::{managers::CACacheManager, Cache, CacheManager, CacheMode};

#[tokio::test]
async fn default_mode() -> Result<()> {
    let mut server = mockito::Server::new_async().await;
    let m = server
        .mock("GET", "/")
        .with_status(200)
        .with_header("cache-control", "max-age=86400, public")
        .with_body("test")
        .create();
    let url = format!("{}/", &server.url());
    let manager = CACacheManager::default();
    let path = manager.path.clone();
    let key = format!("GET:{}", &url);
    let req = Request::new(Method::GET, Url::parse(&url)?);

    // Make sure the record doesn't already exist
    manager.delete(&req).await?;

    // Construct reqwest client with cache defaults
    let client = ClientBuilder::new(Client::new())
        .with(Cache {
            mode: CacheMode::Default,
            cache_manager: CACacheManager::default(),
        })
        .build();

    // Cold pass to load cache
    client.get(url).send().await?;
    m.assert();

    // Try to load cached object
    let data = cacache::read(&path, &key).await;
    assert!(data.is_ok());
    Ok(())
}
