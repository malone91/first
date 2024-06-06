fn main() {
    let url = "https://www.rust-lang.org/";

    println!("fetching url {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("response body html {}", body);
}