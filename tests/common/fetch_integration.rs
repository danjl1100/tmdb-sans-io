use crate::common::execute_request;
use tmdb_sans_io::model::{Movie, TV};
use tmdb_sans_io::themoviedb::*;

const API_KEY: Option<&str> = option_env!("TMDB_API_KEY");
const LANGUAGE: &str = "en";

fn get_tmdb(language: &'static str) -> TMDb {
    let Some(api_key) = API_KEY else {
        // NOTE: do not error during compilation, as the API key is not needed to verify the local
        // crate API is compatible with the test cases
        panic!(
            "this integration test requires the TMDB_API_KEY environment variable at compile time"
        )
    };
    TMDb { api_key, language }
}

#[test]
fn fetch_movie() {
    let movie_get = get_tmdb(LANGUAGE).fetch_id(157_336).finish();
    let movie: Movie = execute_request(movie_get);

    assert_eq!("Interstellar", movie.original_title);
}

#[test]
fn fetch_movie_languages() {
    let tmdb_eng = get_tmdb("en");
    let movie_en_get = tmdb_eng.fetch_id(2277).finish();
    let movie_eng: Movie = execute_request(movie_en_get);
    assert_eq!("Bicentennial Man", movie_eng.title);

    let tmdb_de = get_tmdb("de");
    let movie_de_get = tmdb_de.fetch_id(2277).finish();
    let movie_de: Movie = execute_request(movie_de_get);
    assert_eq!("Der 200 Jahre Mann", movie_de.title);

    let tmdb_esp = get_tmdb("es");
    let movie_esp_get = tmdb_esp.fetch_id(2277).finish();
    let movie_esp: Movie = execute_request(movie_esp_get);
    assert_eq!("El hombre bicentenario", movie_esp.title);
}

#[test]
fn fetch_movie_append_to_response() {
    let movie_get = get_tmdb(LANGUAGE)
        .fetch_id(2277)
        .append_videos()
        .append_credits()
        .finish();
    let movie: Movie = execute_request(movie_get);

    assert!(movie.videos.is_some());
    assert!(movie.credits.is_some());
}

#[test]
fn search_movie() {
    let page_get = get_tmdb(LANGUAGE) // rustfmt hint
        .search_title("Bicentennial Man")
        .year(1999)
        .finish();
    let page = execute_request(page_get);

    let movies = page.results;

    assert_eq!(1, page.total_results);
    assert!(!movies.is_empty());
    assert_eq!("Bicentennial Man", movies[0].title);
}

#[test]
fn find_movie_by_imdb_id() {
    let find_get = get_tmdb(LANGUAGE).find_id("tt0816692").finish();
    let find_result = execute_request(find_get);

    let movies = find_result.movie_results;

    assert_eq!(1, movies.len());
    assert!(!movies.is_empty());
    assert_eq!("Interstellar", movies[0].title);
}

#[test]
fn fetch_searched_movie() {
    let tmdb = get_tmdb(LANGUAGE);

    let page_get = tmdb // rustfmt hint
        .search_title("Bicentennial Man")
        .year(1999)
        .finish();
    let page = execute_request(page_get);

    let movies = page.results;
    let movie_get = movies[0].fetch(&tmdb);
    let movie = execute_request(movie_get);

    assert_eq!(2277, movie.id);
}

#[test]
fn fetch_tv() {
    let tv_get = get_tmdb(LANGUAGE).fetch_id(2316).finish();
    let tv: TV = execute_request(tv_get);

    assert_eq!("The Office", tv.original_name);
}

#[test]
fn fetch_tv_append_to_response() {
    let tv_get = get_tmdb(LANGUAGE)
        .fetch_id(2316)
        .append_videos()
        .append_credits()
        .finish();
    let tv: TV = execute_request(tv_get);

    assert!(tv.videos.is_some());
    assert!(tv.credits.is_some());
}
