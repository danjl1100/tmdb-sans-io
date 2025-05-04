use tmdb_sans_io::themoviedb::*;

fn main() {
    let Some(api_key) = option_env!("TMDB_API_KEY") else {
        panic!("requires TMDB_API_KEY environment variable at compile time")
    };
    let tmdb = TMDb {
        api_key,
        language: "en",
    };

    let find_get = tmdb.find().imdb_id("tt0816692").finish();

    let find_result = execute_request(find_get);

    let movies = find_result.movie_results;

    println!("Movies: {:#?}", movies);
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
