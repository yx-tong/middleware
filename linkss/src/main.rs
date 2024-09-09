#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // 注意：Rust 为编译型语言，直接修改代码不能直接生效！请在控制台右上角“导出代码”，然后根据 README.md 中的说明编译代码并重新上传。
    // 注意：Rust 为编译型语言，直接修改代码不能直接生效！请在控制台右上角“导出代码”，然后根据 README.md 中的说明编译代码并重新上传。
    // 注意：Rust 为编译型语言，直接修改代码不能直接生效！请在控制台右上角“导出代码”，然后根据 README.md 中的说明编译代码并重新上传。
    // Notice: You need to complie the code first otherwise the code change will not take effect.
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}