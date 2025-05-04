use tmdb_sans_io::model::*;
use tmdb_sans_io::themoviedb::*;

fn main() {
    let Some(api_key) = option_env!("TMDB_API_KEY") else {
        panic!("requires TMDB_API_KEY environment variable at compile time")
    };
    let tmdb = TMDb {
        api_key,
        language: "en",
    };

    let movie_get = tmdb
        .fetch()
        .id(2277)
        .append_videos()
        .append_credits()
        .finish();

    let movie: Movie = execute_request(movie_get);

    println!("{:#?}", movie);
}

fn execute_request<T>(http_get: HttpGet<T>) -> T
where
    T: serde::de::DeserializeOwned,
{
    let response = ureq::get(http_get.request_url())
        .call()
        .unwrap()
        .into_body()
        .into_reader();

    http_get.receive_response(response).unwrap()
}
