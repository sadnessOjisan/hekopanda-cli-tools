use std::sync::Arc;

use channel::unbounded;
use crossbeam::{
    self,
    channel::{self, Receiver, Sender},
};
use tokio::{self, spawn};
#[derive(Debug)]
pub struct NumState {
    pub num: u8,
    is_stoped: bool,
}

#[derive(Debug)]
pub struct Slot {
    pub tx: Sender<bool>,
    rx: Receiver<bool>,
    pub output: (NumState, NumState, NumState),
}

pub enum Position {
    First,
    Second,
    Third,
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

    pub fn spin(&mut self) {
        if (!self.output.0.is_stoped) {
            let new_num = self.increment_slot(self.output.0.num);
            self.output.0.num = new_num;
        }
        if !self.output.1.is_stoped {
            let new_num = self.increment_slot(self.output.1.num);
            self.output.1.num = new_num;
        }
        if (!self.output.2.is_stoped) {
            let new_num = self.increment_slot(self.output.2.num);
            self.output.2.num = new_num;
        }
    }

    pub fn stop(&mut self, pos: Position) {
        match pos {
            Position::First => {
                self.output.0.is_stoped = true;
            }
            Position::Second => {
                self.output.1.is_stoped = true;
            }
            Position::Third => {
                self.output.2.is_stoped = true;
            }
        }
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
