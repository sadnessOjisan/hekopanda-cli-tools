/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_4.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Window creation and input example.
      Use the cursor keys to move the window
      around the screen.
*/

extern crate ncurses;

use hekopanda_cli_tools::slot::Slot;
use ncurses::*;
use std::io::{stdout, Write};

fn main() {
    let mut slot = Slot::new();
    loop {
        slot.spin();
        if slot.is_finish() {
            break;
        }
        stdout().flush();
        print!(
            "\r{}{}{}",
            slot.output.0.num, slot.output.1.num, slot.output.2.num
        );
    }
}
