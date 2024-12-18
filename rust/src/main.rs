use clap::Parser;

mod days;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => days::day01::solve(),
        2 => days::day02::solve(),
        3 => days::day03::solve(),
        _ => eprintln!("Day {} not implemented", args.day),
    }
}