use channel::unbounded;
use crossbeam::{
    self,
    channel::{self, Receiver, Sender},
};
use std::{sync::Arc, thread::spawn};

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
        let (tx, rx) = unbounded();
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
        // https://qiita.com/yasuyuky/items/0856343e087c65aa6ff4
        let a = crossbeam::scope(|scope| {
            scope.spawn(|_| {
                loop {
                    // println!("a");
                    let should_stop_ = self.rx.try_recv();
                    match should_stop_ {
                        Ok(should_stop_) => {
                            println!("{}",should_stop_);
                            if (!self.output.0.is_stoped) {
                                self.increment_slot(self.output.0.num);
                                if (should_stop_ == true) {
                                    self.output.0.is_stoped = true;
                                }
                            }
                            if !self.output.1.is_stoped {
                                self.increment_slot(self.output.1.num);
                                if (should_stop_ == true) {
                                    self.output.1.is_stoped = true;
                                }
                            }
                            if (!self.output.2.is_stoped) {
                                self.increment_slot(self.output.2.num);
                                if (should_stop_ == true) {
                                    self.output.2.is_stoped = true;
                                }
                            }
                        }
                        Err(_) => {
                            // println!("err");
                        }
                    }
                }
                // while let should_stop_= self.rx.try_recv(){

                // }
            });
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
