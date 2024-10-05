use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("cargo:rustc-env={}={}", key, value);
    }
}
