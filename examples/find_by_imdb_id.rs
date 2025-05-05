use tmdb_sans_io::themoviedb::{Executable, HttpGet, TMDb, TMDbApi};

fn main() {
    let Some(api_key) = option_env!("TMDB_API_KEY") else {
        eprintln!("requires TMDB_API_KEY environment variable at compile time");
        std::process::exit(1)
    };
    let tmdb = TMDb {
        api_key,
        language: "en",
    };

    let find_get = tmdb.find_id("tt0816692").finish();

    let find_result = execute_request(find_get);

    let movies = find_result.movie_results;

    println!("Movies: {movies:#?}");
}

fn execute_request<T>(http_get: HttpGet<T>) -> T
where
    T: serde::de::DeserializeOwned,
{
    let response = ureq::get(http_get.request_url())
        .call()
        .expect("HTTP should succeed")
        .into_body()
        .into_reader();

    http_get
        .receive_response(response)
        .expect("HTTP response should be the expected JSON object format")
}
