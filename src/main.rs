use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // ここは参照で借用を起こさないとエラーになる。indexアクセスは参照ではなく本体へアクセスする。そして、StringはCopyトレイトが無いので、letに対してmoveを引き起こす。しかしVecは一部の値だけがmoveを起こすのを許容しないのでエラーになる
    let filename = &args[2];

    // {}を探しています
    println!("Searching for {}", query);
    // {}というファイルの中
    println!("In file {}", filename);

    // find file
    let mut f = File::open(filename).expect("file not found");

    // read file
    let mut contents = String::new();
    f.read_to_string(&mut contents) // read_to_stringにおいてファイルディスクリプタはmutableである。OSによってはファイル読み込みによってFDがガチで変更されるから
        .expect("something went wrong");

    // テキストは...です
    println!("With text:\n{}", contents);
}
