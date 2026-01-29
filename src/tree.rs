use std::collections::HashSet;
use std::path::Path;
use std::io;
use ignore::{WalkBuilder, DirEntry};
use crate::args::Args;

pub fn print_tree(path: &Path, prefix: &str, args: &Args, exclude_set: &HashSet<String>, current_depth: u32) {
    // 1. 深度檢查
    if let Some(max_depth) = args.depth {
        if current_depth >= max_depth { return; }
    }

    // 2. 收集並過濾該層級的檔案
    let entries = collect_entries(path, prefix, args, exclude_set);

    let count = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        let is_last = i == count - 1;
        let entry_path = entry.path();
        
        // 3. 格式化並印出當前行
        let label = format_node_label(&entry, args);
        let char_prefix = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix, char_prefix, label);

        // 4. 遞迴處理子目錄
        if entry_path.is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree(&entry_path, &new_prefix, args, exclude_set, current_depth + 1);
        }
    }
}

/// 負責處理檔案系統的走訪與錯誤回報
fn collect_entries(path: &Path, prefix: &str, args: &Args, exclude_set: &HashSet<String>) -> Vec<DirEntry> {
    let mut entries = Vec::new();
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
    // 依名稱排序
    entries.sort_by_key(|e| e.file_name().to_os_string());
    entries
}

/// 負責處理單一檔案或目錄的文字與圖示格式化
fn format_node_label(entry: &DirEntry, args: &Args) -> String {
    let path = entry.path();
    let name = entry.file_name().to_string_lossy();
    let icon = get_icon(path);
    
    let size_str = if args.size {
        entry.metadata().ok()
            .filter(|m| m.is_file())
            .map(|m| format!(" [{}]", format_size(m.len())))
            .unwrap_or_default()
    } else {
        String::new()
    };

    format!("{} {}{}", icon, name, size_str)
}

/// 專門處理錯誤訊息的顯示
fn print_error_message(prefix: &str, e: ignore::Error) {
    let msg = e.io_error()
        .map(|io_err| match io_err.kind() {
            io::ErrorKind::PermissionDenied => "Permission Denied",
            io::ErrorKind::NotFound => "Not Found",
            _ => "Access Error",
        })
        .unwrap_or("Unknown Error");
    
    println!("{}\x1b[33m└── [{}]\x1b[0m", prefix, msg);
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