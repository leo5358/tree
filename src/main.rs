mod args;
mod tree;

use args::Args;
use clap::Parser;
use std::collections::HashSet;
use std::path::Path;
use colored::*;

fn main() {
    let args = Args::parse();
    let root_path = Path::new(&args.path);
    let exclude_set: HashSet<String> = args.exclude.iter().cloned().collect();

    if !root_path.exists() {
        eprintln!("錯誤: 路徑 '{}' 不存在", args.path);
        return;
    }

    println!("{}", args.path);
    
    let stats = tree::print_tree(root_path, "", &args, &exclude_set, 0);
    print_summary(stats);
}

fn print_summary(stats: tree::TreeStats) {
    println!("\n{} directories, {} files", 
        stats.directories.to_string().blue().bold(), 
        stats.files.to_string().green().bold()
    );
}