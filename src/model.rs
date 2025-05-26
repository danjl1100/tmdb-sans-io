use serde_derive::{Deserialize, Serialize};

#[deprecated = "use the camel case type `TvCast` directly"]
pub type TVCast = TvCast;
#[deprecated = "use the camel case type `TvCreator` directly"]
pub type TVCreator = TvCreator;
#[deprecated = "use the camel case type `TvCredits` directly"]
pub type TVCredits = TvCredits;
#[deprecated = "use the camel case type `Tv` directly"]
pub type TV = Tv;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Results<T> {
    pub results: Vec<T>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Video {
    pub id: String,
    pub iso_639_1: String,
    pub key: String,
    pub name: String,
    pub site: String,
    pub size: u16,
    #[serde(rename = "type")]
    pub video_type: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Cast {
    pub id: u64,
    pub cast_id: u64,
    pub credit_id: String,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u8,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TvCast {
    pub id: u64,
    pub credit_id: String,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TvCreator {
    pub id: u64,
    pub credit_id: String,
    pub name: String,
    pub gender: Option<u8>,
    pub profile_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Crew {
    pub credit_id: String,
    pub department: String,
    pub gender: Option<u8>,
    pub id: u64,
    pub job: String,
    pub name: String,
    pub profile_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Credits {
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TvCredits {
    pub cast: Vec<TvCast>,
    pub crew: Vec<Crew>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LastEpisode {
    pub air_date: String,
    pub episode_number: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub production_code: Option<String>,
    pub season_number: u32,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ProductionCompany {
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Network {
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Season {
    pub air_date: Option<String>,
    pub episode_count: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub season_number: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    pub id: u64,
    pub imdb_id: String,
    pub title: String,
    pub tagline: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub runtime: u32,
    pub homepage: Option<String>,
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub budget: u64,
    pub adult: bool,
    pub videos: Option<Results<Video>>,
    pub credits: Option<Credits>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Tv {
    pub id: u64,
    pub backdrop_path: Option<String>,
    pub created_by: Vec<TvCreator>,
    pub episode_run_time: Vec<u64>,
    pub first_air_date: String,
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: String,
    pub last_episode_to_air: Option<LastEpisode>,
    pub name: String,
    pub networks: Vec<Network>,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<ProductionCompany>,
    pub seasons: Vec<Season>,
    pub status: String,
    pub r#type: String,
    pub vote_average: f64,
    pub vote_count: u64,
    pub videos: Option<Results<Video>>,
    pub credits: Option<TvCredits>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TvSeason {
    pub episodes: Vec<TvEpisode>,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub season_number: u32,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TvEpisode {
    pub air_date: String,
    pub episode_number: u32,
    pub id: u64,
    pub name: String,
    pub overview: String,
    pub production_code: Option<String>,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub adult: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FindMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchResult {
    pub page: u8,
    pub total_results: u8,
    pub total_pages: u8,
    pub results: Vec<SearchMovie>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FindResult {
    pub movie_results: Vec<FindMovie>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ConfigDetails {
    pub images: ConfigImageDetails,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ConfigImageDetails {
    pub base_url: String,
}
