use pancurses::{endwin, initscr, noecho, Input, Window};

use mundo;
use std::thread;
use std::time;

pub fn setscreen() -> CursesSystem {
    let stdscr = initscr();
    noecho();

    //*x = stdscr.get_max_x();
    //*y = stdscr.get_max_y();

    stdscr.nodelay(true);
    stdscr.keypad(true);

    return stdscr;
}

pub fn myendwin() {
    endwin();
}
pub const DELAY: time::Duration = time::Duration::from_millis(10);

type CursesSystem = Window;
//pub struct CursesSystem(Window);

pub fn rendering_system(pantalla: &mut CursesSystem, mundo: &mut mundo::Storage) {
    pantalla.clear();

    for id in mundo.ids_collected() {
        if let Some(b) = mundo.visible.get_opt(id) {
            if let Some(a) = mundo.posicion.get_opt_mut(id) {
                //    pantalla.mvprintw(a.y as i32, a.x as i32, format!("{}", a.x));
                pantalla.mvprintw(a.y as i32, a.x as i32, b);
            }
        }
    }
    pantalla.refresh();
    thread::sleep(DELAY); // usleep(DELAY);
}
