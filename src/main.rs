extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};

const WORLD_SIZE: usize = 50;
const BLOCK_SIZE: f64 = 16.0;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0; 4];

type World = [[bool; WORLD_SIZE]; WORLD_SIZE];

fn draw_cell(x: usize, y: usize, context: Context, graphics: &mut G2d<'_>) {
    rectangle(
        BLACK,
        [
            x as f64 * BLOCK_SIZE,
            y as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        context.transform,
        graphics,
    );
}

fn count_neighbors(world: World, x: usize, y: usize) -> usize {
    let mut ct = 0;
    for y_ in (max(1, y) - 1)..min(y + 2, WORLD_SIZE) {
        for x_ in (max(1, x) - 1)..min(x + 2, WORLD_SIZE) {
            if (x_ != x || y_ != y) && world[x_][y_] {
                ct = ct + 1;
            }
        }
    }
    ct
}

fn step(w1: World, w2: &mut World) {
    for y in 0..WORLD_SIZE {
        for x in 0..WORLD_SIZE {
            let n_neighbors = count_neighbors(w1, x, y);

            match (w1[x][y], n_neighbors) {
                (false, 3) => w2[x][y] = true,
                (false, _) => (),
                (true, 0) | (true, 1) => w2[x][y] = false,
                (true, 2) | (true, 3) => (),
                (true, _) => w2[x][y] = false,
            }
        }
    }
}

fn main() {
    let mut world: World = [[false; WORLD_SIZE]; WORLD_SIZE];

    for y in 0..WORLD_SIZE {
        for x in 0..WORLD_SIZE {
            world[x][y] = thread_rng().gen_range(0, 2) == 0;
        }
    }

    let edge_size = (WORLD_SIZE as f64 * BLOCK_SIZE) as u32;

    let mut window: PistonWindow = WindowSettings::new(
        "Hello Piston!",
        [edge_size; 2],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let max_fps = window.get_event_settings().max_fps;
    let evt_rate = 0.5;

    let mut frame_ctr = 0;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            frame_ctr += 1;

            if frame_ctr as f64 >= max_fps as f64 * evt_rate {
                let mut w2 = world;
                step(world, &mut w2);
                world = w2;
                frame_ctr = 0;
            }

            clear(WHITE, graphics);
            for y in 0..WORLD_SIZE {
                for x in 0..WORLD_SIZE {
                    let c = world[x][y];
                    if c {
                        draw_cell(x, y, context, graphics);
                    }
                }
            }
        });
    }
}
