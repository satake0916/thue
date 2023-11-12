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
    let arg: AppArg = AppArg::parse();
    println!(
        "{}:{}",
        arg.ja.unwrap_or(String::from("ja nai")),
        arg.en.unwrap_or(String::from("en nai"))
    )
}
