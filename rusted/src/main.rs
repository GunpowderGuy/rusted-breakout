#[macro_use]
extern crate derive_new;

extern crate pancurses;
use pancurses::{endwin, initscr, noecho};

use std::{thread, time};
mod objetos;

fn delay() -> std::time::Duration {
    let delay: std::time::Duration = time::Duration::from_millis(10); // const delay :i64 = 3000;
    return delay;
}

fn main() {
    let mut local = objetos::Pelota::new(1, 1, 0, 0, 1, 1);
    let mut jugador = objetos::Bloque::new(25, 25);

    let stdscr = initscr();
    noecho();

    let max_x = stdscr.get_max_x() as i64; // uso los metodos del tipo SCREEn
    let max_y = stdscr.get_max_y() as i64; // en la version original enviaba la pantalla como argumento

    stdscr.nodelay(true);
    stdscr.keypad(true);

    while jugador.mover(stdscr.getch()) == true {
        stdscr.clear();

        stdscr.mvprintw(local.y as i32, local.x as i32, "o");
        stdscr.mvprintw(jugador.y as i32, jugador.x as i32, "o");

        stdscr.refresh();

        thread::sleep(delay()); // usleep(DELAY);

        local.mover(max_x, max_y);
    }

    endwin();
}
