use crate::config::*;
use hyper::header::HeaderMap;

pub fn set(config: ConfigHeader) {
    let mut headers = HeaderMap::new();
    headers.insert("HOST", config.content_type.to_string().parse().unwrap());
    headers.insert(
        "CONTENT_LENGTH",
        config.content_encoding.to_string().parse().unwrap(),
    );
    println!("{:?}", headers);
    // headers.remove("HOST");
}
