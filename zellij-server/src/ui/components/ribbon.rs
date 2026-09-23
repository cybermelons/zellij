use super::{text::stringify_text, Coordinates, Text};
use crate::panes::terminal_character::{AnsiCode, CharacterStyles, RESET_STYLES};
use zellij_utils::data::{PaletteColor, Style};

static LEFT_CAP: &str = "";
static RIGHT_CAP: &str = "";

pub fn ribbon(
    content: Text,
    style: &Style,
    arrow_fonts: bool,
    component_coordinates: Option<Coordinates>,
) -> Vec<u8> {
    let colors = style.colors;
    let background = colors.text_unselected.background;
    if content.disabled {
        let declaration = colors.ribbon_unselected;
        let disabled_content = content.into_disabled();
        let (first_arrow_styles, text_style, last_arrow_styles) = (
            character_style(declaration.background, background),
            RESET_STYLES
                .foreground(Some(declaration.base.into()))
                .background(Some(declaration.background.into()))
                .italic(Some(AnsiCode::On)),
            character_style(declaration.background, background),
        );
        let (left_cap, right_cap, padding) = if arrow_fonts {
            (LEFT_CAP, RIGHT_CAP, Some(4))
        } else {
            ("", "", None)
        };
        let (text, _text_width) = stringify_text(
            &disabled_content,
            padding,
            &component_coordinates,
            &declaration,
            &colors,
            text_style,
        );
        let mut stringified = component_coordinates
            .map(|c| c.to_string())
            .unwrap_or_else(|| String::new());
        stringified.push_str(&format!(
            "{}{}{}{} {} {}{}{}",
            RESET_STYLES,
            first_arrow_styles,
            left_cap,
            text_style,
            text,
            last_arrow_styles,
            right_cap,
            RESET_STYLES
        ));
        return stringified.as_bytes().to_vec();
    }
    let declaration = if content.selected {
        colors.ribbon_selected
    } else {
        colors.ribbon_unselected
    };
    let (first_arrow_styles, text_style, last_arrow_styles) = (
        character_style(declaration.background, background),
        character_style(declaration.base, declaration.background),
        character_style(declaration.background, background),
    );

    let (left_cap, right_cap, padding) = if arrow_fonts {
        (LEFT_CAP, RIGHT_CAP, Some(4))
    } else {
        ("", "", None)
    };

    let (text, _text_width) = stringify_text(
        &content,
        padding,
        &component_coordinates,
        &declaration,
        &colors,
        text_style,
    );
    let mut stringified = component_coordinates
        .map(|c| c.to_string())
        .unwrap_or_else(|| String::new());
    stringified.push_str(&format!(
        "{}{}{}{} {} {}{}{}",
        RESET_STYLES,
        first_arrow_styles,
        left_cap,
        text_style,
        text,
        last_arrow_styles,
        right_cap,
        RESET_STYLES
    ));
    stringified.as_bytes().to_vec()
}

fn character_style(foreground: PaletteColor, background: PaletteColor) -> CharacterStyles {
    RESET_STYLES
        .foreground(Some(foreground.into()))
        .background(Some(background.into()))
        .bold(Some(AnsiCode::On))
}
