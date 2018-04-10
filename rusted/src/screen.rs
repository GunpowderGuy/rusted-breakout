use pancurses::{endwin, initscr, noecho, Window};

use std::time;

pub fn setscreen(x: &mut i64, y: &mut i64) -> Window {
    let stdscr = initscr();
    noecho();

    *x = stdscr.get_max_x() as i64;
    *y = stdscr.get_max_y() as i64;

    stdscr.nodelay(true);
    stdscr.keypad(true);

    return stdscr;
}

pub fn myendwin() {
    endwin();
}

// pub const DELAY: time::Duration = time::Duration::from_millis(10);
pub fn delay() -> time::Duration {
    let delay: time::Duration = time::Duration::from_millis(10); // const delay :i64 = 3000;
    return delay;
}
