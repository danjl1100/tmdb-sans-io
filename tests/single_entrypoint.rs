//! NOTE: Cargo runs integration test binaries sequentially, so only create one entrypoint

#![expect(clippy::panic)] // tests are allowed to panic
#![expect(clippy::unwrap_used)] // tests are allowed to unwrap

mod common {
    mod fetch_http_layer;
    mod fetch_integration;

    fn execute_request<T>(http_get: tmdb_sans_io::themoviedb::HttpGet<T>) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        dbg!(http_get.request_url());
        let response = ureq::get(http_get.request_url())
            .call()
            .unwrap()
            .into_body()
            .into_reader();

        http_get.receive_response(response).unwrap()
    }
}
