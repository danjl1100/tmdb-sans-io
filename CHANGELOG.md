# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2025-05-26

### Added

- TV seasons query by ID (`tmdb.fetch_tv_season().id`)
- Configuration details query (`tmdb.fetch_config()`)

## Changed

- Deprecated all-caps acronyms ( `TV`, `TVCast`, `TVCreator`, and `TVCredits`)
  in favor of camel case names ( `Tv`, `TvCast`, `TvCreator`, and `TvCredits`)

## [0.1.1] - 2025-05-24

### Added

- Derive common traits (`Clone`, `Debug`) for all public structs

## [0.1.0] - 2025-05-04

### Changed

- Replaced `reqwest` library calls with `HttpGet` to allow the caller to use their favorite HTTP request strategy

[unreleased]: https://github.com/danjl1100/tmdb-sans-io/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/danjl1100/tmdb-sans-io/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/danjl1100/tmdb-sans-io/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/danjl1100/tmdb-sans-io/releases/tag/v0.1.0
