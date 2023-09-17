use druid::kurbo::Circle;
use druid::widget::{Label, Flex, LensWrap, Painter};
use druid::{AppLauncher, Widget, WindowDesc, Data, Lens, WidgetExt, PaintCtx, RenderContext, Color, Env};
use druid::im::Vector;
use druid::piet::kurbo::Line;

const WINDOW_SIZE: (f64, f64) = (600.0, 400.0);
const BOARD_SIZE: usize = 9;
const LINE_WEIGHT: f64 = 5.0;

#[derive(Clone, Data, Lens)]
struct GameState {
    board: Board,
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

#[derive(Clone, Data, PartialEq)]
enum Player {
    Black, White
}

fn build_point_ui(i: usize, j: usize) -> impl Widget<Board> {
    let painter = Painter::new(move |ctx: &mut PaintCtx<'_, '_, '_>, data: &Board, _: &Env| {
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
        match data.board[i][j].owner {
            Some(Player::Black) => ctx.fill(circle, &Color::BLACK),
            Some(Player::White) => ctx.fill(circle, &Color::WHITE),
            None => ()
        };
    });

    Label::dynamic(move |data: &Board, _| match data.board[i][j].number {
        Some(num) => num.to_string(),
        None => "".to_string(),
    }).with_text_color(Color::RED).center().background(painter)
}

// Constructs a given row's UI
fn build_row_ui(i: usize) -> impl Widget<Board> {
    let mut row_ui = Flex::row();
    for j in 0..BOARD_SIZE {
        row_ui = row_ui.with_flex_child(build_point_ui(i, j), 1.0);
    }
    row_ui
}

// Constructs the board's UI, with its rows
fn build_board_ui() -> impl Widget<Board> {
    let mut board_ui = Flex::column();
    for i in 0..BOARD_SIZE {
        board_ui = board_ui.with_flex_child(build_row_ui(i), 1.0);
    }
    board_ui
}

// Constructs the entire UI
fn build_ui() -> impl Widget<GameState> {
    LensWrap::new(build_board_ui(), GameState::board)
}

// Creates an empty board
fn board_init() -> Board {
    let mut board = Vector::new();
    for _ in 0..BOARD_SIZE {
        let mut row = Vector::new();
        for _ in 0..BOARD_SIZE {
            row.push_front(BoardPoint { owner: Some(Player::White), number: Some(1) } );
        }
        board.push_front(row);
    }
    Board { board: board }
}

fn main() {
    let win = WindowDesc::new(build_ui())
        .window_size(WINDOW_SIZE)
        .title("Tsumego");
    let initial_state = GameState { board : board_init() };

    AppLauncher::with_window(win)
        .launch(initial_state)
        .expect("Failed to launch application");
}