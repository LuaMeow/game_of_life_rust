use std::{thread, time};
use std::io::{self, Write};
const X_SIZE: usize = 200;
const Y_SIZE: usize = 50;
const GEN_TIME_MS: u64 = 250;
const MAX_LIFETIME: u8 = 20;

fn main() {
    let mut world = create_world();
    let delay = time::Duration::from_millis(GEN_TIME_MS);

    loop {
        thread::sleep(delay);
        let mut new_world = world;
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                new_world[y][x] = update_cell(world, y, x);
            }
        }
        world = new_world;

        print_world(world);
    }
}

fn create_world() -> [ [u8; X_SIZE]; Y_SIZE ] {
    let mut world: [ [u8; X_SIZE]; Y_SIZE ] = [ [0; X_SIZE]; Y_SIZE ];
    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            let rand = rand::random_range(0..2);
            if rand == 0 {
                world[y][x] = 1; //alive, age 1
            }
        }
    }
    return world;
}

fn print_world(world: [ [u8; X_SIZE]; Y_SIZE ]) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();

    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            print!("{}", if world[y][x] > 0 { 'O' } else { ' ' });
        }
        println!();
    }
    io::stdout().flush().unwrap();
}

fn check_cell(world: [ [u8; X_SIZE]; Y_SIZE ], y: usize, x:usize) -> bool {
    return world[y][x] > 0;
}

fn update_cell(world: [ [u8; X_SIZE]; Y_SIZE ], y: usize, x: usize) -> u8 {
    if (x+1) == X_SIZE || x == 0 {
        return 0;
    }

    if (y+1) == Y_SIZE || y == 0 {
        return 0;
    }

    let neighbor_count: i8 =
        check_cell(world, y, x+1) as i8 +
        check_cell(world, y, x-1) as i8 +
        check_cell(world, y+1, x) as i8 +
        check_cell(world, y-1, x) as i8 +
        check_cell(world, y+1, x+1) as i8 +
        check_cell(world, y-1, x-1) as i8 +
        check_cell(world, y+1, x-1) as i8 +
        check_cell(world, y-1, x+1) as i8;

    let age = world[y][x];

    if age > 0 {
        if age >=MAX_LIFETIME {
            return 0; //death from old age
        }

        if neighbor_count == 2 || neighbor_count == 3 {
            return age + 1; //survives and ages
        }
        return 0; //death from under/overpopulation
    } else {
        if neighbor_count == 3 {
            return 1; //born
        }
    }

    return 0;
}
