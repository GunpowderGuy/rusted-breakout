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
    storage.controlable.insert(jugador, mundo::Controlable {});
    storage.visible.insert(jugador, { "j" });

    let bloque = storage.alloc_id();
    storage
        .posicion
        .insert(bloque, mundo::Posicion { x: 25, y: 25 });
    storage.visible.insert(bloque, { "b" });

    let pelota = storage.alloc_id();
    storage
        .rebota
        .insert(pelota, mundo::Rebota { direction: 1 });
    storage
        .posicion
        .insert(pelota, mundo::Posicion { x: 20, y: 20 });
    storage.visible.insert(pelota, { "p" });

    let mut render = screen::nuevo();

    let Gamedata = storage.alloc_id();

    loop {
        render.renderingSystem(&mut storage);
        mundo::rebotar(&mut storage, render.max_x, render.max_y);
        mundo::controlar(&mut storage, render.get_input())
    }
    //  screen::myendwin();
}
