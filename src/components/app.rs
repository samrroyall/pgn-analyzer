use std::rc::Rc;

use crate::components::board::BoardUi;
use pleco::Board;
use stylist::yew::styled_component;
use yew::{html, Html, Reducible, use_reducer};

enum BoardAction {
    ApplyMove(String),
}

struct BoardState {
    board: Board,
}

impl Default for BoardState {
    fn default() -> Self {
        Self { board: Board::start_pos() }
    }
}

impl Reducible for BoardState {
    type Action = BoardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_board = match action {
            BoardAction::ApplyMove(uci_move) => {
                let mut board = self.board.shallow_clone();
                assert!(board.apply_uci_move(uci_move.as_str()));
                board
            },
        };

        Self { board: next_board }.into()
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let board = use_reducer(BoardState::default);

    html! {
        <BoardUi board_str={board.board.pretty_string()} />
    }
}
