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
    pub const NUM_POINTS: PtIdx = 9;
    pub const WINDOW_SIZE: (f64, f64) = (POINT_SIZE * NUM_POINTS as f64 + 200.0,
        POINT_SIZE * NUM_POINTS as f64);

    pub const CONTROLS_COLOR: Color = Color::GRAY;
    pub const BOARD_COLOR: Color = Color::rgb8(252, 208, 96);
    pub const NUM_COLOR: Color = Color::rgb8(180, 0, 0);

    pub const LINE_WEIGHT: f64 = 5.0;
    pub const LINE_COLOR: Color = Color::BLACK;

    pub const STONE_SIZE: f64 = 0.85; // Percentage of maximum size
    pub const STONE_WEIGHT: f64 = 2.0; // Outline weight

    pub type PtIdx = usize;
    pub type StnNum = u32;
}

#[derive(Clone, Data, Lens)]
pub struct GameState {
    board: Board,
    curr_player: Player,
    curr_num: StnNum, // Number of most recently placed game stone
    playing: bool, // True if playing, false if setting up
    disp_nums: bool
}

impl std::ops::Index<PtIdx> for GameState {
    type Output = Vector<BoardPoint>;

    fn index(&self, index: PtIdx) -> &Self::Output {
        &self.board.board[index]
    }
}

impl std::ops::IndexMut<PtIdx> for GameState {
    fn index_mut(&mut self, index: PtIdx) -> &mut Self::Output {
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
    number: Option<StnNum>,
}

#[derive(Clone, Data, PartialEq, Copy)]
enum Player {
    Black, White
}

#[derive(Clone, Data, PartialEq, Copy)]
enum Stone {
    Player(Player), BlackSetup, WhiteSetup
}

impl From<Stone> for Player {
    // Gives the player version of a stone, discarding setup state
    fn from(stone: Stone) -> Self {
        match stone {
            Stone::Player(player) => player,
            Stone::BlackSetup => Self::Black,
            Stone::WhiteSetup => Self::White
        }
    }
}

impl Stone {
    // Gives the setup stone version of a player
    const fn setup_from_player(player: Player) -> Self {
        Self::setup(Self::Player(player))
    }

    // Gives the setup version of a stone
    const fn setup(stone: Self) -> Self {
        match stone {
            Self::Player(Player::Black) => Self::BlackSetup,
            Self::Player(Player::White) => Self::WhiteSetup,
            _ => stone
        }
    }
}

impl GameState {
    // Triggered when a point is clicked on, and handles that outcome
    fn clicked_on(&mut self, i: PtIdx, j: PtIdx) -> () {
        if !self.playing {
            self.toggle_setup_stone(i, j);
        } else if self.is_valid(i, j) {
            // TODO: rules
            self[i][j].owner = Some(Stone::Player(self.curr_player));
            self.curr_num += 1;
            self[i][j].number = Some(self.curr_num);

            self.toggle_player();
        }
    }

    // Toggles the current player
    fn toggle_player(&mut self) -> () {
        self.curr_player = match self.curr_player {
            Player::Black => Player::White,
            Player::White => Player::Black
        }
    }

    // Verifies if a stone can validly be placed at i, j
    fn is_valid(&self, i: PtIdx, j: PtIdx) -> bool {
        if self[i][j].owner.is_some() {
            false
        } else {
            true
        }
    }

    // Toggles the stone at a point, without enforcing rules
    fn toggle_setup_stone(&mut self, i: PtIdx, j: PtIdx) -> () {
        if self[i][j].owner == Some(Stone::setup_from_player(self.curr_player)) {
            self.remove_setup_stone(i, j);
        } else {
            self.place_setup_stone(i, j);
        }
    }

    // Removes the stone at the coordinates i, j
    fn remove_setup_stone(&mut self, i: PtIdx, j: PtIdx) -> () {
        self[i][j].owner = None;
    }

    // Places a stone at the coordinates i, j
    fn place_setup_stone(&mut self, i: PtIdx, j: PtIdx) -> () {
        self[i][j].owner = Some(Stone::setup_from_player(self.curr_player));
    }

    // Completely resets the entire program
    fn reset(&mut self) -> () {
        self.curr_player = Player::Black;
        self.curr_num = 0;
        self.playing = false;
        self.disp_nums = true;
        for row in self.board.board.iter_mut() {
            for point in row.iter_mut() {
                point.owner = None;
                point.number = None;
            }
        }
    }

    // Returns whether there are game stones on the board
    fn has_game_stones(&self) -> bool {
        self.curr_num != 0
    }

    // Resets the game stones placed. Returns if anything was changed
    fn reset_to_setup(&mut self) -> () {
        self.playing = false;
        self.curr_num = 0;
        for row in self.board.board.iter_mut() {
            for point in row.iter_mut() {
                if point.number.is_some() { // Setup stone
                    point.owner = None;
                    point.number = None;
                }
            }
        }
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
        playing: false,
        disp_nums: true
    };

    AppLauncher::with_window(win)
        .launch(initial_state)
        .expect("Failed to launch application");
}