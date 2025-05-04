use crate::common::execute_request;
use tmdb_sans_io::model::{FindMovie, Movie, SearchMovie, TV};
use tmdb_sans_io::themoviedb::*;

const API_KEY: &str = env!("TMDB_API_KEY");
const LANGUAGE: &str = "en";
const TMDB: TMDb = TMDb {
    api_key: API_KEY,
    language: LANGUAGE,
};

#[test]
fn fetch_movie() {
    let movie_get = TMDB.fetch().id(157336).finish();
    let movie: Movie = execute_request(movie_get);

    assert_eq!("Interstellar", movie.original_title);
}

#[test]
fn fetch_movie_languages() {
    let tmdb_en = TMDb {
        api_key: API_KEY,
        language: "en",
    };
    let movie_en_get = tmdb_en.fetch().id(2277).finish();
    let movie_en: Movie = execute_request(movie_en_get);
    assert_eq!("Bicentennial Man", movie_en.title);

    let tmdb_de = TMDb {
        api_key: API_KEY,
        language: "de",
    };
    let movie_de_get = tmdb_de.fetch().id(2277).finish();
    let movie_de: Movie = execute_request(movie_de_get);
    assert_eq!("Der 200 Jahre Mann", movie_de.title);

    let tmdb_es = TMDb {
        api_key: API_KEY,
        language: "es",
    };
    let movie_es_get = tmdb_es.fetch().id(2277).finish();
    let movie_es: Movie = execute_request(movie_es_get);
    assert_eq!("El hombre bicentenario", movie_es.title);
}

#[test]
fn fetch_movie_append_to_response() {
    let movie_get = TMDB
        .fetch()
        .id(2277)
        .append_videos()
        .append_credits()
        .finish();
    let movie: Movie = execute_request(movie_get);

    assert_eq!(true, movie.videos.is_some());
    assert_eq!(true, movie.credits.is_some());
}

#[test]
fn search_movie() {
    let empty_movies: Vec<SearchMovie> = vec![];

    let page_get = TMDB // rustfmt hint
        .search()
        .title("Bicentennial Man")
        .year(1999)
        .finish();
    let page = execute_request(page_get);

    let movies = page.results;

    assert_eq!(1, page.total_results);
    assert_ne!(empty_movies, movies);
    assert_eq!("Bicentennial Man", movies[0].title);
}

#[test]
fn find_movie_by_imdb_id() {
    let empty_movies: Vec<FindMovie> = vec![];

    let find_get = TMDB.find().imdb_id("tt0816692").finish();
    let find_result = execute_request(find_get);

    let movies = find_result.movie_results;

    assert_eq!(1, movies.len());
    assert_ne!(empty_movies, movies);
    assert_eq!("Interstellar", movies[0].title);
}

#[test]
fn fetch_searched_movie() {
    let page_get = TMDB // rustfmt hint
        .search()
        .title("Bicentennial Man")
        .year(1999)
        .finish();
    let page = execute_request(page_get);

    let movies = page.results;
    let movie_get = movies[0].fetch(&TMDB);
    let movie = execute_request(movie_get);

    assert_eq!(2277, movie.id);
}

#[test]
fn fetch_tv() {
    let tv_get = TMDB.fetch().id(2316).finish();
    let tv: TV = execute_request(tv_get);

    assert_eq!("The Office", tv.original_name);
}

#[test]
fn fetch_tv_append_to_response() {
    let tv_get = TMDB
        .fetch()
        .id(2316)
        .append_videos()
        .append_credits()
        .finish();
    let tv: TV = execute_request(tv_get);

    assert_eq!(true, tv.videos.is_some());
    assert_eq!(true, tv.credits.is_some());
}
