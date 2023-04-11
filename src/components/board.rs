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

    let chars = 
        props.board_str
            .chars()
            .filter(|c| ![' ', '\n'].contains(c))
            .collect::<Vec<char>>();

    html! {
        <table class={board_styles}>
            { 
                (0..8).map(|i| html! {
                    <tr key={format!("board-rank-{}", i)}>{
                        (0..9).map(|j| html! {
                            if j == 0 {
                                <th 
                                    class={cell_container_styles.clone()}
                                    key={format!("board-rank-legend-{}", i+1)}
                                >
                                    <div class={cell_styles.clone()}>{ 
                                        ('8' as u8 - i as u8) as char 
                                    }</div>
                                </th>
                            } else {
                                <td 
                                    class={cell_container_styles.clone()}
                                    style={format!("background-color: {}", cell_color(i, j-1))}
                                    key={format!("board-square-{}-{}", i, j-1)}
                                >
                                    <div class={cell_styles.clone()}>
                                        <PieceUi piece_char={chars[i*8 + j-1]} /> 
                                    </div>
                                </td> 
                            }
                        })
                        .collect::<Html>()
                    }</tr>
                })
                .collect::<Html>()
            } 
            <tr key="board-file-legend-row">{ 
                (0..9).map(|j| html! {
                    <th 
                        key={format!("board-file-legend-{}", j)} 
                        class={cell_container_styles.clone()}
                    >
                        <div class={cell_styles.clone()}>{
                            if j == 0 { ' ' } else { ('a' as u8 + j as u8 - 1) as char }
                        }</div>
                    </th>
                })
                .collect::<Html>()
            }</tr>
        </table>
    }
}

