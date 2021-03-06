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
//!
//! Only process requests with `Content-Type: image/png` or
//! `Content-Type: image/jpeg`:
//!
//! ```rust
//! # async_std::task::block_on(async {
//! use tide_content_type_filter::ContentTypeFilter;
//!
//! let mut server = tide::new();
//!
//! server.with(ContentTypeFilter::any(vec!["image/png", "image/jpeg"]));
//! # });
//! ```

/// A middleware for filtering requests based on their Content-Type.
#[derive(Clone, Debug)]
pub struct ContentTypeFilter {
  content_types: Vec<tide::http::Mime>,
}

impl ContentTypeFilter {
  /// Creates a new filter that will only allow requests through that match
  /// the specified content type.
  ///
  /// Any other content types will return a HTTP 415 Unsupported Media Type
  /// status code.
  pub fn only<T: Into<tide::http::Mime>>(content_type: T) -> Self {
    Self {
      content_types: vec![content_type.into()],
    }
  }

  /// Creates a new filter that will only allow requests through where the
  /// content type matches any of the specified ones.
  ///
  /// Any other content types will return a HTTP 415 Unsupported Media Type
  /// status code.
  pub fn any<T: Into<tide::http::Mime>>(content_types: Vec<T>) -> Self {
    Self {
      content_types: content_types.into_iter().map(Into::into).collect(),
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
      if self.content_types.contains(&content_type)
        || self
          .content_types
          .iter()
          .filter(|allowed| allowed.subtype() == "*")
          .any(|allowed| allowed.basetype() == content_type.basetype())
      {
        return Ok(next.run(request).await);
      }
    }

    Ok(tide::Response::new(tide::StatusCode::UnsupportedMediaType))
  }
}
