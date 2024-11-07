use crate::my_rng::Rng;

#[derive(Clone, Copy, Debug)]
pub enum State {
    Empty,
    Filled { r: u8, g: u8, b: u8 },
}

impl State {
    pub fn random_filled(rng: &mut Rng) -> Self {
        Self::Filled {
            r: 50 + rng.next_u32_n(150) as u8,
            g: 50 + rng.next_u32_n(150) as u8,
            b: 50 + rng.next_u32_n(150) as u8,
        }
    }

    fn rand_step_single(rng: &mut Rng, color_step: i32, val: u8) -> u8 {
        (val as i32 + rng.next_u32_n(2 * color_step as u32 + 1) as i32 - color_step).clamp(0, 255)
            as u8
    }

    pub fn rand_step(self, rng: &mut Rng, color_step: i32) -> Self {
        match self {
            Self::Empty => panic!("state must not be empty"),
            Self::Filled { r, g, b } => Self::Filled {
                r: Self::rand_step_single(rng, color_step, r),
                g: Self::rand_step_single(rng, color_step, g),
                b: Self::rand_step_single(rng, color_step, b),
            },
        }
    }

    pub const fn to_color(self) -> raylib::color::Color {
        match self {
            Self::Empty => raylib::color::Color::new(0, 0, 0, 255),
            Self::Filled { r, g, b } => raylib::color::Color::new(r, g, b, 255),
        }
    }
}
