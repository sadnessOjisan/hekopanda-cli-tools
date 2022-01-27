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

fn main() {
    // /* Setup ncurses. */
    initscr();
    raw();

    // /* Allow for extended keyboard (like F1). */
    // keypad(stdscr(), true);
    // noecho();

    // /* Invisible cursor. */
    // curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // /* Status/help info. */
    addstr("Let's start slot\n");
    // addstr("Use the arrow keys to move");
    refresh(); // 文字出力に必要
    let mut slot = Slot::new();
    loop {
        // getch();
        // addstr(&c.to_string());
        slot.spin();
        if slot.is_finish() {
            break;
        }
        insstr(
            format!(
                "{}{}{}",
                slot.output.0.num, slot.output.1.num, slot.output.2.num
            )
            .as_str(),
        );
        mv(0, 3);
        clrtoeol();
        mv(0, 0);
        refresh(); // 文字出
                   // c += 1;
    }
    endwin();
}
