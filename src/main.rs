extern crate picture_compression;

use std::path::PathBuf;

use picture_compression::{dir_compression, file_compression};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "param", about = "cli param")]
struct Cli {
    /// 处理文件夹路径
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// 输入文件路径
    #[structopt(parse(from_os_str))]
    out_dir: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    if !args.out_dir.is_dir() {
        panic!("output directory:{:?}", args.out_dir.to_str());
    }

    if args.path.is_file() {
        println!("input files:{:?}", args.path.to_str());
        file_compression(&args.path.as_path(), &args.out_dir.as_path());
    } else if args.path.is_dir() {
        println!("input directory:{:?}", args.path.to_str());
        dir_compression(&args.path.as_path(), &args.out_dir.as_path());
    } else {
        panic!("path is not valid file or directory");
    }
}