// TODO add all integration tests to this HTTP-layer file, so tests can run without a TMDB_API_KEY

use tmdb_sans_io::{
    model::{Tv, TvSeason},
    themoviedb::{Executable as _, TMDb, TMDbApi as _},
};

const DUMMY_API_KEY: &str = "DUMMY_API_KEY";
const TMDB: TMDb = TMDb {
    api_key: DUMMY_API_KEY,
    language: "en",
};

#[test]
fn tv_series() {
    const SERIES_ID: u64 = 10;
    const NAME: &str = "NAME";

    let tv_series_get = TMDB.fetch_id(SERIES_ID).finish();
    assert_eq!(
        tv_series_get.request_url(),
        format!("https://api.themoviedb.org/3/tv/{SERIES_ID}?api_key={DUMMY_API_KEY}&language=en")
    );

    let tv_series: Tv = tv_series_get
        .receive_response_str(&json_tv_series(|season| {
            season["id"] = SERIES_ID.into();
            season["name"] = NAME.into();
        }))
        .unwrap();
    assert_eq!(tv_series.id, SERIES_ID);
    assert_eq!(tv_series.name, NAME);
}
fn json_tv_series(modify_fn: impl FnOnce(&mut serde_json::Value)) -> String {
    let mut value = serde_json::json!({
        "id": 0,
        "created_by": [],
        "episode_run_time": [],
        "first_air_date": "",
        "genres": [],
        "in_production": false,
        "languages": [],
        "last_air_date": "",
        "name": "",
        "networks": [],
        "number_of_episodes": 0,
        "number_of_seasons": 0,
        "origin_country": [],
        "original_language": "",
        "original_name": "",
        "overview": "",
        "popularity": 0.0,
        "production_companies": [],
        "seasons": [],
        "status": "",
        "type": "",
        "vote_average": 0.0,
        "vote_count": 0,
    });
    modify_fn(&mut value);
    value.to_string()
}

#[test]
fn tv_season() {
    const SEASON_NAME: &str = "NAME";

    let series_id = 10;
    let season_number = 20;
    let tv_season_get = TMDB.fetch_id(series_id).tv_season(season_number).finish();
    assert_eq!(
        tv_season_get.request_url(),
        format!(
            "https://api.themoviedb.org/3/tv/{series_id}/season/{season_number}?api_key={DUMMY_API_KEY}&language=en"
        )
    );

    let tv_season: TvSeason = tv_season_get
        .receive_response_str(&json_tv_season(|season| {
            season["name"] = SEASON_NAME.into();
            season["episodes"] = vec![
                json_value_episode(|episode| {
                    episode["name"] = "ep0".into();
                    episode["still_path"] = "still0".into();
                }),
                json_value_episode(|episode| {
                    episode["name"] = "ep1".into();
                    episode["still_path"] = "still1".into();
                }),
            ]
            .into();
        }))
        .unwrap();
    assert_eq!(tv_season.name, SEASON_NAME);

    assert_eq!(tv_season.episodes.len(), 2);
    {
        let episode_0 = &tv_season.episodes[0];
        assert_eq!(episode_0.name, "ep0");
        assert_eq!(episode_0.still_path.as_ref().unwrap(), "still0");
    }
    {
        let episode_1 = &tv_season.episodes[1];
        assert_eq!(episode_1.name, "ep1");
        assert_eq!(episode_1.still_path.as_ref().unwrap(), "still1");
    }
}
fn json_tv_season(modify_fn: impl FnOnce(&mut serde_json::Value)) -> String {
    let mut value = serde_json::json!({
        "episodes": [],
        "id": 0,
        "name": "",
        "overview": "",
        "season_number": 0,
    });
    modify_fn(&mut value);
    value.to_string()
}
fn json_value_episode(modify_fn: impl FnOnce(&mut serde_json::Value)) -> serde_json::Value {
    let mut value = serde_json::json!({
        "air_date": "",
        "episode_number": 0,
        "id": 0,
        "overview": "",
        "vote_average": 0.0,
        "vote_count": 0,
    });
    modify_fn(&mut value);
    value
}

#[test]
fn fetch_config() {
    let config_get = TMDB.fetch_config().finish();
    assert_eq!(
        config_get.request_url(),
        format!("https://api.themoviedb.org/3/configuration?api_key={DUMMY_API_KEY}&language=en")
    );
    let config = config_get
        .receive_response_str(r#"{"images":{"base_url":"BASE_URL"}}"#)
        .unwrap();

    assert_eq!(config.images.base_url, "BASE_URL");
}
