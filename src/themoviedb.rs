use crate::model::{FindResult, Movie, SearchMovie, SearchResult, TV};

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

#[derive(Debug)]
pub struct SearchData<'a> {
    pub tmdb: TMDb,
    pub title: Option<&'a str>,
    pub year: Option<u64>,
}

impl<'a> Search<'a> for SearchData<'a> {
    fn title(&mut self, title: &'a str) -> &mut SearchData<'a> {
        self.title = Some(title);
        self
    }

    fn year(&mut self, year: u64) -> &mut SearchData<'a> {
        self.year = Some(year);
        self
    }
}

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
    pub fn request_url(&self) -> &str {
        &self.request_url
    }
    /// Parses the response string into the desired result
    ///
    /// Convenience function for [`Self::receive_response`]
    pub fn receive_response_str(self, response: String) -> Result<T, Error> {
        self.receive_response(response.as_bytes())
    }
    /// Parses the response into the desired result
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

impl<'a> Executable<SearchResult> for SearchData<'a> {
    fn finish(&self) -> HttpGet<SearchResult> {
        let relative_url: String = match self.year {
            None => format!(
                "/search/movie?api_key={}&language={}&query={}&append_to_response=images",
                self.tmdb.api_key,
                self.tmdb.language,
                self.title.unwrap()
            ),
            Some(year) => format!(
                "/search/movie?api_key={}&language={}&query={}&year={}&append_to_response=images",
                self.tmdb.api_key,
                self.tmdb.language,
                self.title.unwrap(),
                year
            ),
        };
        let url = build_url(&relative_url);
        HttpGet::new(url)
    }
}

pub enum Appendable {
    Videos,
    Credits,
}

pub trait Fetch {
    fn id(&mut self, id: u64) -> &mut FetchData;
    fn append_videos(&mut self) -> &mut FetchData;
    fn append_credits(&mut self) -> &mut FetchData;
}

pub struct FetchData {
    pub tmdb: TMDb,
    pub id: Option<u64>,
    pub append_to_response: Vec<Appendable>,
}

impl Fetch for FetchData {
    fn id(&mut self, id: u64) -> &mut FetchData {
        self.id = Some(id);
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
            self.id.unwrap(),
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

impl Executable<TV> for FetchData {
    fn finish(&self) -> HttpGet<TV> {
        let mut relative_url: String = format!(
            "/tv/{}?api_key={}&language={}",
            self.id.unwrap(),
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

pub trait Find<'a> {
    fn imdb_id(&mut self, imdb_id: &'a str) -> &mut FindData<'a>;
}

pub struct FindData<'a> {
    pub tmdb: TMDb,
    pub imdb_id: Option<&'a str>,
}

impl<'a> Find<'a> for FindData<'a> {
    fn imdb_id(&mut self, imdb_id: &'a str) -> &mut FindData<'a> {
        self.imdb_id = Some(imdb_id);
        self
    }
}

impl<'a> Executable<FindResult> for FindData<'a> {
    fn finish(&self) -> HttpGet<FindResult> {
        let relative_url = format!(
            "/find/{}?api_key={}&external_source=imdb_id&language={}&append_to_response=images",
            self.imdb_id.unwrap(),
            self.tmdb.api_key,
            self.tmdb.language
        );
        let url = build_url(&relative_url);
        HttpGet::new(url)
    }
}

pub trait TMDbApi {
    fn search(&self) -> SearchData;
    fn fetch(&self) -> FetchData;
    fn find(&self) -> FindData;
}

#[derive(Debug, Clone)]
pub struct TMDb {
    pub api_key: &'static str,
    pub language: &'static str,
}

impl TMDbApi for TMDb {
    fn search(&self) -> SearchData {
        let tmdb = self.clone();
        SearchData {
            tmdb,
            title: None,
            year: None,
        }
    }

    fn fetch(&self) -> FetchData {
        let tmdb = self.clone();
        FetchData {
            tmdb,
            id: None,
            append_to_response: vec![],
        }
    }

    fn find(&self) -> FindData {
        let tmdb = self.clone();
        FindData {
            tmdb,
            imdb_id: None,
        }
    }
}

pub trait Fetchable {
    fn fetch(&self, tmdb: &TMDb) -> HttpGet<Movie>;
}

impl Fetchable for SearchMovie {
    fn fetch(&self, tmdb: &TMDb) -> HttpGet<Movie> {
        tmdb.fetch().id(self.id).finish()
    }
}

fn build_url(relative_raw: &str) -> String {
    /// https://url.spec.whatwg.org/#query-percent-encode-set
    const QUERY: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>');

    const BASE_URL: &str = "https://api.themoviedb.org/3";
    // const BASE_IMG_URL: &'static str = "https://image.tmdb.org/t/p/w500";
    // "https://image.tmdb.org/t/p/w700_and_h392_bestv2/gq4Z1pfOWHn3FKFNutlDCySps9C.jpg"

    let absolute_raw = format!("{BASE_URL}{relative_raw}");

    percent_encoding::utf8_percent_encode(&absolute_raw, QUERY).to_string()
}
