#![forbid(future_incompatible, unsafe_code)]

//! Tide middleware to filter requests based on their Content-Type.
//!
//! As described in [tide#805](https://github.com/http-rs/tide/issues/805).
//!
//! ## Example
//!
//! Only process requests with `Content-Type: application/json`, returns HTTP
//! 415 Unsupported Media Type for all other requests.
//!
//! ```rust
//! # async_std::task::block_on(async {
//! use tide_content_type_filter::ContentTypeFilter;
//!
//! let mut server = tide::new();
//!
//! server.with(ContentTypeFilter::only("application/json"));
//! # });
//! ```

/// A middleware for filtering requests based on their Content-Type.
#[derive(Clone, Debug)]
pub struct ContentTypeFilter {
  content_type: tide::http::Mime,
}

impl ContentTypeFilter {
  /// Creates a new filter that will only allow requests through that match
  /// the specified content type.
  ///
  /// Any other content types will return a HTTP 415 Unsupported Media Type
  /// status code.
  pub fn only<T: Into<tide::http::Mime>>(content_type: T) -> Self {
    Self {
      content_type: content_type.into(),
    }
  }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> tide::Middleware<State>
  for ContentTypeFilter
{
  async fn handle(
    &self,
    request: tide::Request<State>,
    next: tide::Next<'_, State>,
  ) -> tide::Result {
    if let Some(content_type) = request.content_type() {
      if content_type == self.content_type {
        return Ok(next.run(request).await);
      }
    }

    Ok(tide::Response::new(tide::StatusCode::UnsupportedMediaType))
  }
}
