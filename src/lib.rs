//! # The Movie Database
//!
//! ![The Movie Database](https://www.themoviedb.org/assets/2/v4/logos/408x161-powered-by-rectangle-green-bb4301c10ddc749b4e79463811a68afebeae66ef43d17bcfd8ff0e60ded7ce99.png)
//!
//! This is a fork of [tmdb (Cir0X/tmdb-rs)](https://gitlab.com/Cir0X/tmdb-rs), following the [sans-io](https://sans-io.readthedocs.io/) approach to let you build a custom wrapper around [TMDb API](https://developers.themoviedb.org/3).
//!
//! Bring your own I/O stack!
//!
//! ## Motivation
//!
//! Why yet another [TMDb API library](https://lib.rs/search?q=tmdb) written in Rust?
//!
//! For a throw-away hobby project, I needed to access [TMDb API](https://developers.themoviedb.org/3):
//! - The search on [lib.rs](https://lib.rs/search?q=tmdb) lists several crate implementations, great!
//! - Looking among the results (at the time of writing) some require async runtimes... but [tmdb (Cir0X/tmdb-rs)](https://gitlab.com/Cir0X/tmdb-rs) doesn't, nice!
//!     - N.B. I'm not hating on async, just for this project pulling an async runtime seemed like overkill.  If I did pull in an async runtime, I wanted it to be on my terms.
//! - Running [`cargo audit`](https://crates.io/crates/cargo-audit), it seems one dependency of the chosen library (the latest `reqwest` version 0.9.24 per the semver range, from 2019-12-11) has security advisories.
//!
//! The situation is understandable: an older but still working API access library (last updated in 2021) that uses an older version of HTTP client library is bound to have a few security vulnerabilities reported deep in their dependency tree.
//!
//! Security advisories (in this case, DDOS vulnerabilities) may not be critical for a hobby project but are still not ideal.
//!
//! Instead of manually patching the API access library to use an updated HTTP client library, why not decouple the API models from the HTTP client?
//!
//! Credit goes to the original authors for the data models, and API design. Building upon that, the [`finish`](`themoviedb::Executable`) method is crudely bolted on, returning an [`HttpGet`](`themoviedb::HttpGet`) struct containing the URL to be queried and a `receive_response` method to parse the JSON response.

pub mod model;
pub mod themoviedb;
