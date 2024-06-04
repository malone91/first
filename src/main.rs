use std::env;

// .\target\debug\first 127.0.0.0:8000   或   cargo run -- 127.0.0.0:8000
fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8888".to_string());

    println!("{}", addr);
}