use std::{thread, time};
use std::io::{self, Write};
const X_SIZE: usize = 100;
const Y_SIZE: usize = 50;
const GEN_TIME_MS: u64 = 250;

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

fn create_world() -> [ [char; X_SIZE]; Y_SIZE ] {
    let mut world: [ [char; X_SIZE]; Y_SIZE ] = [ [' '; X_SIZE]; Y_SIZE ];
    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            let rand = rand::random_range(0..2);
            if rand == 0 {
                world[y][x] = 'O';
            }
        }
    }
    return world;
}

fn print_world(world: [ [char; X_SIZE]; Y_SIZE ]) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();

    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            print!("{}", world[y][x]);
        }
        println!();
    }
    io::stdout().flush().unwrap();
}

fn check_cell(world: [ [char; X_SIZE]; Y_SIZE ], y: usize, x:usize) -> bool {
    return world[y][x] == 'O';
}

fn update_cell(world: [ [char; X_SIZE]; Y_SIZE ], y: usize, x: usize) -> char {
    if (x+1) == X_SIZE || x == 0 {
        return ' ';
    }

    if (y+1) == Y_SIZE || y == 0 {
        return ' ';
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

    if (neighbor_count == 2 || neighbor_count == 3) && check_cell(world, y, x) {
        return 'O';
    } else if neighbor_count == 3 && !check_cell(world, y, x) {
        return 'O';
    }

    return ' ';
}
