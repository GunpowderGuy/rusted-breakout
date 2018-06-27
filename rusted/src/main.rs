extern crate pancurses;

mod mundo;
mod screen;

#[macro_use]
extern crate zcomponents;

const NBLOQUES: usize = 1;

fn main() {
    let puntaje = 0;
    let mut storage = mundo::Storage::new();
    let jugador = storage.alloc_id();
    storage
        .posicion
        .insert(jugador, mundo::Posicion { x: 20, y: 20 });
    storage
        .controlable
        .insert(jugador, mundo::Controlable { existe: true });

    let mut render = screen::nuevo();
    loop {
        let a = render.renderingSystem(&mut storage);
        mundo::controlar(&mut storage, a)
    }

    //  screen::myendwin();
}
