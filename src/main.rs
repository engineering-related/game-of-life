use std::{fmt, thread::sleep};

#[derive(Copy, Clone)]
struct Grid {
    grid: [[bool;8];8],
}

impl Grid {
    pub fn new() -> Self{
        Grid {
            grid: [[false; 8];8],
        }
    }

    pub fn is_in_grid(self, i: isize, j: isize) -> bool{
        i >= 0 && i <= self.grid.len() as isize &&
            j >= 0 && j <= self.grid.len() as isize
    }

    pub fn get_number_neighbours(self, i: isize, j: isize) -> usize {
        let mut cnt = 0;
        for y in -1 + i..1 + i {
           for x in -1 + j..1 + j {
                if x != 0 && y != 0 && self.is_in_grid(x, y) {
                    if self.grid[y as usize][x as usize] {
                        cnt += 1; 
                    }     
                }
            } 
        }
        return cnt;
    }

    pub fn update(&mut self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len(){
                let nr_neig = self.get_number_neighbours(i as isize, j as isize);
                if nr_neig < 2 {
                    self.grid[i][j] = false;
                }
                else if nr_neig <= 3 {
                    self.grid[i][j] = true; 
                }
                else {
                    self.grid[i][j] = false;
                }
            }
        }
    } 


}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for row in self.grid {
            for col in row{
                if col {
                    buffer.push('#');
                }
                else {
                    buffer.push(' ');
                }
            }
            buffer.push('\n');
        }
        write!(f, "{}", buffer)
    }
}

fn main() {
    
    let mut grid = Grid::new();
    grid.grid[4][4] = true; 
    grid.grid[4][3] = true;  
    grid.grid[4][2] = true;  
    //loop {
        //grid.update();
        println!("{}", grid);
        std::thread::sleep_ms(100);
        println!("{:?}", grid.grid);
        //print!("{}[2j", 27 as char);
   //}

}
