extern crate clap;

use std::collections::HashMap;

use clap::Parser;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct AppArg {
    #[arg(help = "曜日の英語表記 / Day of the week in English")]
    en: Option<String>,
}

fn main() {
    let en_ja_list = [
        (String::from("sun"), String::from("日曜日")),
        (String::from("mon"), String::from("月曜日")),
        (String::from("tue"), String::from("火曜日")),
        (String::from("wed"), String::from("水曜日")),
        (String::from("thu"), String::from("木曜日")),
        (String::from("fri"), String::from("金曜日")),
        (String::from("sat"), String::from("土曜日")),
    ];

    let en_ja_map: HashMap<String, String> = HashMap::from(en_ja_list.clone());

    let arg: AppArg = AppArg::parse();
    if arg.en.is_some() {
        let input = arg.en.unwrap();
        let enceded = input.to_lowercase();
        println!(
            "{}",
            en_ja_map
                .get(&enceded)
                .unwrap_or(&(input + &String::from(" is not day of week")))
        );
    } else {
        en_ja_list.iter().for_each(|(en, ja)| {
            println!(
                "{}\t{}",
                en.chars().nth(0).unwrap().to_uppercase().to_string() + &en[1..] + &".",
                ja
            );
        })
    }
}
