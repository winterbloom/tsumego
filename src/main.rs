use druid::kurbo::Circle;
use druid::widget::{Label, Flex, Painter, Split, Container, Button, LabelText};
use druid::{AppLauncher, Widget, WindowDesc, Data, Lens, WidgetExt, RenderContext, Color, EventCtx, Env};
use druid::im::Vector;
use druid::piet::kurbo::Line;

const POINT_SIZE: f64 = 50.0;
const NUM_POINTS: usize = 9;
const WINDOW_SIZE: (f64, f64) = (POINT_SIZE * NUM_POINTS as f64 + 200.0, POINT_SIZE * NUM_POINTS as f64);
const BOARD_COLOR: Color = Color::rgb8(252, 208, 96);

const LINE_WEIGHT: f64 = 5.0;
const LINE_COLOR: Color = Color::BLACK;

const STONE_SIZE: f64 = 0.85; // Percentage of maximum size
const STONE_WEIGHT: f64 = 2.0; // Outline weight

// TODO: allow toggling of whether to use numbers; start on rules

#[derive(Clone, Data, Lens)]
struct GameState {
    board: Board,
    curr_player: Player,
    curr_num: u32, // number of most recently placed stone
    locked: bool,
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
        self[i][j].owner = Some(self.curr_player);
        if self.locked {
            self.curr_num += 1;
            self[i][j].number = Some(self.curr_num);
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

// Color corresponding to a given player
fn player_color(player: Player) -> Color {
    match player {
        Player::Black => Color::BLACK,
        Player::White => Color::WHITE
    }
}

// Color corresponding to the opposite player
fn player_color_inv(player: Player) -> Color {
    match player {
        Player::Black => Color::grey(0.2),
        Player::White => Color::grey(0.9)
    }
}

// Creates a painter for a point, including possibly its stone
// (not the number, though)
fn point_painter_init(i: usize, j: usize) -> Painter<GameState> {
    Painter::new(move |ctx, data: &GameState, _| {
        let bounds = ctx.size().to_rect();
        let mid_x = (bounds.x0 + bounds.x1) / 2.0;
        let mid_y = (bounds.y0 + bounds.y1) / 2.0;
        
        // Vertical line
        ctx.stroke(
            Line::new((mid_x, bounds.y0 - 2.0), (mid_x, bounds.y1 + 2.0)),
            &LINE_COLOR, LINE_WEIGHT
        );
        // Horizontal line
        ctx.stroke(
            Line::new((bounds.x0 - 2.0, mid_y), (bounds.x1 + 2.0, mid_y)),
            &LINE_COLOR, LINE_WEIGHT
        );

        // Draw stone, if one exists
        if let Some(player) = data[i][j].owner {
            let circle = Circle::new((mid_x, mid_y), 
                bounds.width().min(bounds.height()) / 2.0 * STONE_SIZE);

            ctx.fill(circle, &player_color(player));
            ctx.stroke(circle, &player_color_inv(player), STONE_WEIGHT);
        }
    })
}

// Constructs the UI for a given point (i, j) on the board
fn build_point_ui(i: usize, j: usize) -> impl Widget<GameState> {
    let painter = point_painter_init(i, j);

    Label::dynamic(move |data: &GameState, _| match data[i][j].number {
            Some(num) => num.to_string(),
            None => "".to_string(),
        }).with_text_color(Color::RED)
        .center()
        .background(painter)
        .on_click(move |_, data: &mut GameState, _| data.toggle_stone(i, j))
}

// Constructs a given row's UI
fn build_row_ui(i: usize) -> impl Widget<GameState> {
    let mut row_ui = Flex::row();
    for j in 0..NUM_POINTS {
        row_ui = row_ui.with_flex_child(build_point_ui(i, j), 1.0);
    }
    row_ui
}

// Constructs the board's UI, with its rows
fn build_board_ui() -> impl Widget<GameState> {
    let mut board_ui = Flex::column();
    for i in 0..NUM_POINTS {
        board_ui = board_ui.with_flex_child(build_row_ui(i), 1.0);
    }
    board_ui
}

// Constructs a UI for one side of a toggle button
fn build_toggle_side<T: Data + Clone>(name: impl Into<LabelText<T>> + 'static,
        active_if: impl Fn(&T, &Env) -> bool + 'static,
        on_click: impl Fn(&mut EventCtx<'_, '_>, &mut T, &Env) + 'static,
        button_color: Color, text_color: Color) ->
        impl Widget<T> {

    let painter = Painter::new(
            move |ctx, data: &T, env| {
        let bounds = ctx.size().to_rect();
        if active_if(data, env) {
            ctx.fill(bounds, &button_color);
            ctx.stroke(bounds, &text_color, 2.0);
        } else {
            // grey out the color if it's not active
            let (mut r, mut g, mut b, a) = button_color.as_rgba();

            fn grey_color(c: f64) -> f64 {
                const CLR_AMT: f64 = 0.6; // percentage of color to keep
                const GREY: f64 = 0.5;
                c * CLR_AMT + GREY * (1.0 - CLR_AMT)
            }

            r = grey_color(r);
            g = grey_color(g);
            b = grey_color(b);

            ctx.fill(bounds, &Color::rgba(r, g, b, a));

            if ctx.is_hot() {
                ctx.stroke(bounds, &text_color, 2.0);
            }
        }
    });

    Label::new(name)
        .with_text_color(text_color)
        .center()
        .background(painter)
        .on_click(on_click)
}

// Constructs a complete toggle button
fn build_toggle<T: Data + Clone>(
        left_name: impl Into<LabelText<T>> + 'static,
        left_active_if: impl Fn(&T, &Env) -> bool + 'static,
        left_on_click: impl Fn(&mut EventCtx<'_, '_>, &mut T, &Env) + 'static,
        left_button_color: Color, left_text_color: Color,
        right_name: impl Into<LabelText<T>> + 'static,
        right_active_if: impl Fn(&T, &Env) -> bool + 'static,
        right_on_click: impl Fn(&mut EventCtx<'_, '_>, &mut T, &Env) + 'static,
        right_button_color: Color, right_text_color: Color) ->
        impl Widget<T> {
    Flex::row()
        .with_default_spacer()
        .with_flex_child(
            build_toggle_side(left_name, left_active_if, left_on_click, 
            left_button_color, left_text_color),
        1.0)
        .with_default_spacer()
        .with_flex_child(
            build_toggle_side(right_name, right_active_if, right_on_click,
            right_button_color, right_text_color)
        , 1.0)
}

// Constructs the UI for the control panel on the right-hand side
fn build_controls() -> impl Widget<GameState> {
    Flex::column()
        .with_default_spacer()
        .with_flex_child(Label::new("Current Player"), 1.0)
        .with_child(
            build_toggle("Black",
            |data: &GameState, _| data.curr_player == Player::Black,
            |_, data: &mut GameState, _| data.curr_player = Player::Black,
            Color::BLACK, Color::WHITE,
            "White",
            |data: &GameState, _| data.curr_player == Player::White,
            |_, data: &mut GameState, _| data.curr_player = Player::White,
            Color::WHITE, Color::BLACK)
        )
        .with_default_spacer()
        .with_flex_child(Label::new("Locked"), 1.0)
        .with_child(
            build_toggle("Yes",
            |data: &GameState, _| data.locked,
            |_, data: &mut GameState, _| data.locked = true, 
            Color::GREEN, Color::WHITE,
            "No",
            |data: &GameState, _| !data.locked,
            |_, data: &mut GameState, _| {
                data.locked = false;
                data.reset_temp();
            },
            Color::RED, Color::WHITE)
        )
        .with_default_spacer()
        .with_flex_child(
            Button::new("Reset")
                .on_click(|_, data: &mut GameState, _| {
                    if !data.locked || !data.reset_temp() {
                        data.reset()
                    }
                }),
        1.0)
}

// Constructs the entire UI
fn build_ui() -> impl Widget<GameState> {
    Split::columns(
        Container::new(build_board_ui()).background(BOARD_COLOR),
        Container::new(build_controls()).background(Color::GRAY)
    ).split_point(0.7)
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
    let win = WindowDesc::new(build_ui())
        .window_size(WINDOW_SIZE)
        .title("Tsumego")
        .resizable(false);
    let initial_state = GameState { 
        board: board_init(), 
        curr_player: Player::Black,
        curr_num: 0,
        locked: false
    };

    AppLauncher::with_window(win)
        .launch(initial_state)
        .expect("Failed to launch application");
}