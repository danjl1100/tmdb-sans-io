# The Movie Database

![The Movie Database](https://www.themoviedb.org/assets/2/v4/logos/408x161-powered-by-rectangle-green-bb4301c10ddc749b4e79463811a68afebeae66ef43d17bcfd8ff0e60ded7ce99.png)

This is a fork of [tmdb (Cir0X/tmdb-rs)](https://gitlab.com/Cir0X/tmdb-rs), following the [sans-io](https://sans-io.readthedocs.io/) approach to let you build a custom wrapper around [TMDb API](https://developers.themoviedb.org/3).

Bring your own I/O stack!

## Motivation

Why yet another [TMDb API library](https://lib.rs/search?q=tmdb) written in Rust?

For a throw-away hobby project, I needed to access [TMDb API](https://developers.themoviedb.org/3):
- The search on [lib.rs](https://lib.rs/search?q=tmdb) lists several crate implementations, great!
- Looking among the results (at the time of writing) some require async runtimes... but [tmdb (Cir0X/tmdb-rs)](https://gitlab.com/Cir0X/tmdb-rs) doesn't, nice!
    - N.B. I'm not hating on async, just for this project pulling an async runtime seemed like overkill.  If I did pull in an async runtime, I wanted it to be on my terms.
- Running [`cargo audit`](https://crates.io/crates/cargo-audit), it seems one dependency of the chosen library (the latest `reqwest` version 0.9.24 per the semver range, from 2019-12-11) has security advisories.

The situation is understandable: an older but still working API access library (last updated in 2021) that uses an older version of HTTP client library is bound to have a few security vulnerabilities reported deep in their dependency tree.

Security advisories (in this case, DDOS vulnerabilities) may not be critical for a hobby project but are still not ideal.

Instead of manually patching the API access library to use an updated HTTP client library, why not decouple the API models from the HTTP client?

Credit goes to the original authors for the data models, and API design. Building upon that, the `finish` method is crudely bolted on, returning an `HttpGet` struct containing the URL to be queried and a `receive_response` method to parse the JSON response.

## Usage

See [examples/basic](examples/basic.rs) for sample usage.

See the [integration tests](tests/common/fetch_integration.rs) for more examples.


**NOTE: All code examples are not rigorously checked, for reference only.**

## Actions

Currently there are 3 actions available:

* Searching
* Fetching
* Finding

### Searching

You can search for movies by `title` and `year`.

```rust
let page_request = tmdb.search()
    .title("Bicentennial Man")
    .year(1999)
    .finish();
let response = {
    let request_url = page_request.request_url();
    unimplemented!("INSERT YOUR HTTP CLIENT LIBRARY HERE");
};
let page = page_request.receive_response(response)?;

let movies = page.results;
```

### Fetching

You can fetch a movie, when you know its ID. Then you get all the movie details.

```rust
let movie_request = tmdb.fetch()
    .id(157336)
    .finish();
let response = {
    let request_url = movie_request.request_url();
    unimplemented!("INSERT YOUR HTTP CLIENT LIBRARY HERE");
};
let movie = movie_request.receive_response(response)?;
```

When you don't have any movie ID, you can search for a movie and then easily fetch the full details.

```rust
let page_request = tmdb.search()
   .title("Bicentennial Man")
   .year(1999)
   .finish();
let response = {
    let request_url = page_request.request_url();
    unimplemented!("INSERT YOUR HTTP CLIENT LIBRARY HERE");
};
let page = page_request.receive_response(response)?;

let movies = page.results;
let movie_request = movies[0].fetch(&tmdb);
let response = {
    let request_url = movie_request.request_url();
    unimplemented!("INSERT YOUR HTTP CLIENT LIBRARY HERE");
};
let movie = movie_request.receive_response(response)?;
```

Furthermore you can request some more data with the [append to response](https://developers.themoviedb.org/3/getting-started/append-to-response) feature.

```rust
let movie_request = tmdb.fetch()
    .id(2277)
    .append_videos()
    .append_credits()
    .finish();
```

### Finding

[Finding](https://developers.themoviedb.org/3/find/find-by-id) a movie with an external ID is currently supported with IMDB IDs.

```rust
let find_result = tmdb.find()
    .imdb_id("tt0816692")
    .finish();

let response = {
    let request_url = find_request.request_url();
    unimplemented!("INSERT YOUR HTTP CLIENT LIBRARY HERE");
};
let find_result = find_request.receive_response(response);

let movies = find_result.movie_results;
```

## Acknowledgements

* This lib is a fork of [tmdb-rs](https://gitlab.com/Cir0X/tmdb-rs)
* Which was heavily inspired by [omdb-rs](https://github.com/aldrio/omdb-rs)
* [The Movie Database (TMDb)](https://www.themoviedb.org/)
