use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_filter() {
  let mut server = tide::new();

  server.with(tide_content_type_filter::ContentTypeFilter::only(
    "plain/text",
  ));

  server.at("/").get(route);

  let mut ok = server.get("/").content_type("plain/text").await.unwrap();
  assert_eq!(ok.status(), tide::http::StatusCode::Ok);
  assert_eq!(ok.body_string().await.unwrap(), "plain/text".to_string());

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

async fn route(request: tide::Request<()>) -> tide::Result {
  let content_type = request.content_type().unwrap();
  assert_eq!(content_type.essence(), "plain/text");
  Ok(content_type.essence().into())
}
