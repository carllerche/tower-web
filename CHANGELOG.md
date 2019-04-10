# 0.3.7 (April 10, 2019)

### Fixed
- fix panics with non-ASCII characters in routes (#206).
- fix build on newer rustc versions (#205).

### Changed
- duplicate routes are now detected at compile-time (#195).

# 0.3.6 (March 13, 2019)

### Fixed
- fix build on newer rustc versions (#193).

### Added
- `Extract` implementation for `serde_json::Value` (#191).

# 0.3.5 (February 25, 2019)

### Added
- Try to detect response content-type (#187).

# 0.3.4 (January 25, 2019)

### Added
- Support extracting a string from a body (#158).
- `rustls` optional support (#160).
- Log 4xx responses (#164).
- `Response` implementation for `Result` (#163).
- Support handlers with large numbers of arguments (#170).
-  RFC7807: Problem details for HTTP APIs (#171).

### Fixed
- Fix build on older Rust versions (#169, #172).
- Parse `Content-Type` header correctly (#179).

# 0.3.3 (November 17, 2018)

* Allow template directory to be specified with env var (#139).
* Implement `Response` for `Option` and `Vec` (#150).
* Use 8 KiB as default chunk size when streaming files (#152).
* Misc codegen tweaks (#155, #151).

# 0.3.2 (October 18, 2018)

* Support generics on response types (#144)
* Support generics on resource types (#143)
* Percent-decode Strings and PathBufs (#108)

# 0.3.1 (October 10, 2018)

* Fix panic when content-type not provided (#123).
* Implement `Extract` for all numeric types (#131).
* Ignore attributes for other derives (#130).
* Avoid clone when logging disabled (#126).
* Add non-blocking `serve` method to run server (#76).

# 0.3.0 (September 28, 2018)

* Add experimental async/await support (#119).
* Add template support (#115).
* Fix potential int overflow when extracting numbers (#110).

# 0.2.2 (September 7, 2018)

* Add #[web(either)] to delegate Response to enum variants (#97)
* Add deflate middleware (#101)
* Add support for service level configuration (#98)

# 0.2.1 (August 30, 2018)

* Add CORS middleware (#61)
* Support for application/x-www-form-urlencoded (#84).

# 0.2.0 (August 14, 2018)

* Enable true attributes on stable Rust (#59).
* Rename HTTP trait alias functions (#64).

# 0.1.2 (August 9, 2018)

* Switch docs to S3.

# 0.1.1 (August 9, 2018)

* Allow warnings to make docs.rs happy.

# 0.1.0 (August 9, 2018)

* Initial release
