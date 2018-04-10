#[macro_use]
extern crate derive_new;

extern crate pancurses;

mod objetos;
mod screen;

const NBLOQUES: usize = 10;

fn main() {
    let puntaje = 0;
    let mut local = objetos::Pelota::new(1, 1, 0, 0, 1, 1);
    let mut jugador = objetos::Bloque::new(25, 25, 10);

    //    let mut bloques: Vec<objetos::Bloque> = vec![jugador; NBLOQUES]; // Vec::new();
    //    for i in 0..(NBLOQUES - puntaje) {
    //        bloques[i].x = bloques[1].x + (i * 5) as i64;
    //    }

    let mut max_x = 0; // uso los metodos del tipo SCREEn
    let mut max_y = 0; // en la version original enviaba la pantalla como argumento
    let stdscr = screen::setscreen(&mut max_x, &mut max_y);

    while jugador.mover(stdscr.getch()) == true {
        stdscr.clear();

        stdscr.mvprintw(local.y as i32, local.x as i32, "o");
        stdscr.mvprintw(jugador.y as i32, jugador.x as i32, "0123456789");
        //for i in 0..(NBLOQUES - puntaje) {
        //        stdscr.mvprintw(bloques[i].y as i32, bloques[i].x as i32, "0");
        //}

        stdscr.refresh();

        std::thread::sleep(screen::delay()); // usleep(DELAY);

        local.mover(max_x, max_y);
        local.comparar(jugador);
    }

    screen::myendwin();
}
