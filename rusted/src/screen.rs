use pancurses::{endwin, initscr, noecho, Input, Window};

use mundo;
use std::thread;
use std::time;

pub fn setscreen(x: &mut i32, y: &mut i32) -> Window {
    let stdscr = initscr();
    noecho();

    *x = stdscr.get_max_x();
    *y = stdscr.get_max_y();

    stdscr.nodelay(true);
    stdscr.keypad(true);

    return stdscr;
}

pub fn myendwin() {
    endwin();
}
pub const DELAY: time::Duration = time::Duration::from_millis(10);

pub struct CursesSystem {
    pub max_x: i32,
    pub max_y: i32,
    stdscr: Option<Window>,
}
pub fn nuevo() -> CursesSystem {
    return CursesSystem {
        max_x: 0,
        max_y: 0,
        stdscr: None,
    };

    impl CursesSystem {
        pub fn get_input(&self) -> Option<Input> {
            let out;
            if let Some(ref pantalla) = self.stdscr {
                out = pantalla.getch();
            } else {
                return None;
            }
            return out;
        }

        pub fn rendering_system(&mut self, mundo: &mut mundo::Storage) {
            match self.stdscr {
                None => self.stdscr = Some(setscreen(&mut self.max_x, &mut self.max_y)),
                _ => (), // does nothing if there is already some value
            }

            if let Some(ref pantalla) = self.stdscr {
                pantalla.clear();

                for id in mundo.ids_collected() {
                    if let Some(b) = mundo.visible.get_opt_mut(id) {
                        if let Some(a) = mundo.posicion.get_opt_mut(id) {
                            pantalla.mvprintw(a.y as i32, a.x as i32, b);
                        }
                    }
                }
                pantalla.refresh();
                thread::sleep(DELAY); // usleep(DELAY);
            }
        }
    }
}
