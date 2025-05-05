use tmdb_sans_io::model::Movie;
use tmdb_sans_io::themoviedb::{Executable, HttpGet, Search, TMDb, TMDbApi};

fn main() {
    let Some(api_key) = option_env!("TMDB_API_KEY") else {
        eprintln!("requires TMDB_API_KEY environment variable at compile time");
        std::process::exit(1)
    };
    let tmdb = TMDb {
        api_key,
        language: "en",
    };

    let movies_get = tmdb // rustfmt hint
        .search_title("Interstellar")
        .year(2014)
        .finish();

    let movies = execute_request(movies_get);

    let id = movies.results[0].id;

    let movie_get = tmdb.fetch_id(id).finish();

    let interstellar: Movie = execute_request(movie_get);

    println!("{interstellar:#?}");
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
