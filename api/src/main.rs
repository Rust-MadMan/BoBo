mod config;
mod entry;
mod header;

use config::*;
use header::*;

// use entry::*;
pub fn main() {
    let default_config_header = ConfigHeader {
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
    set(default_config_header);
    // startup();
}
