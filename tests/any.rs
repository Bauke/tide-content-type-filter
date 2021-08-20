use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_any_filter() {
  let mut server = tide::new();

  server.with(tide_content_type_filter::ContentTypeFilter::any(vec![
    "image/png",
    "image/jpeg",
  ]));

  server.at("/").get(any_route);

  let mut ok = server.get("/").content_type("image/png").await.unwrap();
  assert_eq!(ok.status(), tide::http::StatusCode::Ok);
  assert_eq!(ok.body_string().await.unwrap(), "image/png".to_string());

  let mut ok = server.get("/").content_type("image/jpeg").await.unwrap();
  assert_eq!(ok.status(), tide::http::StatusCode::Ok);
  assert_eq!(ok.body_string().await.unwrap(), "image/jpeg".to_string());

  let unsupported = server.get("/").content_type("image/tiff").await.unwrap();
  assert_eq!(
    unsupported.status(),
    tide::http::StatusCode::UnsupportedMediaType
  );
}

async fn any_route(request: tide::Request<()>) -> tide::Result {
  let content_type = request.content_type().unwrap();
  assert!(content_type.essence().starts_with("image/"));
  Ok(content_type.essence().into())
}
