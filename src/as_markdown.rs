use std::env;

fn main() {
    // let url = "https://www.rust-lang.org/";  cargo run --bin as_markdown -- https:www.baidu.com
    // let args: Vec<String> = env::args().collect();
    // let url = &args[1];

    //与上面的都可以从命令中获取参数
    let url = env::args().nth(1).unwrap();

    println!("fetching url {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("response body html {}", body);

    // target\debug\as_markdown.exe
    // https:www.baidu.com
    for arg in std::env::args() {
        println!("{}", arg);
    }
}