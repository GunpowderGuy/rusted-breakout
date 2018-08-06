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

    let bloque = storage.alloc_id();
    storage
        .posicion
        .insert(bloque, mundo::Posicion { x: 25, y: 25 });

    let mut render = screen::nuevo();
    loop {
        render.renderingSystem(&mut storage);
        mundo::controlar(&mut storage, render.get_input())
    }

    //  screen::myendwin();
}
