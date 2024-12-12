use hyper::Uri;

pub fn format_uri_to_dimension(uri: &Uri) -> String {
    let mut uri_as_string = uri.to_string();
    uri_as_string.remove(0);
    let slash_offset = uri_as_string.find('/').unwrap_or(uri_as_string.len());
    let mut dimension: String = uri_as_string.drain(..slash_offset).collect();
    dimension.remove(dimension.len() - 1);
    dimension
}
