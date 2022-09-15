mod cstring_extras;
mod models;
mod reader;
use std::env;

use reader::Reader;

fn main() {
    let path = env::args().nth(1).unwrap();
    let data = std::fs::read(path).unwrap();
    let reader = Reader::new(data);
    let project_details = reader.get_project_details();
    println!("{:#?}", project_details);
}
