use clap::{Parser, ValueEnum};

/// Chiikawa cli
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Charchacter
    #[arg(short, long, default_value = "chiikawa")]
    charchacter: Charchacter,
}

#[derive(ValueEnum, Clone, Debug)]
enum Charchacter {
    #[value(name = "chiikawa")]
    Chiikawa,

    #[value(name = "hachiware")]
    Hachiware,

    #[value(name = "usagi")]
    Usagi,
}

const CHIIKAWA: &str = include_str!("./chiikawa.txt");
const HACHIWARE: &str = include_str!("./hachiware.txt");
const USAGI: &str = include_str!("./usagi.txt");

fn main() {
    let args = Args::parse();

    match args.charchacter {
        Charchacter::Chiikawa => println!("{}", CHIIKAWA),
        Charchacter::Hachiware => println!("{}", HACHIWARE),
        Charchacter::Usagi => println!("{}", USAGI),
    }
}
