use std::fs;
use std::path::Path;

fn main() {
    //from the current directory 
    //TODO : choose a different directory by argument
    let path = ".";
    println!("{}", path);
    if let Err(e) = print_tree(Path::new(path), "") {
        eprintln!("Error: {}", e);
    }
}

fn print_tree(dir: &Path, prefix: &str) -> std::io::Result<()> {
    let entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .collect();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().into_owned();

        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix, connector, file_name);

        if path.is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree(&path, &new_prefix)?;
        }
    }
    Ok(())
}