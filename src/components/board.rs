use crate::components::piece::PieceUi;
use stylist::yew::styled_component;
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board_str: String,
}

#[styled_component(BoardUi)]
pub fn board(props: &BoardProps) -> Html {
    let cell_color = |i, j| {
        if i % 2 == j % 2 { "beige" } else { "green" }
    };

    let board_styles = css!(r#"
        margin: 0 auto;
        border: 1px solid black;
        border-collapse: collapse;
        user-select: none;
    "#);

    let cell_container_styles = css!(r#"
        height: 5rem;
        width: 5rem;
    "#);
    
    let cell_styles = css!(r#"
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
        width: 100%;
    "#);

    let chars = props.board_str
                    .chars()
                    .filter(|c| ![' ', '\n'].contains(c))
                    .collect::<Vec<char>>();

    html! {
        <table class={board_styles}>{ 
                (0..8).map(|i| html! {
                    <tr key={format!("board-rank-{}", i)}>{
                         (0..8).map(|j| html! {
                            <td 
                                class={cell_container_styles.clone()}
                                style={format!("background-color: {}", cell_color(i, j))}
                                key={format!("board-square-{}-{}", i, j)}
                            >
                                <div class={cell_styles.clone()}>
                                    <PieceUi piece_char={chars[i*8 + j]} /> 
                                </div>
                            </td> 
                        }).collect::<Html>()
                    }</tr>
                }).collect::<Html>()
        }</table>
    }
}

