use std::env;

const SOME_ENV_VAR: &str = "SOME_ENV_VAR";

fn main() {
    let value = env::var(SOME_ENV_VAR).unwrap_or_else(|_| String::from("no value found."));
    println!("value of the env var: {value}");
}
