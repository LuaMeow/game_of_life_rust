use std::{thread, time};
const x_size: usize = 50;
const y_size: usize = 50;

fn main() {
    let mut world = create_world();
    let delay = time::Duration::from_millis(500);

    loop {
        thread::sleep(delay);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut new_world = world;
        for y in 0..y_size {
            for x in 0..x_size {
                new_world[y][x] = update_cell(world, y, x);
            }
        }
        world = new_world;

        print_world(world);
    }
}

fn create_world() -> [ [char; x_size]; y_size ] {
    let mut world: [ [char; x_size]; y_size ] = [ [' '; x_size]; y_size ];
    for y in 0..y_size {
        for x in 0..x_size {
            let rand = rand::random_range(0..20);
            if rand == 0 {
                world[y][x] = 'O';
            }
        }
    }
    return world;
}

fn print_world(world: [ [char; x_size]; y_size ]) {
    for y in 0..y_size {
        for x in 0..x_size {
            print!("{} ", world[y][x]);
        }
        println!();
    }
}

fn update_cell(world: [ [char; x_size]; y_size ], y: usize, x: usize) -> char {
    if (x+1) == x_size || x == 0 {
        return ' ';
    }

    if world[y][x-1] == 'O' {
        return 'O';
    }

    return ' ';
}
