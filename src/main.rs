use clap::Parser;
use log::debug;
use walkdir::WalkDir;

/// A Jib to Javascript compiler.
///
/// Set `RUST_LOG=debug` to enable debug logging.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// The source directory.
    #[arg(index = 1, default_value_t = String::from("./"))]
    directory: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    for entry in WalkDir::new(&args.directory) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() || entry.path().extension().unwrap_or_default() != "jib" {
            continue;
        }
        debug!("Checking file: {:?}", entry);
    }
}
