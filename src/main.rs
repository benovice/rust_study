mod api;
mod utils;

use std::env::args;

fn main() {
    let file_path = args().nth(1).expect("no pattern given");
    let path = args().nth(2).expect("no path given");

    println!("pattern: {:?}, path: {:?}", file_path, path)
}
