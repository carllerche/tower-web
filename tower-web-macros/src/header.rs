use http::header::HeaderName;

pub(crate) fn arg_to_header_name(arg: &str) -> HeaderName {
    let header = arg.replace("_", "-").to_lowercase();
    header.parse().unwrap()
}
