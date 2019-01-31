extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::env;

fn main() {
    use game_objects::*;

    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 as usize {
        (&args[1]).to_string()
    } else {
        "map.txt".to_string()
    };

    let fps: u32 = if args.len() > 2 as usize {
        let i = match args[2].parse::<u32>() {
            Ok(i) => i,
            _ => 30,
        };
        i
    } else {
        30
    };

    println!("filename: {} fps limit: {}", filename, fps);

    let map = Map::from_file(filename.to_string());

    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: Window = WindowSettings::new("Game of Life", [400, 400])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        map: map,
        fps_limit: fps,
    };

    app.game_loop(window);
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    map: crate::game_objects::Map,
    fps_limit: u32,
}

impl App {
    fn game_loop(self: &mut Self, mut window: Window) {
        use std::{thread, time};

        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut window) {
            let t = time::SystemTime::now();

            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if self.fps_limit == 0 {
                continue;
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }

            if let Ok(d) = time::SystemTime::now().duration_since(t) {
                let wait_time = time::Duration::from_secs(1) / self.fps_limit;
                let wait_time = if wait_time <= d {
                    time::Duration::from_nanos(0)
                } else {
                    wait_time - d
                };
                println!("Wait time: {:?}", wait_time);
                thread::sleep(wait_time);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        use vecmath::*;

        let map = &self.map;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let step_x = args.width / map.width as f64;
        let step_y = args.height / map.height as f64;

        let vec2: Vector2<f64> = [step_y, step_x];
        let scalar = vec2_len(vec2);

        let mut rectangles: Vec<([f64; 4], [f32; 4])> = Vec::new();

        for index_x in 0..map.width {
            for index_y in 0..map.height {
                if let Some(cell) = map.get_cell(index_x, index_y) {
                    let x = step_x * index_x as f64;
                    let y = step_y * index_y as f64;

                    let rect = rectangle::square(x, y, scalar);
                    let color = if cell.is_alive { RED } else { GREEN };

                    rectangles.push((rect, color));
                }
            }
        }

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform;

            for (rect, color) in rectangles {
                rectangle(color, rect, transform, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let map = &mut self.map;

        for index_x in 0..map.width {
            for index_y in 0..map.height {
                if let Some(ref mut cell) = map.get_cell_mut(index_x, index_y) {
                    cell.is_alive = cell.is_alive_next;
                }
            }
        }

        for index_x in 0..map.width {
            for index_y in 0..map.height {
                let count = map.count_living_neighbours(index_x as i32, index_y as i32);

                if let Some(ref mut cell) = map.get_cell_mut(index_x, index_y) {
                    cell.calculate_next_round(count);
                }
            }
        }
    }
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
                self.is_alive_next = neighours >= 2 && neighours < 4;
            } else {
                self.is_alive_next = neighours == 3;
            }
        }
    }

    impl Map {
        pub fn from_file(path: String) -> Map {
            use std::fs;
            use std::io::ErrorKind;
            let original_path = path.clone();
            let contents = fs::read_to_string(path);
            let contents = match contents {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => match fs::read_to_string("map.txt") {
                        Ok(file) => {
                            println!(
                                "could not find {}. Loaded {} instead",
                                original_path, "map.txt"
                            );
                            file
                        }
                        Err(error) => panic!("Could not read map file! {}", error),
                    },
                    other_error => panic!("Could not read map file! {:?}", other_error),
                },
            };
            let result: Vec<_> = contents.lines().collect();
            let width = result[0].parse::<usize>().unwrap();
            let height = result[1].parse::<usize>().unwrap();

            println!("width: {}, height: {}", width, height);

            let mut fields: Vec<Vec<bool>> = vec![vec![false; height]; width];

            let mut field_index_x = 0;
            for index in 2..result.len() {
                let mut row = result[index].to_string();
                row.retain(|c| c != ' '); // strip out blanks used for formatting
                for (i, item) in row.chars().enumerate() {
                    fields[field_index_x][i] = item == '1';
                }
                field_index_x += 1;
            }
            Map::from(width, height, fields)
        }

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

        pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
            if x < self.width && y < self.height {
                return Some(&mut self.fields[x][y]);
            }
            None
        }

        pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
            if x < self.width && y < self.height {
                return Some(&self.fields[x][y]);
            }
            None
        }

        pub fn count_living_neighbours(&self, x: i32, y: i32) -> i32 {
            let mut counter = 0;
            let width = self.width as i32;
            let height = self.height as i32;

            for counter_x in x - 1..x + 2 {
                let index_x = Map::get_map_bounds(counter_x, width);

                for counter_y in y - 1..y + 2 {
                    if counter_x == x && counter_y == y {
                        continue;
                    }

                    let index_y = Map::get_map_bounds(counter_y, height);

                    let cell = &self.fields[index_x][index_y];
                    if cell.is_alive {
                        counter += 1;
                    }
                }
            }
            counter
        }

        fn get_map_bounds(counter: i32, dim: i32) -> usize {
            if counter < 0 {
                (counter + dim) as usize
            } else if counter >= dim {
                (counter - dim) as usize
            } else {
                counter as usize
            }
        }
    }
}
