use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    folder: PathBuf,

    #[arg(short, long, default_value_t = 3)]
    depth: usize,

    #[arg(short, long, default_value_t = '=')]
    separator: char,
}

fn main() {
    let args = Args::parse();

    let folder = args.folder;
    let depth = args.depth;
    let separator = args.separator;

    // Verify folder exists
    if folder.is_dir() {
        let _result = process_folder(&folder, 0, depth, separator);
    } else {
        eprintln!("{:#?} does not exist; exiting", folder);
    }
}

fn is_valid_dir(path: &PathBuf) -> bool {
    let is_hidden = path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.'))
        .unwrap_or(false);

    if path.is_dir() && !is_hidden {
        return true;
    } else {
        return false;
    }
}

fn process_folder(
    folder: &PathBuf,
    depth: usize,
    max_depth: usize,
    separator: char,
) -> std::io::Result<()> {
    if depth >= max_depth {
        return Ok(());
    }

    let mut prefix = separator.to_string().repeat(depth);
    if depth > 0 {
        prefix.push(' ');
    }

    for entry in fs::read_dir(folder)? {
        let entry = entry.unwrap();
        let path = entry.path();
        if is_valid_dir(&path) {
            println!("{}{}", prefix, path.file_name().unwrap().to_str().unwrap());
            process_folder(&path, depth + 1, max_depth, separator)?;
        }
    }

    Ok(())
}
