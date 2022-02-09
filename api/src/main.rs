mod config;
mod header;

use config::*;
use header::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
 


fn getheader(){
    ConfigHeader {
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
    };
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
 
    // 通过req.method()和req.uri().path()来识别方法和请求路径
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
            *response.headers_mut() = HeaderMap::
        },
        (&Method::GET, "/get") => {
            *response.body_mut() = Body::from("welcome index ");
        },
        (&Method::POST, "/post") => {
            // 将POST的内容保持不变返回
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/post/uppercase") => {
            // 把请求stream中的字母都变成大写，并返回
            let mapping = req.into_body();

            // 把stream变成body
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        (&Method::POST, "/post/reverse") => {
            // 这里需要完整的body，所以需要等待全部的stream并把它们变为bytes
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
 
            // 把body逆向
            let reversed = full_body.iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();
 
            *response.body_mut() = reversed.into();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
 
    Ok(response)
}
 
#[tokio::main]
async fn main() {
    
    println!("{:?}", set(getheader()));

    let addr = ([127, 0, 0, 1], 3000).into();
    println!("Listening on http://{}", addr); 
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(echo))
    });
 
    let server = Server::bind(&addr).serve(make_svc);
 
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }



}