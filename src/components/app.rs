use crate::components::board::BoardUi;

use pleco::Board;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store)]
struct AppState {
    board: Board,
    error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            board: Board::start_pos(),
            error: None,
        }
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<AppState>();

    let onchange = dispatch.reduce_callback_with(|state, e: Event| {
        let input = e.target_unchecked_into::<HtmlInputElement>();

        let mut new_board = state.board.shallow_clone();

        if let Ok(uci_move) = input.value().parse::<String>() {
            if uci_move.len() == 0 {
                return AppState {
                    board: new_board,
                    error: None,
                }.into();
            }

            let move_success = new_board.apply_uci_move(uci_move.as_str());

            if move_success {
                AppState {
                    board: new_board,
                    error: None,
                }.into()
            } else {
                AppState {
                    board: new_board,
                    error: Some(String::from("Invalid move provided")),
                }.into()
            }
            
        } else {
            AppState {
                board: new_board,
                error: Some(String::from("Unable to parse input value")),
            }.into()
        }
    });

    let container_styles = css!(r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    "#);

    html! {
        <div class={container_styles}>
            <BoardUi board_str={store.board.pretty_string()} />
            <span style="color: red;"> { store.error.as_ref().unwrap_or(&String::new()) } </span>
            <input
                {onchange}
                placeholder="Paste UCI move here..."
            />
        </div>
    }
}
