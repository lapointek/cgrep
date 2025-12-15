use clap::Parser;

// Search for a pattern and display the lines
#[derive(Parser)]
struct Cli {
    // Pattern to look for in file
    pattern: String,
    // Read file path
    path: std::path::PathBuf,
}

fn main() {
    // Parse into the Cli struct
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
