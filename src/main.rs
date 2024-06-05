use std::env;

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a+$b
    };
    ($a:expr) => {
        $a
    }
}

macro_rules! add_all {
    ($($a:expr), *) => {
        {
            0
            $( + $a)*
        }
    };
}

// .\target\debug\first 127.0.0.0:8000   或   cargo run -- 127.0.0.0:8000
fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or("127.0.0.1:8888".to_string());

    println!("{}", addr);


    let x = 0;
    let sum = add!(1,2);
    println!("{}", sum);
    let sum = add!(x);
    println!("{}", sum);

    let sum = add_all!(1,2,3);
    println!("{}", sum);
}