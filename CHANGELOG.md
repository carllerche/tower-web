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
