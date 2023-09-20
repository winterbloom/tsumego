use crate::druid_use::*;
use crate::consts::*;

use crate::widgets;

use crate::GameState;
use crate::Player;

// Constructs the entire UI
pub fn build_ui() -> impl Widget<GameState> {
    Split::columns(
        Container::new(board_ui::build_board_ui()).background(BOARD_COLOR),
        Container::new(build_controls()).background(CONTROLS_COLOR)
    ).split_point(0.7)
}

// Constructs the UI for the control panel on the right-hand side
fn build_controls() -> impl Widget<GameState> {
    Split::rows(
        Flex::column()
            .with_default_spacer()
            .with_flex_child(Label::new("Current Player"), 1.0)
            .with_child(
                widgets::build_toggle("Black",
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
                widgets::build_toggle("Yes",
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
            ),
        Flex::column()
            .with_default_spacer()
            .with_flex_child(Label::new("Display Numbers"), 1.0)
            .with_child(
                widgets::build_toggle("Yes",
                |data: &GameState, _| data.disp_nums, 
                |_, data: &mut GameState, _| data.disp_nums = true,
                Color::BLACK, Color::WHITE,
                "No",
                |data: &GameState, _| !data.disp_nums,
                |_, data: &mut GameState, _| data.disp_nums = false,
                Color::BLACK, Color::WHITE)
            )
            .with_default_spacer()
            .with_child(
                widgets::build_button("Reset",
                |_, data: &mut GameState, _| {
                    if !data.locked || !data.reset_temp() {
                        data.reset()
                    }
                }, Color::BLACK, Color::WHITE)
            )
    ).split_point(0.7)
}

mod board_ui {
    use crate::druid_use::*;
    use crate::consts::*;
    use crate::GameState;
    use crate::Player;
    use crate::Stone;

    // Color of stone corresponding to a given stone
    const fn stone_color(stone: Stone) -> Color {
        match stone {
            Stone::Player(Player::Black) => Color::BLACK,
            Stone::Player(Player::White) => Color::WHITE,
            Stone::BlackTemp => Color::grey8(40),
            Stone::WhiteTemp => Color::grey8(230)
        }
    }

    // Color to outline a stone with
    const fn stone_outline_color(stone: Stone) -> Color {
        match stone {
            Stone::Player(Player::Black) => Color::grey8(50),
            Stone::Player(Player::White) => Color::grey8(230),
            Stone::BlackTemp => Color::grey8(65),
            Stone::WhiteTemp => Color::grey8(200)
        }
    }

    // Creates a painter for a point, including possibly its stone
    // (not the number, though)
    fn point_painter_init(i: Ptidx, j: Ptidx) -> Painter<GameState> {
        Painter::new(move |ctx, data: &GameState, _| {
            let bounds = ctx.size().to_rect();
            let (mid_x, mid_y) = bounds.center().into();
            let inset = bounds.inset(2.0);
            
            // Vertical line
            ctx.stroke(
                Line::new((mid_x, inset.y0), (mid_x, inset.y1)),
                &LINE_COLOR, LINE_WEIGHT
            );
            // Horizontal line
            ctx.stroke(
                Line::new((inset.x0, mid_y), (inset.x1, mid_y)),
                &LINE_COLOR, LINE_WEIGHT
            );

            // Draw stone, if one exists
            if let Some(stone) = data[i][j].owner {
                let circle = Circle::new((mid_x, mid_y), 
                    bounds.width().min(bounds.height()) / 2.0 * STONE_SIZE);

                ctx.fill(circle, &stone_color(stone));
                ctx.stroke(circle, &stone_outline_color(stone), STONE_WEIGHT);
            }
        })
    }

    // Constructs the UI for a given point (i, j) on the board
    fn build_point_ui(i: Ptidx, j: Ptidx) -> impl Widget<GameState> {
        let painter = point_painter_init(i, j);

        Label::dynamic(move |data: &GameState, _| {
                if data.disp_nums {
                    if let Some(num) = data[i][j].number {
                        return num.to_string();
                    }
                };
                "".to_string()
            })
            .with_text_color(NUM_COLOR)
            .center()
            .background(painter)
            .on_click(move |_, data: &mut GameState, _| data.clicked_on(i, j))
    }

    // Constructs a given row's UI
    fn build_row_ui(i: Ptidx) -> impl Widget<GameState> {
        let mut row_ui = Flex::row();
        for j in 0..NUM_POINTS {
            row_ui = row_ui.with_flex_child(build_point_ui(i, j), 1.0);
        }
        row_ui
    }

    // Constructs the board's UI, with its rows
    pub fn build_board_ui() -> impl Widget<GameState> {
        let mut board_ui = Flex::column();
        for i in 0..NUM_POINTS {
            board_ui = board_ui.with_flex_child(build_row_ui(i), 1.0);
        }
        board_ui
    }
}