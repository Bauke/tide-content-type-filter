use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_only_filter() {
  let mut server = tide::new();

  server.with(tide_content_type_filter::ContentTypeFilter::only(
    "text/plain",
  ));

  server.at("/").get(only_route);

  let requests = vec![
    ("text/plain", tide::http::StatusCode::Ok),
    ("text/html", tide::http::StatusCode::UnsupportedMediaType),
  ];

  for (content_type, status) in requests {
    let mut response =
      server.get("/").content_type(content_type).await.unwrap();
    assert_eq!(response.status(), status);
    if status.is_success() {
      assert_eq!(&response.body_string().await.unwrap(), content_type);
    }
  }
}

async fn only_route(request: tide::Request<()>) -> tide::Result {
  let content_type = request.content_type().unwrap();
  assert_eq!(content_type.essence(), "text/plain");
  Ok(content_type.essence().into())
}
