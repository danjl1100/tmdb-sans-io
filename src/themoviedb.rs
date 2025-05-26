use crate::model::{ConfigDetails, FindResult, Movie, SearchMovie, SearchResult, Tv, TvSeason};

pub trait Executable<T>
where
    T: serde::de::DeserializeOwned,
{
    fn finish(&self) -> HttpGet<T>;
}

pub trait Search<'a> {
    fn title(&mut self, title: &'a str) -> &mut SearchData<'a>;
    fn year(&mut self, year: u64) -> &mut SearchData<'a>;
}

#[derive(Clone, Debug)]
pub struct SearchData<'a> {
    tmdb: TMDb,
    title: &'a str,
    year: Option<u64>,
}

impl<'a> Search<'a> for SearchData<'a> {
    fn title(&mut self, title: &'a str) -> &mut SearchData<'a> {
        self.title = title;
        self
    }

    fn year(&mut self, year: u64) -> &mut SearchData<'a> {
        self.year = Some(year);
        self
    }
}

#[derive(Clone, Debug)]
pub struct HttpGet<T>
where
    T: serde::de::DeserializeOwned,
{
    request_url: String,
    response_ty: std::marker::PhantomData<T>,
}
impl<T> HttpGet<T>
where
    T: serde::de::DeserializeOwned,
{
    fn new(request_url: String) -> Self {
        Self {
            request_url,
            response_ty: std::marker::PhantomData,
        }
    }

    /// Returns the URL needed to fulfill the request
    #[must_use]
    pub fn request_url(&self) -> &str {
        &self.request_url
    }
    /// Parses the response string into the desired result
    ///
    /// Convenience function for [`Self::receive_response`]
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not valid JSON for the expected data model type
    pub fn receive_response_str(self, response: &str) -> Result<T, Error> {
        self.receive_response(response.as_bytes())
    }
    /// Parses the response into the desired result
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not valid JSON for the expected data model type
    pub fn receive_response(self, response: impl std::io::Read) -> Result<T, Error> {
        serde_json::from_reader(response)
            .map_err(ErrorKind::SerdeDecode)
            .map_err(|kind| Error { kind })
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}
#[derive(Debug)]
enum ErrorKind {
    SerdeDecode(serde_json::Error),
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::SerdeDecode(e) => Some(e),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { kind } = self;
        match kind {
            ErrorKind::SerdeDecode(_) => write!(f, "failed to decode JSON response"),
        }
    }
}

impl Executable<SearchResult> for SearchData<'_> {
    fn finish(&self) -> HttpGet<SearchResult> {
        let relative_url: String = match self.year {
            None => format!(
                "/search/movie?api_key={}&language={}&query={}&append_to_response=images",
                self.tmdb.api_key, // rustfmt hint
                self.tmdb.language,
                self.title
            ),
            Some(year) => format!(
                "/search/movie?api_key={}&language={}&query={}&year={}&append_to_response=images",
                self.tmdb.api_key, // rustfmt hint
                self.tmdb.language,
                self.title,
                year
            ),
        };
        let url = build_url(&relative_url);
        HttpGet::new(url)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Appendable {
    Videos,
    Credits,
}

pub trait Fetch {
    fn id(&mut self, id: u64) -> &mut FetchData;
    fn append_videos(&mut self) -> &mut FetchData;
    fn append_credits(&mut self) -> &mut FetchData;
}

#[derive(Clone, Debug)]
pub struct FetchData {
    tmdb: TMDb,
    id: u64,
    append_to_response: Vec<Appendable>,
}

impl FetchData {
    pub fn tv_season(self, season_number: u32) -> FetchDataTvSeason {
        FetchDataTvSeason {
            fetch_data: self,
            season_number,
        }
    }
}
impl Fetch for FetchData {
    fn id(&mut self, id: u64) -> &mut FetchData {
        self.id = id;
        self
    }

    fn append_videos(&mut self) -> &mut FetchData {
        self.append_to_response.push(Appendable::Videos);
        self
    }

    fn append_credits(&mut self) -> &mut FetchData {
        self.append_to_response.push(Appendable::Credits);
        self
    }
}

impl Executable<Movie> for FetchData {
    fn finish(&self) -> HttpGet<Movie> {
        let mut relative_url: String = format!(
            "/movie/{}?api_key={}&language={}",
            self.id, // rustfmt hint
            self.tmdb.api_key,
            self.tmdb.language
        );

        if !self.append_to_response.is_empty() {
            relative_url.push_str("&append_to_response=");
            for appendable in &self.append_to_response {
                match appendable {
                    Appendable::Videos => relative_url.push_str("videos,"),
                    Appendable::Credits => relative_url.push_str("credits,"),
                }
            }
        }

        let url = build_url(&relative_url);

        HttpGet::new(url)
    }
}

impl Executable<Tv> for FetchData {
    fn finish(&self) -> HttpGet<Tv> {
        let mut relative_url: String = format!(
            "/tv/{}?api_key={}&language={}",
            self.id, // rustfmt hint
            self.tmdb.api_key,
            self.tmdb.language
        );

        if !self.append_to_response.is_empty() {
            relative_url.push_str("&append_to_response=");
            for appendable in &self.append_to_response {
                match appendable {
                    Appendable::Videos => relative_url.push_str("videos,"),
                    Appendable::Credits => relative_url.push_str("credits,"),
                }
            }
        }

        let url = build_url(&relative_url);

        HttpGet::new(url)
    }
}

#[derive(Clone, Debug)]
#[must_use]
pub struct FetchDataTvSeason {
    fetch_data: FetchData,
    season_number: u32,
}
impl Fetch for FetchDataTvSeason {
    fn id(&mut self, id: u64) -> &mut FetchData {
        self.fetch_data.id(id)
    }
    fn append_videos(&mut self) -> &mut FetchData {
        self.fetch_data.append_videos()
    }
    fn append_credits(&mut self) -> &mut FetchData {
        self.fetch_data.append_credits()
    }
}

impl Executable<TvSeason> for FetchDataTvSeason {
    fn finish(&self) -> HttpGet<TvSeason> {
        let Self {
            fetch_data:
                FetchData {
                    tmdb: TMDb { api_key, language },
                    id,
                    append_to_response,
                },
            season_number,
        } = self;

        let mut relative_url: String =
            format!("/tv/{id}/season/{season_number}?api_key={api_key}&language={language}");

        if !append_to_response.is_empty() {
            relative_url.push_str("&append_to_response=");
            for appendable in append_to_response {
                match appendable {
                    Appendable::Videos => relative_url.push_str("videos,"),
                    Appendable::Credits => relative_url.push_str("credits,"),
                }
            }
        }

        let url = build_url(&relative_url);

        HttpGet::new(url)
    }
}

pub trait Find<'a> {
    fn imdb_id(&mut self, imdb_id: &'a str) -> &mut FindData<'a>;
}

#[derive(Clone, Debug)]
pub struct FindData<'a> {
    tmdb: TMDb,
    imdb_id: &'a str,
}

impl<'a> Find<'a> for FindData<'a> {
    fn imdb_id(&mut self, imdb_id: &'a str) -> &mut FindData<'a> {
        self.imdb_id = imdb_id;
        self
    }
}

impl Executable<FindResult> for FindData<'_> {
    fn finish(&self) -> HttpGet<FindResult> {
        let relative_url = format!(
            "/find/{}?api_key={}&external_source=imdb_id&language={}&append_to_response=images",
            self.imdb_id, // rustfmt hint
            self.tmdb.api_key,
            self.tmdb.language
        );
        let url = build_url(&relative_url);
        HttpGet::new(url)
    }
}

#[derive(Clone, Debug)]
pub struct FetchConfig {
    tmdb: TMDb,
}
impl Executable<ConfigDetails> for FetchConfig {
    fn finish(&self) -> HttpGet<ConfigDetails> {
        let relative_url = format!(
            "/configuration?api_key={}&language={}",
            self.tmdb.api_key, self.tmdb.language
        );
        let url = build_url(&relative_url);
        HttpGet::new(url)
    }
}

pub trait TMDbApi {
    fn search_title<'a>(&self, title: &'a str) -> SearchData<'a>;
    fn fetch_id(&self, id: u64) -> FetchData;
    fn find_id<'a>(&self, tmdb_id: &'a str) -> FindData<'a>;
    fn fetch_config(&self) -> FetchConfig;
}

#[derive(Clone, Debug)]
pub struct TMDb {
    pub api_key: &'static str,
    pub language: &'static str,
}

impl TMDbApi for TMDb {
    fn search_title<'a>(&self, title: &'a str) -> SearchData<'a> {
        let tmdb = self.clone();
        SearchData {
            tmdb,
            title,
            year: None,
        }
    }

    fn fetch_id(&self, id: u64) -> FetchData {
        let tmdb = self.clone();
        FetchData {
            tmdb,
            id,
            append_to_response: vec![],
        }
    }

    fn find_id<'a>(&self, imdb_id: &'a str) -> FindData<'a> {
        let tmdb = self.clone();
        FindData { tmdb, imdb_id }
    }

    fn fetch_config(&self) -> FetchConfig {
        let tmdb = self.clone();
        FetchConfig { tmdb }
    }
}

pub trait Fetchable {
    fn fetch(&self, tmdb: &TMDb) -> HttpGet<Movie>;
}

impl Fetchable for SearchMovie {
    fn fetch(&self, tmdb: &TMDb) -> HttpGet<Movie> {
        tmdb.fetch_id(self.id).finish()
    }
}

fn build_url(relative_raw: &str) -> String {
    /// <https://url.spec.whatwg.org/#query-percent-encode-set>
    const QUERY: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>');

    const BASE_URL: &str = "https://api.themoviedb.org/3";

    let absolute_raw = format!("{BASE_URL}{relative_raw}");

    percent_encoding::utf8_percent_encode(&absolute_raw, QUERY).to_string()
}
