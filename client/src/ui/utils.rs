/**
 * CREDIT TO https://github.com/vfreitas-/bevy-quick-warrior -- Using
 */
use bevy::prelude::*;

pub fn basic_text(
    text: &str,
    size: f32,
    font: Handle<Font>,
    margin_top: Option<f32>,
    margin_bottom: Option<f32>,
    margin_right: Option<f32>,
    color : Color
) -> TextBundle {
    let _margin_top = if margin_top.is_some() {
        margin_top.unwrap()
    } else {
        0.
    };
    let _margin_bottom = if margin_bottom.is_some() {
        margin_bottom.unwrap()
    } else {
        0.
    };
    let _margin_right = if margin_right.is_some() {
        margin_right.unwrap()
    } else {
        0.
    };

    return TextBundle {
        style: Style {
            margin: UiRect {
                left: Val::Px(0.),
                right: Val::Px(_margin_right),
                top:Val::Px(_margin_top),
                bottom: Val::Px(_margin_bottom),
            },
            ..Default::default()
        },
        text: Text::from_section(
            text,
            TextStyle {
                font: font,
                font_size: size,
                color
            },
        ),
        ..Default::default()
    };
}