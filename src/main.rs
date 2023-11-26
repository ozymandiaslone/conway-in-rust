use terminal_size::{Width, Height, terminal_size}; 
use rand::Rng;
use std::thread;
use std::time::Duration;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io::stdout;

fn main() {
    let width: usize;
    let height: usize;
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        width = w as usize;
        height = h as usize;
        println!("INFO: Your terminal is {} cols wide and {} lines tall.", width, height);

    } else {
        println!("ERROR: unable to get terminal size.");
        std::process::exit(1);
    }

    let mut grid: Vec<Vec<u8>> = vec![vec![0; width-1]; height-1];
    initialize_grid(&mut grid);
    let mut stdout = stdout();
    
    let grid_area: u32 = width as u32 * height as u32;

    loop {

        update_grid(&mut grid);
        if check_full(&grid, grid_area) {
            initialize_grid(&mut grid);
        }
        let display_string = create_string(&grid);
        execute!(stdout, MoveTo(0, 0)).unwrap();
        print!("{}", display_string);
        // 1/60 :: 16667
        // 1/30 :: 33334
        // 1/15 :: 66667
        thread::sleep(Duration::from_micros(66667));

    }
}


fn check_full (grid: &Vec<Vec<u8>>, grid_area: u32) -> bool {
    let mut population: u32 = 0;
    for row in grid {
        for cell in row {
           if *cell == 1 {
                population += 1;
            } 
        }
    }
    if population as f32 > grid_area as f32 / 3 as f32 {
        return true;
    }
    return false;
}

fn create_string(grid: &Vec<Vec<u8>>) -> String {
    let mut display_string = String::new();
    for row in grid {
        for cell in row {
            if *cell == 1 {
                display_string.push_str("+");
            } else {
                display_string.push_str(" ");
            }
        }
        display_string.push_str("\n");
    }
    display_string
}

fn update_grid(grid: &mut Vec<Vec<u8>>) {

    let width = grid[0].len();
    let height = grid.len();

    for row in 0..height {
        for column in 0..width {
            let mut living_neighbors: u8 = 0;
            // check the 8 points around the current point
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let new_row = row as i32 + dy;
                    let new_column = column as i32 + dx;
                    // dont check current point
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    //skip points that dont exist on the grid
                    if new_row < 0 || new_column < 0 || new_row >= height as i32 || new_column >= width as i32 {
                        continue;
                    }
                    // check value of point
                    let value = grid[new_row as usize][new_column as usize];
                    if value == 1 {
                        living_neighbors = living_neighbors + 1;
                    } 
                }
            }
            
            if living_neighbors < 2 {
                // dies of lonliness 
                grid[row][column] = 0;
            } else if living_neighbors > 3 {
                // dies from overpopulation
                grid[row][column] = 0;
            }

            if living_neighbors == 3 && grid[row][column] == 0 {
                // born in the right conditions
                grid[row][column] = 1;
            }
        }
    }

}

fn initialize_grid(grid: &mut Vec<Vec<u8>>) {

    let mut rng = rand::thread_rng();
    for row in grid.iter_mut() {
        for cell in row {
            let first_pass = rng.gen_range(0..=1);
            if first_pass == 1 {
                let second_pass = rng.gen_range(0..=1);
                if second_pass == 1 {
                    *cell = rng.gen_range(0..=1);
                } else { *cell = 0 }
            } else { *cell = 0 }
        }
    }

}

// DEAD CODE
fn _display_grid(grid: &Vec<Vec<u8>>) {

    for row in grid {
        for cell in row {
            if *cell == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

}
