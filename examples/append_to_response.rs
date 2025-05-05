use tmdb_sans_io::model::Movie;
use tmdb_sans_io::themoviedb::{Executable, Fetch, HttpGet, TMDb, TMDbApi};

fn main() {
    let Some(api_key) = option_env!("TMDB_API_KEY") else {
        eprintln!("requires TMDB_API_KEY environment variable at compile time");
        std::process::exit(1)
    };
    let tmdb = TMDb {
        api_key,
        language: "en",
    };

    let movie_get = tmdb
        .fetch_id(2277)
        .append_videos()
        .append_credits()
        .finish();

    let movie: Movie = execute_request(movie_get);

    println!("{movie:#?}");
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
