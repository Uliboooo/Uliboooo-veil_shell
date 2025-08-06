// use std::io::Write;

use serde::{Deserialize, Serialize};
use storable::Storable;

mod storable;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    apps: Vec<App>,
}

impl Storable for Config {}

#[derive(Debug, Serialize, Deserialize)]
struct App {
    command: String,
    args: Option<Vec<Arg>>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Arg {
    Positional(String),
    Optional(Optional),
}

#[derive(Debug, Serialize, Deserialize)]
/// if -arg is None, option is used as a flag
struct Optional {
    option: String,
    arg: Option<String>,
}

fn main() {
    println!("Hello, world!");
}
