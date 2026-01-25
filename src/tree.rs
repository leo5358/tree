use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::args::Args;

pub fn print_tree(path: &Path, prefix: &str, args: &Args, exclude_set: &HashSet<String>, current_depth: u32) {
    if let Some(max_depth) = args.depth {
        if current_depth >= max_depth { return; }
    }

    let entries = match fs::read_dir(path) {
        Ok(read_dir) => {
            let mut list: Vec<_> = read_dir
                .filter_map(|e| e.ok())
                .filter(|entry| {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if !args.all && name.starts_with('.') { return false; }
                    if exclude_set.contains(&name) { return false; }
                    true
                })
                .collect();
            list.sort_by_key(|e| e.file_name());
            list
        }
        Err(_) => return,
    };

    let count = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        let is_last = i == count - 1;
        let metadata = entry.metadata().ok();
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();

        let icon = get_icon(&path);

        let size_str = if args.size {
            if let Some(meta) = metadata {
                if meta.is_file() {
                    format!(" [{}]", format_size(meta.len()))
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        let char_prefix = if is_last { "└── " } else { "├── " };
        println!("{}{}{} {}{}", prefix, char_prefix, icon, name, size_str);

        if path.is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree(&path, &new_prefix, args, exclude_set, current_depth + 1);
        }
    }
}

fn get_icon(path: &Path) -> &'static str {
    if path.is_dir() {
        return "\u{f07b}"; 
    }

    let extension = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match extension {
        "rs" => "\u{e7a8}",    //  (Rust)
        "md" => "\u{f48a}",    //  (Markdown)
        "toml" => "\u{e60b}",  //  (Configuration/TOML)
        "lock" => "\u{f023}",  //  (Lock file)
        "gitignore" => "\u{f1d3}", //  (Git)
        "py" => "\u{e606}",    //  (Python)
        "js" | "ts" => "\u{e781}", //  (JS/TS)
        _ => "\u{f15b}",       //  (Default File)
    }
}

/// 將 Byte 轉換為人類易讀的格式
fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    }
}