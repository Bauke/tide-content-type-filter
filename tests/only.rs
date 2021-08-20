use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_only_filter() {
  let mut server = tide::new();

  server.with(tide_content_type_filter::ContentTypeFilter::only(
    "text/plain",
  ));

  server.at("/").get(only_route);

  let mut ok = server.get("/").content_type("text/plain").await.unwrap();
  assert_eq!(ok.status(), tide::http::StatusCode::Ok);
  assert_eq!(ok.body_string().await.unwrap(), "text/plain".to_string());

  let unsupported = server
    .get("/")
    .content_type("application/json")
    .await
    .unwrap();
  assert_eq!(
    unsupported.status(),
    tide::http::StatusCode::UnsupportedMediaType
  );
}

async fn only_route(request: tide::Request<()>) -> tide::Result {
  let content_type = request.content_type().unwrap();
  assert_eq!(content_type.essence(), "text/plain");
  Ok(content_type.essence().into())
}
