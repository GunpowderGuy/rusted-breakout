use pancurses::Input;
use screen::cursesSystem;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Id(i32);

#[derive(Clone, Debug)]
pub struct Posicion {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct Colision {
    pub range: usize,
}

#[derive(Clone, Debug)]
pub struct Controlable {
    pub existe: bool,
}

pub fn controlar(mundo: &mut Storage, caracter: Option<Input>) {
    for id in mundo.ids_collected() {
        if let Some(b) = mundo.controlable.get_opt_mut(id) {
            if let Some(a) = mundo.posicion.get_opt_mut(id) {
                match caracter {
                    Some(Input::KeyUp) => a.x = a.x - 1,
                    Some(Input::KeyDown) => a.x = a.x + 1,
                    Some(Input::KeyRight) => a.y = a.y + 1,
                    Some(Input::KeyLeft) => a.y = a.y - 1,
                    //    Some(Input::KeyF12) => return false,
                    //    Some(Input::KeyEnter) => return false,
                    _ => (), // None => (),
                }
            }
        }
    }
}

zcomponents_storage!(Storage<Id>: {
    posicion : Posicion,
    colision : Colision,
    controlable : Controlable,
});
