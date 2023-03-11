use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // {}を探しています
    println!("Searching for {}", config.query);
    // {}というファイルの中
    println!("In file {}", config.filename);

    // find file
    let mut f = File::open(config.filename).expect("file not found");

    // read file
    let mut contents = String::new();
    f.read_to_string(&mut contents) // read_to_stringにおいてファイルディスクリプタはmutableである。OSによってはファイル読み込みによってFDがガチで変更されるから
        .expect("something went wrong");

    // テキストは...です
    println!("With text:\n{}", contents);
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

// argsを解析することは、それ自体がconfigの生成だと言えるのでnewを実装
impl Config<'_> {
    // ライフタイムは、引数に自動で設定され、ライフタイムが1つしか設定されないゆえに、それが自動で出力にも設定される。なので表記不要。
    fn new(args: &Vec<String>) -> Config {
        // 下記argsへのアクセスは参照で借用を起こさないとエラーになる。indexアクセスは参照ではなく本体へアクセスする。そして、StringはCopyトレイトが無いので、letに対してmoveを引き起こす。しかしVecは一部の値だけがmoveを起こすのを許容しないのでエラーになる
        let query = &args[1];
        let filename = &args[2];
        return Config { query, filename };
    }
}
