use std::{env, fs, process::exit};

use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;

include!("src/cli.rs");

#[allow(clippy::expect_used)]
fn main() {
    let completion_path = env::var_os("SHELL_COMPLETIONS_DIR")
        .or_else(|| env::var_os("OUT_DIR"))
        .unwrap_or_else(|| exit(0));

    fs::create_dir_all(&completion_path).expect("unable to create completion path");

    let mut command = Cli::command();
    for shell in Shell::value_variants() {
        clap_complete::generate_to(
            *shell,
            &mut command,
            "cubase-project-plugins",
            &completion_path,
        )
        .expect("unable to generate completions");
    }
}
