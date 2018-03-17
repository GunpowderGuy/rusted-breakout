use pancurses::Input;

#[derive(new, Copy, Clone)]
pub struct Bloque {
    pub x: i64,
    pub y: i64,
}

impl Bloque {
    pub fn mover(&mut self, caracter: Option<Input>) -> bool {
        match caracter {
            Some(Input::KeyUp) => self.y = self.y - 1,
            Some(Input::KeyDown) => self.y = self.y + 1,
            Some(Input::KeyRight) => self.x = self.x + 1,
            Some(Input::KeyLeft) => self.x = self.x - 1,
            Some(Input::KeyF1) => return false,
            Some(Input::KeyEnter) => return false,
            _ => (), // None => (),
        }
        return true;
    }
}

#[derive(new, Copy, Clone)]
pub struct Pelota {
    pub x: i64,
    pub y: i64,
    next_x: i64,
    next_y: i64,
    direction_x: i64,
    direction_y: i64,
}

impl Pelota {
    pub fn mover(&mut self, max_x: i64, max_y: i64) {
        mover_pelota(self, max_x, max_y)
    }
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
