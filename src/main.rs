mod cstring_extras;
mod models;
mod reader;
use crate::models::cli::Cli;
use clap::Parser;
use reader::Reader;

fn main() {
    let cli = Cli::parse();

    for project_path in cli.project_paths {
        let data = std::fs::read(project_path.clone()).unwrap();
        let reader = Reader::new(data);
        let project_details = reader.get_project_details();

        println!("{}", project_path);
        println!("{:#?}", project_details);
        println!();
    }
}
