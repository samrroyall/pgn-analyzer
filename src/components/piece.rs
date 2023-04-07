use pleco::{PieceType};
use stylist::{css, yew::styled_component};
use yew::{html, Html, Properties};

const PIECE_ICON_IDS: [char; 12] = [
    '\u{2659}',
    '\u{2658}',
    '\u{2657}',
    '\u{2656}',
    '\u{2655}',
    '\u{2654}',
    '\u{265f}',
    '\u{265e}',
    '\u{265d}',
    '\u{265c}',
    '\u{265b}',
    '\u{265a}',
];

fn pretty_piece(c: char) -> char {
    match c {
        '-' => ' ',
        _ => {
            let player = !c.is_ascii_uppercase() as usize;

            let piece_type = match c.to_ascii_uppercase() {
                'P' => PieceType::P,
                'N' => PieceType::N,
                'B' => PieceType::B,
                'R' => PieceType::R,
                'Q' => PieceType::Q,
                'K' => PieceType::K,
                _ => unreachable!("unexpected character: {}", c),
            } as usize;

            PIECE_ICON_IDS[(piece_type - 1) + 6 * player]
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct PieceProps {
    pub piece_char: char,
}

#[styled_component(PieceUi)]
pub fn piece(props: &PieceProps) -> Html {
    let piece_styles = css!(r#"
        cursor: pointer;
        font-size: 3rem;
    "#);

    html! { 
        <span class={piece_styles}>
            {pretty_piece(props.piece_char)}
        </span>
    }
}