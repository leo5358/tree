use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple tree utility in Rust")]
struct Args {
    /// file size
    #[arg(short = 's', long)]
    size: bool,

    /// depth
    #[arg(short = 'd', long)]
    depth: Option<u32>,

    /// path
    #[arg(short = 'p', long, default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let path = &args.path;
    println!("{}", path.display());

    if let Err(e) = print_tree(path, "", 1, args.depth, args.size) {
        eprintln!("Error: {}", e);
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn print_tree(
    dir: &Path,
    prefix: &str,
    current_depth: u32,
    max_depth: Option<u32>,
    show_size: bool,
) -> std::io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .collect();
    
    entries.sort_by_key(|e| e.file_name());

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let file_name = entry.file_name().to_string_lossy().into_owned();

        let connector = if is_last { "└── " } else { "├── " };
        
        let size_info = if show_size {
            format!("[{:>10}] ", format_size(metadata.len()))
        } else {
            "".to_string()
        };

        println!("{}{}{}{}", prefix, connector, size_info, file_name);

        if metadata.is_dir() {
            let can_go_deeper = max_depth.map_or(true, |d| current_depth < d);
            if can_go_deeper {
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                print_tree(&path, &new_prefix, current_depth + 1, max_depth, show_size)?;
            }
        }
    }
    Ok(())
}