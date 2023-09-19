use crate::druid_use::*;

// Creates a greyer version of a color
fn grey_color(color: Color) -> Color {
    let (mut r, mut g, mut b, a) = color.as_rgba();

    fn grey_c(c: f64) -> f64 {
        const CLR_AMT: f64 = 0.6; // percentage of color to keep
        const GREY: f64 = 0.5;
        c * CLR_AMT + GREY * (1.0 - CLR_AMT)
    }

    r = grey_c(r);
    g = grey_c(g);
    b = grey_c(b);

    Color::rgba(r, g, b, a)
}

// Constructs a UI for one side of a toggle button
fn build_toggle_side<T: Data + Clone>(name: impl Into<LabelText<T>> + 'static,
        active_if: impl Fn(&T, &Env) -> bool + 'static,
        on_click: impl Fn(&mut EventCtx<'_, '_>, &mut T, &Env) + 'static,
        button_color: Color, text_color: Color, grey_out: bool) ->
        impl Widget<T> {

    let painter = Painter::new(
            move |ctx, data: &T, env| {
        let bounds = ctx.size().to_rect().inset(-2.0).to_rounded_rect(5.0);
        if active_if(data, env) || !grey_out {
            ctx.fill(bounds, &button_color);
        } else { // grey out the button
            ctx.fill(bounds, &grey_color(button_color));
        }
        if active_if(data, env) || ctx.is_hot() {
            ctx.stroke(bounds, &text_color, 2.0);
        }
    });

    Label::new(name)
        .with_text_color(text_color)
        .padding((10.0, 5.0, 10.0, 5.0))
        .center()
        .background(painter)
        .on_click(on_click)
}

pub fn build_button<T: Data + Clone>(name: impl Into<LabelText<T>> + 'static,
        on_click: impl Fn(&mut EventCtx<'_, '_>, &mut T, &Env) + 'static,
        button_color: Color, text_color: Color) ->
        impl Widget<T> {
    Flex::row()
        .with_default_spacer()
        .with_child(build_toggle_side(name, |_, _| false, on_click, button_color, text_color, false))
        .with_default_spacer()
}

// Constructs a complete toggle button
pub fn build_toggle<T: Data + Clone>(
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
            left_button_color, left_text_color, true),
        1.0)
        .with_default_spacer()
        .with_flex_child(
            build_toggle_side(right_name, right_active_if, right_on_click,
            right_button_color, right_text_color, true),
        1.0)
        .with_default_spacer()
}