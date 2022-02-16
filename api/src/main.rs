mod config;
mod header;

use config::*;
use header::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
 



async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
 
    // 通过req.method()和req.uri().path()来识别方法和请求路径
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
            // *response.headers_mut() = set(getheader());
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
    let a:Foo = Foo::default();
    // println!("{:?}", a.get_access_control_allow_origin());

    
    // 启动服务
    // let addr = ([127, 0, 0, 1], 3000).into();
    // println!("Listening on http://{}", addr); 
    // let make_svc = make_service_fn(|_conn| async {
    //     Ok::<_, hyper::Error>(service_fn(echo))
    // });
 
    // let server = Server::bind(&addr).serve(make_svc);
 
    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }



}