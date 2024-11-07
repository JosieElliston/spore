use raylib::prelude::*;

use crate::bijective_finite_sequence::BijectiveFiniteSequence;
use crate::my_rng::Rng;
use crate::state::State;

// TODO: maybe if border stored empty pixels it can be faster?

#[derive(Clone, Debug)]
pub struct Dish {
    states: Vec<Vec<State>>, // TODO: this should be a 2d vec, not a Vec<Vec<_>>
    /// holds states that are empty and adjacent to states that are filled
    border: BijectiveFiniteSequence<(usize, usize)>,
}

impl Dish {
    pub fn new(size: usize) -> Self {
        Self {
            states: vec![vec![State::Empty; size]; size],
            border: BijectiveFiniteSequence::new(),
        }
    }

    pub fn from_seed_count(rng: &mut Rng, size: usize, seed_count: usize) -> Self {
        let mut slf = Self::new(size);
        for _ in 0..seed_count {
            slf.insert_seed(
                rng.next_u32_n(size as u32) as usize,
                rng.next_u32_n(size as u32) as usize,
                State::random_filled(rng),
            );
        }
        slf
    }

    pub fn is_done(&self) -> bool {
        self.border.is_empty()
    }

    /// approximate perimeter of the colored region
    pub fn perimeter(&self) -> f32 {
        self.border.len() as f32
    }

    // pub fn as_slice(&self) -> &[Vec<State>] {
    //     &self.states
    // }

    pub fn insert_seed(&mut self, row: usize, col: usize, state: State) {
        self.states[row][col] = state;

        if self.on_border(row, col) {
            self.border.insert((row, col));
        }

        for (row_1, col_1) in [
            (row as i32, col as i32 + 1),
            (row as i32 + 1, col as i32),
            (row as i32, col as i32 - 1),
            (row as i32 - 1, col as i32),
        ] {
            if (0..self.states.len() as i32).contains(&row_1)
                && (0..self.states.len() as i32).contains(&col_1)
            {
                let row_1 = row_1 as usize;
                let col_1 = col_1 as usize;
                if self.on_border(row_1, col_1) {
                    self.border.insert((row_1, col_1));
                } else {
                    self.border.remove(&(row_1, col_1));
                }
            }
        }
    }

    /// returns whether the state is filled and has a neighbor that's empty
    fn on_border(&self, row: usize, col: usize) -> bool {
        let state = self.states[row][col];
        match state {
            State::Empty => false,
            State::Filled { .. } => [
                (row as i32, col as i32 + 1),
                (row as i32 + 1, col as i32),
                (row as i32, col as i32 - 1),
                (row as i32 - 1, col as i32),
            ]
            .into_iter()
            .any(|(row_1, col_1)| {
                usize::try_from(row_1).map_or(false, |row_1| {
                    usize::try_from(col_1).map_or(false, |col_1| {
                        self.states.get(row_1).map_or(false, |line_1| {
                            line_1
                                .get(col_1)
                                .map_or(false, |state_1| matches!(state_1, State::Empty))
                        })
                    })
                })
                // if (0..self.states.len() as i32).contains(&row_1)
                //     && (0..self.states.len() as i32).contains(&col_1)
                // {
                //     matches!(self.states[row_1 as usize][col_1 as usize], State::Empty)
                // } else {
                //     false
                // }
            }),
        }
    }

    /// returns whether a step was taken (a step fails with probability > 0.5)
    pub fn maybe_step(&mut self, rng: &mut Rng, color_step: i32) -> bool {
        const DEBUG_PRINT: bool = false;
        // assert!(!self.is_done());
        if let Some(&(row, col)) = self.border.get_random(rng) {
            let state = self.states[row][col];
            match state {
                State::Empty => panic!("empty states should not be on the border"),
                State::Filled { .. } => {
                    let (row_1, col_1) = match rng.next(2) {
                        0 => (row as i32, col as i32 + 1),
                        1 => (row as i32 + 1, col as i32),
                        2 => (row as i32, col as i32 - 1),
                        3 => (row as i32 - 1, col as i32),
                        _ => unreachable!("rng invariance violated"),
                    };
                    if let Ok(row_1) = usize::try_from(row_1) {
                        if let Ok(col_1) = usize::try_from(col_1) {
                            if let Some(line_1) = self.states.get(row_1) {
                                if let Some(state_1) = line_1.get(col_1) {
                                    if matches!(state_1, State::Empty) {
                                        self.states[row_1][col_1] =
                                            state.rand_step(rng, color_step);
                                        let mut any_empty = false;
                                        for (row_2, col_2) in [
                                            (row_1 as i32, col_1 as i32 + 1),
                                            (row_1 as i32 + 1, col_1 as i32),
                                            (row_1 as i32, col_1 as i32 - 1),
                                            (row_1 as i32 - 1, col_1 as i32),
                                        ] {
                                            if (0..self.states.len() as i32).contains(&row_2)
                                                && (0..self.states.len() as i32).contains(&col_2)
                                            {
                                                any_empty |= matches!(
                                                    self.states[row_2 as usize][col_2 as usize],
                                                    State::Empty
                                                );
                                                if !self.on_border(row_2 as usize, col_2 as usize) {
                                                    self.border
                                                        .remove(&(row_2 as usize, col_2 as usize));
                                                }
                                            }
                                        }
                                        if any_empty {
                                            self.border.insert((row_1, col_1));
                                        }
                                        if DEBUG_PRINT {
                                            println!("updated");
                                        }
                                        return true;
                                    }
                                    if DEBUG_PRINT {
                                        println!("not matches!(state_1, State::Empty)");
                                    }
                                } else if DEBUG_PRINT {
                                    println!("not let Some(state_1) = line_1.get(col_1)");
                                }
                            } else if DEBUG_PRINT {
                                println!("not let Some(line_1) = self.states.get(row_1)");
                            }
                        } else if DEBUG_PRINT {
                            println!("not let Ok(col_1) = usize::try_from(col_1)");
                        }
                    } else if DEBUG_PRINT {
                        println!("not let Ok(row_1) = usize::try_from(row_1)");
                    }
                }
            }
        } else if DEBUG_PRINT {
            println!("not let Some(&(row, col)) = self.border.get_random(rng)");
        }
        false
    }

    pub fn save_to_image(&self, path: &std::path::Path, highlight_border: bool) {
        let mut image = image::ImageBuffer::new(self.states.len() as u32, self.states.len() as u32);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let color = self.states[y as usize][x as usize].to_color();
            *pixel = image::Rgb([color.r, color.g, color.b]);
        }
        if highlight_border {
            for (row, col) in self.border.as_slice().iter().copied() {
                image.put_pixel(col as u32, row as u32, image::Rgb::from([255, 255, 255]));
            }
        }
        image.save(path).unwrap();
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle, highlight_border: bool) {
        // TODO: parallel write to texture?
        for (row, line) in self.states.iter().enumerate() {
            for (col, state) in line.iter().enumerate() {
                if matches!(state, State::Filled { .. }) {
                    draw_handle.draw_pixel(col as i32, row as i32, state.to_color());
                }
            }
        }
        if highlight_border {
            for (row, col) in self.border.as_slice().iter().copied() {
                draw_handle.draw_pixel(col as i32, row as i32, raylib::color::Color::WHITE);
            }
        }
    }

    pub fn validate(&self) {
        for row in 0..self.states.len() {
            for col in 0..self.states.len() {
                assert_eq!(self.border.contains(&(row, col)), self.on_border(row, col));
            }
        }
        self.border.validate();
    }
}
