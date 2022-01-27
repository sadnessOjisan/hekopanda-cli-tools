use std::{
    sync::{
        mpsc::{self, channel, Receiver, Sender},
        Arc,
    },
    thread::spawn,
};

#[derive(Debug)]
struct NumState {
    num: u8,
    is_stoped: bool,
}

#[derive(Debug)]
pub struct Slot {
    pub tx: Sender<bool>,
    rx: Receiver<bool>,
    output: (NumState, NumState, NumState),
}

impl Slot {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<bool>();
        Slot {
            tx,
            rx,
            output: (
                NumState {
                    num: 0,
                    is_stoped: false,
                },
                NumState {
                    num: 1,
                    is_stoped: false,
                },
                NumState {
                    num: 2,
                    is_stoped: false,
                },
            ),
        }
    }

    pub fn do_slot(self: &mut Self) {
        spawn(move || loop {
            let should_stop = self.rx.recv();
            if (!self.output.0.is_stoped) {
                self.increment_slot(self.output.0.num);
                if (should_stop.unwrap() == true) {
                    self.output.0.is_stoped = true;
                }
            }
            if !self.output.1.is_stoped {
                self.increment_slot(self.output.1.num);
                if (should_stop.unwrap() == true) {
                    self.output.1.is_stoped = true;
                }
            }
            if (!self.output.2.is_stoped) {
                self.increment_slot(self.output.2.num);
                if (should_stop.unwrap() == true) {
                    self.output.2.is_stoped = true;
                }
            }
        });
    }

    pub fn is_finish(&self) -> bool {
        self.output.0.is_stoped && self.output.1.is_stoped && self.output.2.is_stoped
    }

    fn increment_slot(&self, mut num: u8) -> u8 {
        if (num < 10) {
            num += 1;
        }
        if (num == 10) {
            num = 0;
        }
        num
    }
}
