use macroquad::prelude::*;


struct GameState {
    cells: std::collections::HashSet<(i32, i32)>,
}

impl GameState {
    fn new() -> GameState {
        Self {
            cells: std::collections::HashSet::new(),
        }
    }

    fn update(&mut self, window: &Window) {
        // std::mem::swap(&mut self.cells, &mut self.cells_buffer);
        let mut cells = std::collections::HashSet::new();

        let mut must_visit = std::collections::HashSet::new();
        // cells is now the state and cells_buffer is now the cells we are updating
        for cell in self.cells.iter() {
            let mut neighbours = 0;
            must_visit.remove(&(cell.0, cell.1));
            for x in -1..=1 {
                for y in -1..=1 {
                    must_visit.insert((cell.0 + x, cell.1 + y));
                    if x == 0 && y == 0 {
                        continue;
                    }
                    if self.cells.contains(&(cell.0 + x, cell.1 + y)) {
                        neighbours += 1;
                    }
                }
            }
            // 3 for game of life or 4 for weird patterns
            if neighbours == 3 || (neighbours == 2 && self.cells.contains(cell)) {
                cells.insert(*cell);
            } else {
                cells.remove(cell);
            }
        }
        for cell in must_visit.iter() {
            let mut neighbours = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    if self.cells.contains(&(cell.0 + x, cell.1 + y)) {
                        neighbours += 1;
                    }
                }
            }
            if neighbours == 3 {
                cells.insert((cell.0, cell.1));
            }
        }
        self.cells = cells;
    }
}

struct Window {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Window {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Window {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn print(&self, g: &GameState) {
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                if g.cells.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!("------------------------------------------------------------------");
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let CELL_SIZE = 20.0;

    let iterations = 100;
    let mut game_state = GameState::new();
    let mut window = Window::new(-5, -5, 40, 30);
    game_state.cells.insert((0, 0));
    game_state.cells.insert((1, 0));
    game_state.cells.insert((2, 0));
    // insert 100 random cells
    // seed the random number generator with the current time
    rand::srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64);
    for _ in 0..iterations {
        game_state.cells.insert((
            rand::rand() as i32 % window.width + window.x,
            rand::rand() as i32 % window.height + window.y,
        ));
    }
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::Space) {
            game_state.update(&window);
            // std::thread::sleep(std::time::Duration::from_millis(40));
            // window.print(&game_state);
        }
        if is_key_down(KeyCode::Up) {
            window.y -= 1;
        }
        if is_key_down(KeyCode::Down) {
            window.y += 1;
        }
        if is_key_down(KeyCode::Left) {
            window.x -= 1;
        }
        if is_key_down(KeyCode::Right) {
            window.x += 1;
        }
        if is_key_down(KeyCode::Backspace) {
            game_state.cells.clear();
        }
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            game_state.cells.insert((
                (mouse_pos.0 / CELL_SIZE) as i32 + window.x,
                (mouse_pos.1 / CELL_SIZE) as i32 + window.y,
            ));
        }
        for x in window.x..window.x + window.width {
            draw_line(
                (x as f32 - window.x as f32) * CELL_SIZE,
                0.0,
                (x as f32 - window.x as f32) * CELL_SIZE,
                window.height as f32 * CELL_SIZE,
                1.0,
                WHITE,
            );
        }
        for y in window.y..window.y + window.height {
            draw_line(
                0.0,
                (y as f32 - window.y as f32) * CELL_SIZE,
                window.width as f32 * CELL_SIZE,
                (y as f32 - window.y as f32) * CELL_SIZE,
                1.0,
                WHITE,
            );
        }
        for cell in game_state.cells.iter() {
            draw_rectangle(
                ((cell.0 as f32 - window.x as f32) * CELL_SIZE),
                ((cell.1 as f32 - window.y as f32) * CELL_SIZE),
                CELL_SIZE,
                CELL_SIZE,
                WHITE,
            );
        }
        next_frame().await
    }
}
