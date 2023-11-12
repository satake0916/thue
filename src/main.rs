use std::collections::HashMap;

use clap::{ArgGroup, Parser};


#[derive(Parser)]
#[clap(
    name = "thue",
    author = "satake makoto",
    version = "v0.1",
    about = "Translate day of week japanese and english",
)]
#[clap(group(ArgGroup::new("ja-or-en")
    .arg("ja")
    .conflicts_with("en"))
)]
struct AppArg {
    #[clap(short ='j', long="ja")]
    ja: Option<String>,

    #[clap(short = 'e', long = "en")]
    en: Option<String>
}

fn main() {
    
    let en_ja_map: HashMap<String, String> = HashMap::from([
        (String::from("sun"), String::from("日曜日")),
        (String::from("mon"), String::from("月曜日")),
        (String::from("tue"), String::from("火曜日")),
        (String::from("wed"), String::from("水曜日")),
        (String::from("thu"), String::from("木曜日")),
        (String::from("fri"), String::from("金曜日")),
        (String::from("sat"), String::from("土曜日"))
    ]);

    let arg: AppArg = AppArg::parse();
    if arg.en.is_some() {
        println!("{}", en_ja_map.get(&arg.en.unwrap()).unwrap())
    }
}
