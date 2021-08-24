use tide_testing::TideTestingExt;

#[async_std::test]
async fn test_wildcard_filter() {
  let mut server = tide::new();

  server.with(tide_content_type_filter::ContentTypeFilter::only("image/*"));

  server.at("/").get(only_route);

  let requests = vec![
    ("image/png", tide::http::StatusCode::Ok),
    ("image/jpeg", tide::http::StatusCode::Ok),
    ("text/plain", tide::http::StatusCode::UnsupportedMediaType),
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
  assert_eq!(content_type.basetype(), "image");
  Ok(content_type.essence().into())
}
