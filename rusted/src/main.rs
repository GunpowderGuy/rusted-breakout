extern crate pancurses;
use std::{thread, time};
use pancurses::{endwin, initscr, noecho, Input};

fn delay() -> std::time::Duration {
    let delay: std::time::Duration = time::Duration::from_millis(10); // const delay :i64 = 3000;
    return delay;
}

struct Pelota {
    x: i64,
    y: i64,
    next_x: i64,
    next_y: i64,
    direction_x: i64,
    direction_y: i64,
}

fn mover_pelota(input: &mut Pelota, max_x: i64, max_y: i64) {
    input.next_x = input.x + input.direction_x;
    input.next_y = input.y + input.direction_y;

    if input.next_x >= max_x || input.next_x < 0 {
        input.direction_x *= -1;
    } else {
        input.x = input.x + input.direction_x;
    }

    if input.next_y >= max_y || input.next_y < 0 {
        input.direction_y *= -1;
    } else {
        input.y = input.y + input.direction_y;
    }
}

fn main() {
    let mut local = Pelota {
        direction_x: 1,
        direction_y: 1,
        x: 0,
        y: 0,
        next_x: 0,
        next_y: 0,
    };

    let stdscr = initscr();
    noecho();

    let max_x = stdscr.get_max_x() as i64; // uso los metodos del tipo SCREEn
    let max_y = stdscr.get_max_y() as i64; // en la version original enviaba la pantalla como argumento

    loop {
        stdscr.clear();
        stdscr.mvprintw(local.y as i32, local.x as i32, "o");
        stdscr.refresh();

        thread::sleep(delay()); // usleep(DELAY);

        mover_pelota(&mut local, max_x, max_y)
    }
    endwin();
}
