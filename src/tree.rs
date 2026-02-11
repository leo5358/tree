use std::collections::HashSet;
use std::path::Path;
use std::fs;
use std::io;
use ignore::{WalkBuilder, DirEntry};
use colored::*; // 引入顏色套件
use crate::args::Args;

pub struct TreeStats {
    pub directories: u32,
    pub files: u32,
}

impl TreeStats {
    pub fn new() -> Self {
        TreeStats { directories: 0, files: 0 }
    }

    pub fn add(&mut self, other: TreeStats) {
        self.directories += other.directories;
        self.files += other.files;
    }
}

pub fn print_tree(path: &Path, prefix: &str, args: &Args, exclude_set: &HashSet<String>, current_depth: u32) -> TreeStats {
    let mut stats = TreeStats::new();

    if let Some(max_depth) = args.depth
        && current_depth >= max_depth {
            return stats;
        }

    let entries = collect_entries(path, prefix, args, exclude_set);
    let count = entries.len();
    
    for (i, entry) in entries.into_iter().enumerate() {
        let is_last = i == count - 1;
        let entry_path = entry.path();
        
        // 取得格式化後的顯示字串
        let label = format_node_label(&entry, args);
        
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix.truecolor(90, 90, 90), connector.truecolor(90, 90, 90), label);

        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            stats.directories += 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            stats.add(print_tree(entry_path, &new_prefix, args, exclude_set, current_depth + 1));
        } else {
            stats.files += 1;
        }
    }
    
    stats
}

fn collect_entries(path: &Path, prefix: &str, args: &Args, exclude_set: &HashSet<String>) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    //wallerbuilder 會自動處理 .gitignore 和 hidden 檔案，這裡我們只需要設定好參數即可
    let walker = WalkBuilder::new(path)
        .max_depth(Some(1))
        .hidden(!args.all)
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.path() == path { continue; }
                let name = entry.file_name().to_string_lossy();
                if exclude_set.contains(name.as_ref()) { continue; }
                entries.push(entry);
            }
            Err(e) => {
                print_error_message(prefix, e);
            }
        }
    }
    
    // 排序：目錄優先，然後依名稱排序 (可選，這裡維持純名稱排序)
    entries.sort_by_key(|e| e.file_name().to_os_string());
    entries
}

fn format_node_label(entry: &DirEntry, args: &Args) -> String {
    let path = entry.path();
    let name = entry.file_name().to_string_lossy();
    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
    let is_symlink = entry.path_is_symlink();

    // 1. 圖示
    let icon = if args.no_icon {
        String::new()
    } else {
        format!("{} ", get_icon(path, is_dir))
    };
    
    // 2. 名稱與顏色
    let mut display_name = if is_dir {
        name.blue().bold().to_string()
    } else if is_symlink {
        name.cyan().to_string()
    } else {
        name.to_string()
    };

    // 3. 符號連結目標 (Link -> Target)
    if is_symlink
        && let Ok(target) = fs::read_link(path) {
            display_name = format!("{} -> {}", display_name, target.to_string_lossy().truecolor(150, 150, 150));
        }

    // 4. 檔案大小
    let size_str = if args.size && !is_dir {
        entry.metadata().ok()
            .map(|m| format!(" [{}]", format_size(m.len())).truecolor(90, 90, 90).to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    format!("{}{}{}", icon, display_name, size_str)
}

fn print_error_message(prefix: &str, e: ignore::Error) {
    let msg = e.io_error()
        .map(|io_err| match io_err.kind() {
            io::ErrorKind::PermissionDenied => "Permission Denied",
            io::ErrorKind::NotFound => "Not Found",
            _ => "Access Error",
        })
        .unwrap_or("Unknown Error");
    
    println!("{}{} [{}]", prefix.truecolor(90, 90, 90), "└── ".truecolor(90, 90, 90), msg.yellow());
}

fn get_icon(path: &Path, is_dir: bool) -> &'static str {
    if is_dir {
        return "\u{f07b}"; //  (Folder)
    }

    let extension = path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    let file_name = path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    // 特殊檔名優先判斷
    match file_name {
        "Dockerfile" | "docker-compose.yml" => return "\u{f308}", // 
        "Makefile" => return "\u{e60d}", // 
        "Cargo.toml" | "Cargo.lock" => return "\u{e7a8}", // 
        ".gitignore" | ".gitattributes" => return "\u{f1d3}", // 
        _ => {}
    }

    // 副檔名判斷
    match extension.as_str() {
        "rs" => "\u{e7a8}",    //  (Rust)
        "py" => "\u{e606}",    //  (Python)
        "js" => "\u{e74e}",    //  (JS)
        "ts" | "tsx" => "\u{e628}", //  (TS)
        "html" => "\u{e736}",  //  (HTML)
        "css" => "\u{e749}",   //  (CSS)
        "scss" | "sass" => "\u{e74b}", //  (SASS)
        "json" => "\u{e60b}",  //  (JSON)
        "md" => "\u{f48a}",    //  (Markdown)
        "toml" | "yaml" | "yml" => "\u{e60b}", //  (Config)
        "c" | "h" => "\u{e61e}", //  (C)
        "cpp" | "hpp" | "cc" => "\u{e61d}", //  (C++)
        "go" => "\u{e627}",    //  (Go)
        "java" | "jar" => "\u{e738}", //  (Java)
        "sh" | "bash" | "zsh" => "\u{f489}", //  (Shell)
        "lock" => "\u{f023}",  //  (Lock)
        "zip" | "tar" | "gz" | "7z" => "\u{f410}", //  (Archive)
        "png" | "jpg" | "jpeg" | "svg" | "ico" => "\u{f1c5}", //  (Image)
        "pdf" => "\u{f1c1}",   //  (PDF)
        "txt" => "\u{f15c}",   //  (Text)
        _ => "\u{f15b}",       //  (Default File)
    }
}

fn format_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if size < 1024 {
        format!("{} B", size)
    } else if (size as f64) < MB {
        format!("{:.1} KB", size as f64 / KB)
    } else if (size as f64) < GB {
        format!("{:.1} MB", size as f64 / MB)
    } else {
        format!("{:.1} GB", size as f64 / GB)
    }
}