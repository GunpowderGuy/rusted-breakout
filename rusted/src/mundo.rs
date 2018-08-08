use pancurses::Input;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Id(i32);

#[derive(Clone, Debug)]
pub struct Posicion {
    pub x: i32,
    pub y: i32,
}

type sprite = &'static str;

#[derive(Clone, Debug)]
pub struct Colision {
    pub range: usize,
}

#[derive(Clone, Debug)]
pub struct Controlable {}

#[derive(Clone, Debug)]
pub struct Rebota {
    pub direction: i32,
}

pub fn rebotar(mundo: &mut Storage, limitex: i32, limitey: i32) {
    for id in mundo.ids_collected() {
        if let Some(pelota) = mundo.rebota.get_opt_mut(id) {
            if let Some(a) = mundo.posicion.get_opt_mut(id) {
                if (a.x + pelota.direction) >= limitex || (a.x + pelota.direction) < 0 {
                    pelota.direction = pelota.direction * -1;
                } else {
                    a.x = a.x + pelota.direction;
                };
            }
        }
    }
}

pub fn controlar(mundo: &mut Storage, caracter: Option<Input>) {
    for id in mundo.ids_collected() {
        if let Some(b) = mundo.controlable.get_opt_mut(id) {
            if let Some(a) = mundo.posicion.get_opt_mut(id) {
                match caracter {
                    Some(Input::KeyUp) => a.y = a.y - 1,
                    Some(Input::KeyDown) => a.y = a.y + 1,
                    Some(Input::KeyRight) => a.x = a.x + 1,
                    Some(Input::KeyLeft) => a.x = a.x - 1,
                    //    Some(Input::KeyF12) => return false,
                    //    Some(Input::KeyEnter) => return false,
                    _ => (), // None => (),
                }
            }
        }
    }
}

zcomponents_storage!(Storage<Id>: {
    rebota : Rebota,
    posicion : Posicion,
    colision : Colision,
    controlable : Controlable,
    visible : sprite,
}
);
