use std::{thread, time};

fn main() {
    let mut world: [ [char; 50]; 50 ] = [ [' '; 50]; 50 ];
    for x in 0..50 {
        for y in 0..50 {
            let rand = rand::random_range(0..10);
            if rand == 0 {
                world[x][y] = 'O';
            }
            print!("{} ", world[x][y]);
        }
        println!();
    }
    let delay = time::Duration::from_secs(1);
    loop {
        thread::sleep(delay);

    }
}
