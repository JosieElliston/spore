#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::unreadable_literal)]

mod bijective_finite_sequence;
mod dish;
mod my_rng;
mod state;

use dish::Dish;
use my_rng::Rng;
use raylib::prelude::*;
use state::State;

const SCREEN_SIZE: usize = 700;

// TODO: something other than rgb
// TODO: state as a newtype of rgba with a=0 => empty and a=255 => colored
// TODO: click to insert a seed
// TODO: zooming + pan + switch to egui/eframe

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let mut rng = Rng::seeded();

    // bench(&mut rng);
    // panic!("bench done");

    // generate(5000, 8, 2, true);

    run_raylib(&mut rng);
}

fn run_raylib(rng: &mut Rng) {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_SIZE as i32, SCREEN_SIZE as i32)
        .title("spore")
        .build();

    let mut paused = false;
    // how much the radius (in units of pixels) of the colored region should grow per second
    let mut radius_per_second: f32 = 32.0;
    // how many pixels are colored initially
    let mut seed_count: usize = 2;
    let mut color_step: i32 = 3;
    let mut highlight_border = true;
    let mut dish = Dish::from_seed_count(rng, SCREEN_SIZE, seed_count);
    while !rl.window_should_close() {
        // assert!(rl.get_time() < 10.0);
        // assert!(!dish.is_done());

        let dt = rl.get_frame_time().min(1.0 / 30.0);
        // println!("dt: {dt}");

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
            dish = Dish::from_seed_count(rng, SCREEN_SIZE, seed_count);
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_K) {
            paused = !paused;
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_M) {
            radius_per_second = (0.5 * radius_per_second).max(2e-16);
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SLASH) {
            radius_per_second = (2.0 * radius_per_second).min(2e16);
        }
        let steps_per_second = dish.perimeter() * radius_per_second;

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_COMMA) {
            println!("todo: one step backwards");
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_PERIOD) {
            while !dish.maybe_step(rng, color_step) {}
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_J) {
            println!("todo: one second backwards");
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_L) {
            let mut step_count = 0;
            while step_count < steps_per_second as usize && !dish.is_done() {
                step_count += dish.maybe_step(rng, color_step) as usize;
            }
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_MINUS) {
            seed_count = seed_count.saturating_sub(1);
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_EQUAL) {
            seed_count += 1;
            // loop {
            //     let row = rng.next_u32_n(SCREEN_SIZE as u32) as usize;
            //     let col = rng.next_u32_n(SCREEN_SIZE as u32) as usize;
            //     if matches!(dish.as_slice()[row][col], State::Empty) {
            //         dish.insert_seed(row, col, State::random_filled(rng));
            //         break;
            //     }
            // }
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT_BRACKET) {
            color_step = (color_step - 1).max(0);
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT_BRACKET) {
            color_step += 1;
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_P) {
            dish.save_to_image(std::path::Path::new("./image.png"), highlight_border);
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_B) {
            highlight_border = !highlight_border;
        }

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_V) {
            dish.validate();
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
            && ((0..SCREEN_SIZE as i32).contains(&rl.get_mouse_x())
                && !(0..SCREEN_SIZE as i32).contains(&rl.get_mouse_y()))
        {
            dish.insert_seed(
                rl.get_mouse_y() as usize,
                rl.get_mouse_x() as usize,
                State::random_filled(rng),
            );
        }

        // step the dish
        if !paused && !dish.is_done() {
            // find steps_per_second = d/dt (area) from radius_per_second = d/dt (radius)
            // suppose the filled region is a circle (the pi's cancel in the end so its actually invariant to shape)
            // radius = perimeter / (2 * pi)
            // area = pi * radius^2
            // d/dt (area) = d/dt (pi * radius^2)
            // d/dt (area) = 2 * pi * radius * d/dt (radius)
            // d/dt (area) = perimeter * d/dt (radius)
            #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let target_step_count = ((steps_per_second * dt) as usize).max(1);
            let mut step_count = 0;
            while step_count < target_step_count && !dish.is_done() {
                step_count += dish.maybe_step(rng, color_step) as usize;
            }
        }

        let mut draw_handle: RaylibDrawHandle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);
        dish.draw(&mut draw_handle, highlight_border);
    }
}

fn bench(rng: &mut Rng) {
    for size_mul in 1..20 {
        let size = size_mul * 100;
        let mut dish = Dish::new(size);
        dish.insert_seed(size / 2, size / 2, State::random_filled(rng));

        let start = std::time::Instant::now();
        while !dish.is_done() {
            dish.maybe_step(rng, 3);
        }
        let elapsed = start.elapsed();
        // time should be linear in area
        println!("{}, {}", size * size, elapsed.as_secs_f32());
    }
}

/// saves an image with these parameters
fn generate(size: usize, seed_count: usize, color_step: i32, highlight_border: bool) {
    let start = std::time::Instant::now();
    let mut rng = Rng::seeded();
    let mut dish = Dish::from_seed_count(&mut rng, size, seed_count);
    while !dish.is_done() {
        dish.maybe_step(&mut rng, color_step);
    }
    dish.save_to_image(std::path::Path::new("./image.png"), highlight_border);
    let elapsed = start.elapsed();
    println!(
        "generated dish with size {} in time {}s",
        size,
        elapsed.as_secs_f32()
    );
}
