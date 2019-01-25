fn main() {
    use std::fs;
    let filename = "/home/nils/Desktop/map4.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let result: Vec<_> = contents.lines().collect();
    let width = result[0].parse::<usize>().unwrap();
    let height = result[1].parse::<usize>().unwrap();
    println!("width: {}, height: {}", width, height);
    let mut fields: Vec<Vec<bool>> = vec![vec![false; height]; width];
    for index in 2..result.len() - 3 {
        let mut row = result[index].to_string();
        row.retain(|c| c != ' ');
        for (i, item) in row.chars().enumerate() {
            fields[i][index] = item == '1';
        }
    }
    let map = game_objects::Map::from(width, height, fields);
    game_loop(map);
}

fn game_loop(mut map: game_objects::Map) {
    loop {
        for index_x in 0..map.width {
            for index_y in 0..map.height {
                let mut cell_opt = map.get_cell(index_x, index_y);

                if let Some(ref mut cell) = cell_opt {
                    cell.is_alive = cell.is_alive_next;
                }
            }
        }
        for index_x in 0..map.width {
            for index_y in 0..map.height {
                let count = map.count_living_neighbours(index_x, index_y);
                let mut cell_opt = map.get_cell(index_x, index_y);

                if let Some(ref mut cell) = cell_opt {
                    cell.calculate_next_round(count);
                }
            }
        }

        use std::{thread, time};

        let wait_time = time::Duration::from_secs(1);
        thread::sleep(wait_time);
        render(&mut map);
    }
}

fn render(map: &mut game_objects::Map) {
    // use std::io;
    use std::ops::Add;
    let mut buffer = String::new();
    for index_x in 0..map.width {
        let mut temp = String::new();
        for index_y in 0..map.height {
            let cell_opt = map.get_cell(index_x, index_y);
            if let Some(cell) = cell_opt {
                if cell.is_alive {
                    temp = temp.add("#");
                } else {
                    temp = temp.add(" ");
                }
            } else {
                temp = temp.add(" ");
            }
        }
        buffer = buffer.add(&temp);
        buffer = buffer.add("\n");
    }
    std::process::Command::new("clear");
    print!("{}", buffer);
}

mod game_objects {
    pub struct Map {
        fields: Vec<Vec<Cell>>,
        pub width: usize,
        pub height: usize,
    }

    pub struct Cell {
        pub is_alive: bool,
        pub is_alive_next: bool,
    }
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Clone for Cell {
        fn clone(&self) -> Cell {
            Cell {
                is_alive: self.is_alive,
                is_alive_next: self.is_alive_next,
            }
        }
    }
    impl Cell {
        pub fn calculate_next_round(&mut self, neighours: i32) {
            if self.is_alive {
                if neighours > 2 && neighours < 4 {
                    self.is_alive_next = true;
                } else {
                    self.is_alive_next = false;
                }
            } else if neighours == 3 {
                self.is_alive_next = true;
            } else {
                self.is_alive_next = false;
            }
        }
    }

    impl Point {
        pub fn new(x_pos: i32, y_pos: i32) -> Point {
            Point { x: x_pos, y: y_pos }
        }
    }

    impl Map {
        pub fn from(width: usize, height: usize, field_values: Vec<Vec<bool>>) -> Map {
            let mut fields: Vec<Vec<Cell>> = vec![
                vec![
                    Cell {
                        is_alive: false,
                        is_alive_next: false
                    };
                    height
                ];
                width
            ];
            for x in 0..width {
                for y in 0..height {
                    let is_alive = field_values[x][y];
                    fields[x][y].is_alive = is_alive;
                    fields[x][y].is_alive_next = is_alive;
                }
            }

            Map {
                width,
                height,
                fields,
            }
        }

        pub fn get_cell(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
            if x < self.width && y < self.height {
                return Some(&mut self.fields[x][y]);
            }
            None
        }

        pub fn count_living_neighbours(&self, x: usize, y: usize) -> i32 {
            let mut counter = 0;
            let low_x = if x == 0 { 0 } else { x - 1 };
            let low_y = if y == 0 { 0 } else { y - 1 };

            for index_x in low_x..x + 2 {
                if index_x >= self.width {
                    return counter;
                }
                for index_y in low_y..y + 2 {
                    if index_y >= self.height {
                        break;
                    }
                    let cell = &self.fields[index_x as usize][index_y as usize];
                    if cell.is_alive {
                        counter += 1;
                    }
                }
            }
            counter
        }
    }
}
