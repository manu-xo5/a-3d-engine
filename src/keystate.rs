use sdl2::keyboard::Keycode;

pub struct KeyState {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
    pub h: bool,
    pub j: bool,
    pub k: bool,
    pub l: bool,
}

pub enum KeyDirection {
    KeyUp,
    KeyDown,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            w: false,
            s: false,
            a: false,
            d: false,

            h: false,
            j: false,
            k: false,
            l: false,
        }
    }

    pub fn handle_event(&mut self, keycode: Option<Keycode>, direction: KeyDirection) {
        let value = match direction {
            KeyDirection::KeyUp => {
                println!("{:?} released", keycode,);
                false
            }
            KeyDirection::KeyDown => {
                println!("{:?} pressed", keycode,);
                true
            }
        };

        match keycode {
            Some(Keycode::W) => self.w = value,
            Some(Keycode::S) => self.s = value,
            Some(Keycode::A) => self.a = value,
            Some(Keycode::D) => self.d = value,

            Some(Keycode::H) => self.h = value,
            Some(Keycode::L) => self.l = value,

            Some(Keycode::K) => self.k = value,
            Some(Keycode::J) => self.j = value,
            _ => {}
        }
    }
}
