extern crate minigrep;
use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        //                                     ^^^ unwrap_or_elseはResult型をunwrapするか、失敗ならエラーで何かしてくれる
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 失敗だけをハンドリングしたいなら、if letでエラーを直接取得
    // let _ = match run(&config) {
    //     Ok(_) => (),
    //     Err(error) => {
    //         println!("Application error: {}", error);
    //         process::exit(1);
    //     }
    // };
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
