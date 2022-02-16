
pub struct Foo {
    access_control_allow_origin: String,
    // 访问白名单设置
    content_type: String,
    // 网络返回编码类型
    date: String,
    // 日期
    connection: String,
    // 连接保持方式，也可以通过设置close关闭
    keep_alive: String,
    // 通道保持5秒自动断开 ，通道最多传输300个情况自动断开，
    last_modified: String,
    server: String,
    set_cookie: String,
    // 才贵cookie设置
    content_length: String,
    // 请求长度
    content_encoding: String,
    //压缩模式
    transfer_encoding: String,
    // 分块模式反馈gzip，否则先把整个压缩后的文本数据存储到一个数组中，然后计算出Content-Length
    x_frame_options: String 
    // 主要解决站点安全性问题的字段                        
    // DENY:不能被嵌入到任何iframe或者frame中。
    // SAMEORIGIN:页面只能被本站页面嵌入到iframe或者frame中
    // ALLOW-FROM uri:只能被嵌入到指定域名的框架中
}

impl Foo {
    fn get_access_control_allow_origin(&self) {
        self.access_control_allow_origin;
    }
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            access_control_allow_origin: "*".to_string(),
            // 访问白名单设置
            content_type: "text/html; charset=utf-8".to_string(),
            // 网络返回编码类型
            date: "Mon, 18 Jul 2016 16:06:00 GMT".to_string(),
            // 日期
            connection: "Keep-Alive".to_string(),
            // 连接保持方式，也可以通过设置close关闭
            keep_alive: "timeout=5, max=300".to_string(),
            // 通道保持5秒自动断开 ，通道最多传输300个情况自动断开，
            last_modified: "".to_string(),
            server: "".to_string(),
            set_cookie: "".to_string(),
            // 才贵cookie设置
            content_length: "".to_string(),
            // 请求长度
            content_encoding: "gzip".to_string(),
            //压缩模式
            transfer_encoding: "chunked".to_string(),
            // 分块模式反馈gzip，否则先把整个压缩后的文本数据存储到一个数组中，然后计算出Content-Length
            x_frame_options: "DENY".to_string(), // 主要解决站点安全性问题的字段
                                                 // DENY:不能被嵌入到任何iframe或者frame中。
                                                 // SAMEORIGIN:页面只能被本站页面嵌入到iframe或者frame中
                                                 // ALLOW-FROM uri:只能被嵌入到指定域名的框架中
        }
    }
}



pub trait With<T> {
    fn with(value:T) -> Self;
}


impl With<String> for Foo {
    fn with(x: String) -> Self {
        Foo {
            server: "http://www.sb1.com".to_string(),
            ..Default::default()
        }
    }
}

impl With<i32> for Foo {
    fn with(x: i32) -> Self {
        Foo {
            server: "http://www.sb2.com".to_string(),
            ..Default::default()
        }
    }
}

impl With<bool> for Foo {
    fn with(x: bool) -> Self {
        Foo {
            server: "http://www.sb3.com".to_string(),
            ..Default::default()
        }
    }
}





// 拆分无参

// impl SetHeader for Unparameter{
//     fn setheader(&self) -> HeaderMap {
//         self.headers = HeaderMap::new();
//         self.headers.insert("HOST", "text/html; charset=utf-8".to_string().parse().unwrap());
//         self.headers.insert("CONTENT_LENGTH", "".to_string().parse().unwrap());
//         return self.headers;
//         // headers.remove("HOST");
//     }
// }

// //拆分有参
// impl SetHeader for Parameter{
//     fn setheader(&self) -> HeaderMap {
//         self.headers = HeaderMap::new();
//         self.headers.insert("HOST", "text/html; charset=utf-8".to_string().parse().unwrap());
//         self.headers.insert("CONTENT_LENGTH", "".to_string().parse().unwrap());
//         return self.headers;
//         // headers.remove("HOST");
//     }
// }




// pub fn set () ->  HeaderMap {
//     let mut headers = HeaderMap::new();
//     headers.insert("HOST", "text/html; charset=utf-8".to_string().parse().unwrap());
//     headers.insert("CONTENT_LENGTH", "".to_string().parse().unwrap(),
//     );
//     return headers;
//     // headers.remove("HOST");
// }