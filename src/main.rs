use druid::kurbo::Circle;
use druid::widget::{Label, Flex, Painter, RadioGroup, Split};
use druid::{AppLauncher, Widget, WindowDesc, Data, Lens, WidgetExt, RenderContext, Color};
use druid::im::Vector;
use druid::piet::kurbo::Line;

const WINDOW_SIZE: (f64, f64) = (600.0, 400.0);
const BOARD_SIZE: usize = 9;
const LINE_WEIGHT: f64 = 5.0;

#[derive(Clone, Data, Lens)]
struct GameState {
    board: Board,
    curr_player: Player,
    curr_num: u32, // number of most recently placed stone
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
struct BoardPoint {
    owner: Option<Player>,
    number: Option<u32>,
}

#[derive(Clone, Data, PartialEq, Copy)]
enum Player {
    Black, White
}

impl GameState {
    fn place_stone(&mut self, i: usize, j: usize) -> () {
        if self[i][j].owner == Some(self.curr_player) { // remove stone
            self[i][j].owner = None;
            // decrement curr num if this is the most recently placed stone
            if self[i][j].number == Some(self.curr_num) {
                self.curr_num -= 1;
                self[i][j].number = None;
            }
        } else { // place stone
            self[i][j].owner = Some(self.curr_player);
            self.curr_num += 1;
            self[i][j].number = Some(self.curr_num);
        }
    }
}

// Constructs the UI for a given point (i, j) on the board
fn build_point_ui(i: usize, j: usize) -> impl Widget<GameState> {
    let painter = Painter::new(move |ctx, data: &GameState, _| {
        let bounds = ctx.size().to_rect();
        let mid_x = (bounds.x0 + bounds.x1) / 2.0;
        let mid_y = (bounds.y0 + bounds.y1) / 2.0;
        // Vertical line
        ctx.stroke(
            Line::new((mid_x, bounds.y0), (mid_x, bounds.y1)),
            &Color::BLACK, LINE_WEIGHT
        );
        // Horizontal line
        ctx.stroke(
            Line::new((bounds.x0, mid_y), (bounds.x1, mid_y)),
            &Color::BLACK, LINE_WEIGHT
        );

        let circle = Circle::new((mid_x, mid_y), 
            bounds.width().min(bounds.height()) / 4.0);
        match data[i][j].owner {
            Some(Player::Black) => ctx.fill(circle, &Color::BLACK),
            Some(Player::White) => ctx.fill(circle, &Color::WHITE),
            None => ()
        };
    });

    Label::dynamic(move |data: &GameState, _| match data[i][j].number {
            Some(num) => num.to_string(),
            None => "".to_string(),
        }).with_text_color(Color::RED)
        .center()
        .background(painter)
        .on_click(move |_, data: &mut GameState, _| data.place_stone(i, j))
}

// Constructs a given row's UI
fn build_row_ui(i: usize) -> impl Widget<GameState> {
    let mut row_ui = Flex::row();
    for j in 0..BOARD_SIZE {
        row_ui = row_ui.with_flex_child(build_point_ui(i, j), 1.0);
    }
    row_ui
}

// Constructs the board's UI, with its rows
fn build_board_ui() -> impl Widget<GameState> {
    let mut board_ui = Flex::column();
    for i in 0..BOARD_SIZE {
        board_ui = board_ui.with_flex_child(build_row_ui(i), 1.0);
    }
    board_ui
}

// Constructs the UI for the control panel on the right-hand side
fn build_controls() -> impl Widget<GameState> {
    Flex::column()
        .with_default_spacer()
        .with_child(Label::new("Current Player"))
        .with_flex_child(RadioGroup::row(
            vec![("Black", Player::Black), ("White", Player::White)]
        ), 1.0)
        .lens(GameState::curr_player)
}

// Constructs the entire UI
fn build_ui() -> impl Widget<GameState> {
    Split::columns(
        build_board_ui(),
        build_controls()
    ).split_point(0.7)
}

// Creates an empty board
fn board_init() -> Board {
    let mut board = Vector::new();
    for _ in 0..BOARD_SIZE {
        let mut row = Vector::new();
        for _ in 0..BOARD_SIZE {
            row.push_front(BoardPoint { owner: None, number: None } );
        }
        board.push_front(row);
    }
    Board { board: board }
}

fn main() {
    let win = WindowDesc::new(build_ui())
        .window_size(WINDOW_SIZE)
        .title("Tsumego");
    let initial_state = GameState { 
        board: board_init(), 
        curr_player: Player::Black,
        curr_num: 0,
    };

    AppLauncher::with_window(win)
        .launch(initial_state)
        .expect("Failed to launch application");
}