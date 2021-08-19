# tide-content-type-filter

Tide middleware to filter requests based on their Content-Type.

As described in [tide#805](https://github.com/http-rs/tide/issues/805).

## Example

Only process requests with `Content-Type: application/json`, returns HTTP
415 Unsupported Media Type for all other requests.

```rust
use tide_content_type_filter::ContentTypeFilter;

let mut server = tide::new();

server.with(ContentTypeFilter::only("application/json"));
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
