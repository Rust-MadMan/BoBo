use crate::config::*;
use hyper::header::HeaderMap;



trait SetHeader {
    fn setheader(&self) -> HeaderMap;
}

struct Unparameter{
    headers:HeaderMap,
}
struct Parameter{
    headers:HeaderMap,
}

// 拆分无惨
impl SetHeader for Unparameter{
    fn setheader(&self) -> HeaderMap {
        self.headers = HeaderMap::new();
        self.headers.insert("HOST", "text/html; charset=utf-8".to_string().parse().unwrap());
        self.headers.insert("CONTENT_LENGTH", "".to_string().parse().unwrap());
        return self.headers;
        // headers.remove("HOST");
    }
}

//拆分有参
impl SetHeader for Parameter{
    fn setheader(&self) -> HeaderMap {
        self.headers = HeaderMap::new();
        self.headers.insert("HOST", "text/html; charset=utf-8".to_string().parse().unwrap());
        self.headers.insert("CONTENT_LENGTH", "".to_string().parse().unwrap());
        return self.headers;
        // headers.remove("HOST");
    }
    // fn setheader(&self) -> HeaderMap {
    //     self.headers = HeaderMap::new();
    //     self.headers.insert("HOST", "text/html; charset=utf-8".to_string().parse().unwrap());
    //     self.headers.insert("CONTENT_LENGTH", "".to_string().parse().unwrap());
    //     return self.headers;
    //     // headers.remove("HOST");
    // }
}




// pub fn set () ->  HeaderMap {
//     let mut headers = HeaderMap::new();
//     headers.insert("HOST", "text/html; charset=utf-8".to_string().parse().unwrap());
//     headers.insert("CONTENT_LENGTH", "".to_string().parse().unwrap(),
//     );
//     return headers;
//     // headers.remove("HOST");
// }