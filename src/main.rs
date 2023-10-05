use log::debug;
use walkdir::WalkDir;

use jib::{
    get_args,
    lexer::{Lexer, TokenType},
};

fn main() {
    env_logger::init();
    let args = get_args();

    for entry in WalkDir::new(&args.directory)
        .into_iter()
        .map(|e| e.expect("should find a file or directory"))
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().unwrap_or_default() == "jib")
    {
        let filepath = entry.path();
        debug!("Opening file: `{}`", filepath.display());

        let mut lexer = Lexer::new(filepath);
        loop {
            let token = lexer.get_token().expect("should get a token");
            debug!("{:?}", token);
            if token.token_type == TokenType::EndOfFile {
                break;
            }
        }
    }
}
