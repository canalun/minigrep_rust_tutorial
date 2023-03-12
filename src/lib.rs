use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // find file
    let mut f = File::open(config.filename)?;

    // read file
    let mut contents = String::new();
    f.read_to_string(&mut contents)?; // read_to_stringにおいてファイルディスクリプタはmutableである。OSによってはファイル読み込みによってFDがガチで変更されるから

    let results = if config.case_sensitive {
        search(config.query, &contents)
    } else {
        search_case_insensitive(config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

// argsを解析することは、それ自体がconfigの生成だと言えるのでnewを実装
impl Config<'_> {
    // ライフタイムは、引数に自動で設定され、ライフタイムが1つしか設定されないゆえに、それが自動で出力にも設定される。なので表記不要。
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        //                        ^^^ エラーハンドリングはpanic使わず、result型で返す
        if args.len() < 3 {
            return Err("not enough args");
        }

        // 下記argsへのアクセスは参照で借用を起こさないとエラーになる。indexアクセスは参照ではなく本体へアクセスする。そして、StringはCopyトレイトが無いので、letに対してmoveを引き起こす。しかしVecは一部の値だけがmoveを起こすのを許容しないのでエラーになる
        let query = &args[1];
        let filename = &args[2];

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Duct";
        let contents = "
Rust:
safe, fast, productive.
ducT
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive.", "ducT"],
            search_case_insensitive(query, contents)
        );
    }
}

// contentsの部分を返すので、ライフタイムはcontentsと返り値で共通になる
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
}
