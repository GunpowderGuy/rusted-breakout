extern crate pancurses;

mod mundo;
mod screen;

#[macro_use]
extern crate zcomponents;

fn main() {
    let mut storage = mundo::Storage::new();

    let jugador = storage.alloc_id();
    storage
        .posicion
        .insert(jugador, mundo::Posicion { x: 25, y: 10 });
    storage.controlable.insert(jugador, mundo::Controlable {});
    storage.visible.insert(jugador, { "jugador" });
    storage.paleta.insert(jugador, mundo::Paleta { rango: 7 });

    let bloque = storage.alloc_id();
    storage
        .posicion
        .insert(bloque, mundo::Posicion { x: 15, y: 35 });
    storage.visible.insert(bloque, { "b" });

    let pelota = storage.alloc_id();
    storage.rebota.insert(
        pelota,
        mundo::Rebota {
            vectorx: 1,
            vectory: 1,
        },
    );
    storage
        .posicion
        .insert(pelota, mundo::Posicion { x: 20, y: 20 });
    storage.visible.insert(pelota, { "p" });

    let mut render = screen::setscreen();

    while mundo::controlar(&mut storage, render.getch()) {
        screen::rendering_system(&mut render, &mut storage);
        mundo::rebotar(&mut storage, render.get_max_x(), render.get_max_y());
        mundo::rebotar_de_objeto(&mut storage)
    }
    screen::myendwin();
}
