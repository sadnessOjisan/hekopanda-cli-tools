use clap::Parser;
use hekopanda::{gravure::print, slot::do_slot};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// スロットモード
    #[clap(short, long)]
    slot: bool,
    /// グラビアモード
    #[clap(short, long)]
    gravure: bool,
}

fn main() {
    let args = Cli::parse();
    if args.slot {
        do_slot();
    } else if args.gravure {
        print();
    }
}
