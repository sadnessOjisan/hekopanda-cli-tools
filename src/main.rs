use hekopanda::slot::Slot;
use clap::Parser;
use std::{io::stdin, sync::mpsc::channel, thread::spawn};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Number of times to greet
    #[clap(short, long)]
    slot: bool
}

fn main() {
    let args = Cli::parse();
    let mut slot = Slot::new();
    let (tx, rx) = channel::<bool>();
    spawn(move || loop {
        let received = rx.try_recv();
        slot.spin();
        match received {
            Ok(should_stop) => {
                if should_stop {
                    slot.stop();
                }
                if slot.is_finish() {
                    break;
                }
            }
            Err(_) => {}
        }
        print!(
            "\r{}{}{}🐼",
            slot.output.0.num, slot.output.1.num, slot.output.2.num
        );
    });

    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line.");
        let _ = tx.send(true);
    }
}
