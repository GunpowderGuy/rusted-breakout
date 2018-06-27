use pancurses::{endwin, initscr, noecho, Input, Window};

use mundo;
use std::thread;
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

pub struct cursesSystem {
    max_x: i64,
    max_y: i64,
    stdscr: Option<Window>,
}
pub fn nuevo() -> cursesSystem {
    return cursesSystem {
        max_x: 0,
        max_y: 0,
        stdscr: None,
    };

    impl cursesSystem {
        pub fn renderingSystem(&mut self, mundo: &mut mundo::Storage) -> Option<Input> {
            let out;
            match self.stdscr {
                None => self.stdscr = Some(setscreen(&mut self.max_x, &mut self.max_y)),
                _ => (), // does nothing if there is already some value
            }
            if let Some(ref pantalla) = self.stdscr {
                out = pantalla.getch();
                pantalla.clear();

                for id in mundo.ids_collected() {
                    if let Some(a) = mundo.posicion.get_opt_mut(id) {
                        pantalla.mvprintw(a.x as i32, a.y as i32, "0123456789");
                    }
                }
                pantalla.refresh();
                thread::sleep(delay()); // usleep(DELAY);
                return out;
            } else {
                return None;
            }
        }
    }
}
