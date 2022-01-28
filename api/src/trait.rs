pub trait Api{
    fn apply();
    // 从 engine 中生成目标图片，注意这里用的是 self，而非 self 的引用
    fn hello() -> String;

}
