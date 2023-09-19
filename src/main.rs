mod ui;
mod widgets;
use druid_use::*;
use consts::*;

pub mod druid_use {
    pub use druid::kurbo::Circle;
    pub use druid::widget::{Label, Flex, Painter, Split, Container, LabelText};
    pub use druid::{AppLauncher, Widget, WindowDesc, Data, Lens, WidgetExt, RenderContext, Color, EventCtx, Env};
    pub use druid::im::Vector;
    pub use druid::piet::kurbo::Line;
}

pub mod consts {
    use crate::druid_use::*;
    pub const POINT_SIZE: f64 = 50.0;
    pub const NUM_POINTS: usize = 9;
    pub const WINDOW_SIZE: (f64, f64) = (POINT_SIZE * NUM_POINTS as f64 + 200.0,
        POINT_SIZE * NUM_POINTS as f64);

    pub const CONTROLS_COLOR: Color = Color::GRAY;
    pub const BOARD_COLOR: Color = Color::rgb8(252, 208, 96);
    pub const NUM_COLOR: Color = Color::rgb8(180, 0, 0);

    pub const LINE_WEIGHT: f64 = 5.0;
    pub const LINE_COLOR: Color = Color::BLACK;

    pub const STONE_SIZE: f64 = 0.85; // Percentage of maximum size
    pub const STONE_WEIGHT: f64 = 2.0; // Outline weight
}

// TODO: start on rules

#[derive(Clone, Data, Lens)]
pub struct GameState {
    board: Board,
    curr_player: Player,
    curr_num: u32, // number of most recently placed stone
    locked: bool,
    disp_nums: bool,
}

impl std::ops::Index<usize> for GameState {
    type Output = Vector<BoardPoint>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.board.board[index]
    }
}

impl std::ops::IndexMut<usize> for GameState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board.board[index]
    }
}

#[derive(Clone, Data, Lens)]
struct Board {
    board: Vector<Vector<BoardPoint>>,
}

#[derive(Clone, Data)]
pub struct BoardPoint {
    owner: Option<Player>,
    number: Option<u32>,
}

#[derive(Clone, Data, PartialEq, Copy)]
enum Player {
    Black, White, BlackTemp, WhiteTemp
}

impl Player {
    // Gives the temporary version of a player
    const fn temp(player: Player) -> Player {
        match player {
            Player::Black => Player::BlackTemp,
            Player::White => Player::WhiteTemp,
            _ => player
        }
    }
}

impl GameState {
    fn toggle_stone(&mut self, i: usize, j: usize) -> () {
        if self[i][j].owner == Some(self.curr_player) {
            self.remove_stone(i, j);
        } else {
            self.place_stone(i, j);
        }
    }

    // Removes the stone at the coordinates i, j
    fn remove_stone(&mut self, i: usize, j: usize) -> () {
        self[i][j].owner = None;
        // decrement curr num if this is the most recently placed stone
        if self[i][j].number == Some(self.curr_num) {
            self.curr_num -= 1;
            self[i][j].number = None;
        }
    }

    // Places a stone at the coordinates i, j
    fn place_stone(&mut self, i: usize, j: usize) -> () {
        if self.locked {
            self[i][j].owner = Some(Player::temp(self.curr_player));
            self.curr_num += 1;
            self[i][j].number = Some(self.curr_num);
        } else {
            self[i][j].owner = Some(self.curr_player);
        }
    }

    // Completely resets the entire program
    fn reset(&mut self) -> () {
        self.curr_player = Player::Black;
        self.curr_num = 0;
        for row in self.board.board.iter_mut() {
            for point in row.iter_mut() {
                point.owner = None;
                point.number = None;
            }
        }
    }

    // Resets the temporary stones placed. Returns if anything was changed
    fn reset_temp(&mut self) -> bool {
        self.curr_num = 0;
        let mut changed = false;
        for row in self.board.board.iter_mut() {
            for point in row.iter_mut() {
                if point.number.is_some() { // Temporary stone
                    changed = true;
                    point.owner = None;
                    point.number = None;
                }
            }
        }
        changed
    }
}

// Creates an empty board
fn board_init() -> Board {
    let mut board = Vector::new();
    for _ in 0..NUM_POINTS {
        let mut row = Vector::new();
        for _ in 0..NUM_POINTS {
            row.push_front(BoardPoint { owner: None, number: None } );
        }
        board.push_front(row);
    }
    Board { board: board }
}

fn main() {
    let win = WindowDesc::new(ui::build_ui())
        .window_size(WINDOW_SIZE)
        .title("Tsumego")
        .resizable(false);
    let initial_state = GameState { 
        board: board_init(), 
        curr_player: Player::Black,
        curr_num: 0,
        locked: false,
        disp_nums: true
    };

    AppLauncher::with_window(win)
        .launch(initial_state)
        .expect("Failed to launch application");
}