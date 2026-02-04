use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust Tree CLI Tool")]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: String,

    /// 顯示隱藏檔案與目錄
    #[arg(short = 'a', long = "all", default_value_t = false)]
    pub all: bool,

    /// 限制顯示的深度
    #[arg(short = 'd', long = "depth")]
    pub depth: Option<u32>,

    /// 顯示檔案大小
    #[arg(short = 's', long = "size", default_value_t = false)]
    pub size: bool,

    /// 排除指定的目錄或檔案名稱
    #[arg(short = 'e', long = "exclude", num_args = 1..)]
    pub exclude: Vec<String>,

    /// 不顯示圖示 (適用於無 Nerd Fonts 環境)
    #[arg(short = 'n', long = "no-icon", default_value_t = false)]
    pub no_icon: bool,
}