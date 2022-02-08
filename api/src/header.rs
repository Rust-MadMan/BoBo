use crate::config::*;
use hyper::header::HeaderMap;

pub fn set (config: ConfigHeader) ->  HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("HOST", config.content_type.to_string().parse().unwrap());
    headers.insert("CONTENT_LENGTH",config.content_encoding.to_string().parse().unwrap(),
    );
    return headers;
    // headers.remove("HOST");
}
