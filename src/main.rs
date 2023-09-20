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
    pub const NUM_POINTS: Ptidx = 9;
    pub const WINDOW_SIZE: (f64, f64) = (POINT_SIZE * NUM_POINTS as f64 + 200.0,
        POINT_SIZE * NUM_POINTS as f64);

    pub const CONTROLS_COLOR: Color = Color::GRAY;
    pub const BOARD_COLOR: Color = Color::rgb8(252, 208, 96);
    pub const NUM_COLOR: Color = Color::rgb8(180, 0, 0);

    pub const LINE_WEIGHT: f64 = 5.0;
    pub const LINE_COLOR: Color = Color::BLACK;

    pub const STONE_SIZE: f64 = 0.85; // Percentage of maximum size
    pub const STONE_WEIGHT: f64 = 2.0; // Outline weight

    pub type Ptidx = usize;
}

#[derive(Clone, Data, Lens)]
pub struct GameState {
    board: Board,
    curr_player: Player,
    curr_num: u32, // number of most recently placed stone
    play: bool,
    disp_nums: bool,
}

impl std::ops::Index<Ptidx> for GameState {
    type Output = Vector<BoardPoint>;

    fn index(&self, index: Ptidx) -> &Self::Output {
        &self.board.board[index]
    }
}

impl std::ops::IndexMut<Ptidx> for GameState {
    fn index_mut(&mut self, index: Ptidx) -> &mut Self::Output {
        &mut self.board.board[index]
    }
}

#[derive(Clone, Data, Lens)]
struct Board {
    board: Vector<Vector<BoardPoint>>,
}

#[derive(Clone, Data)]
pub struct BoardPoint {
    owner: Option<Stone>,
    number: Option<u32>,
}

#[derive(Clone, Data, PartialEq, Copy)]
enum Player {
    Black, White
}

#[derive(Clone, Data, PartialEq, Copy)]
enum Stone {
    Player(Player), BlackTemp, WhiteTemp
}

impl From<Stone> for Player {
    // Gives the player version of a stone, discarding temporary state
    fn from(stone: Stone) -> Self {
        match stone {
            Stone::Player(player) => player,
            Stone::BlackTemp => Self::Black,
            Stone::WhiteTemp => Self::White
        }
    }
}

impl Stone {
    // Gives the temporary stone version of a player
    const fn temp_from_player(player: Player) -> Self {
        Self::temp(Self::Player(player))
    }

    // Gives the temporary version of a stone
    const fn temp(stone: Self) -> Self {
        match stone {
            Self::Player(Player::Black) => Self::BlackTemp,
            Self::Player(Player::White) => Self::WhiteTemp,
            _ => stone
        }
    }
}

impl GameState {
    fn clicked_on(&mut self, i: Ptidx, j: Ptidx) -> () {
        if !self.play {
            self.toggle_stone(i, j);
        } else if self.is_valid(i, j) {
            self[i][j].owner = Some(Stone::temp_from_player(self.curr_player));
            self.curr_num += 1;
            self[i][j].number = Some(self.curr_num);
        }
    }

    // Verifies if a stone can validly be placed at i, j
    fn is_valid(&mut self, i: Ptidx, j: Ptidx) -> bool {
        true
    }

    // Toggles the stone at a point, without enforcing rules
    fn toggle_stone(&mut self, i: Ptidx, j: Ptidx) -> () {
        if self[i][j].owner == Some(Stone::Player(self.curr_player)) {
            self.remove_stone(i, j);
        } else {
            self.place_stone(i, j);
        }
    }

    // Removes the stone at the coordinates i, j
    fn remove_stone(&mut self, i: Ptidx, j: Ptidx) -> () {
        self[i][j].owner = None;
    }

    // Places a stone at the coordinates i, j
    fn place_stone(&mut self, i: Ptidx, j: Ptidx) -> () {
        self[i][j].owner = Some(
            if self.play { Stone::temp_from_player(self.curr_player) }
            else { Stone::Player(self.curr_player) }
        );
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
        play: false,
        disp_nums: true
    };

    AppLauncher::with_window(win)
        .launch(initial_state)
        .expect("Failed to launch application");
}