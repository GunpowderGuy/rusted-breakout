use pancurses::Input;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Id(i32);

#[derive(Clone, Debug)]
pub struct Posicion {
    pub x: i32,
    pub y: i32,
}

type Sprite = &'static str;

#[derive(Clone, Debug)]
pub struct Colision {
    pub range: usize,
}

#[derive(Clone, Debug)]
pub struct Controlable {}

#[derive(Clone, Debug)]
pub struct Paleta {
    pub rango: i32,
}

#[derive(Clone, Debug)]
pub struct Rebota {
    pub vectorx: i32,
    pub vectory: i32,
}

zcomponents_storage!(Storage<Id>: {
    rebota : Rebota,
    posicion : Posicion,
    colision : Colision,
    controlable : Controlable,
    visible : Sprite,
    paleta: Paleta,
}
);

pub fn rebotar(mundo: &mut Storage, limitex: i32, limitey: i32) {
    for id in mundo.ids_collected() {
        if let Some(pelota) = mundo.rebota.get_opt_mut(id) {
            if let Some(a) = mundo.posicion.get_opt_mut(id) {
                if (a.x + pelota.vectorx) >= limitex || (a.x + pelota.vectorx) < 0 {
                    pelota.vectorx = pelota.vectorx * -1;
                } else {
                    a.x = a.x + pelota.vectorx;
                };
                if (a.y + pelota.vectory) >= limitey || (a.y + pelota.vectory) < 0 {
                    pelota.vectory = pelota.vectory * -1;
                } else {
                    a.y = a.y + pelota.vectory;
                };
            }
        }
    }
}

pub fn controlar(mundo: &mut Storage, caracter: Option<Input>) -> bool {
    for id in mundo.ids_collected() {
        if let Some(b) = mundo.controlable.get_opt_mut(id) {
            if let Some(a) = mundo.posicion.get_opt_mut(id) {
                match caracter {
                    Some(Input::KeyUp) => a.y = a.y - 1,
                    Some(Input::KeyDown) => a.y = a.y + 1,
                    Some(Input::KeyRight) => a.x = a.x + 1,
                    Some(Input::KeyLeft) => a.x = a.x - 1,
                    Some(Input::KeyF12) => return false,
                    Some(Input::KeyEnter) => return false,
                    _ => (), // None => (),
                }
            }
        }
    }
    return true;
}

pub fn rebotar_de_objeto(mundo: &mut Storage) {
    let mut mundo2 = mundo.clone();
    for id in mundo.ids_collected() {
        if let Some(pelota) = mundo.rebota.get_opt_mut(id) {
            if let Some(posicionbola) = mundo.posicion.get_opt_mut(id) {
                for id in mundo2.ids_collected() {
                    if let Some(Paleta) = mundo2.paleta.get_opt_mut(id) {
                        if let Some(Posicionp) = mundo2.posicion.get_opt_mut(id) {
                            let absoluto = posicionbola.x + pelota.vectorx;
                            if posicionbola.y + pelota.vectory == posicionbola.y
                                && absoluto.abs() <= Paleta.rango
                            {
                                pelota.vectory = pelota.vectory * -1;
                            }
                        }
                    }
                }
            }
        }
    }
}
