#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
    Menu,
    Playing,
    Pause,
    Loading,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Winner,
    Tie,
    Playing,
}

pub struct Game {
    pub current_state: CurrentState,
    pub game_state: GameState,
}

impl Game {
    pub fn is_playing(&self) -> bool {
        self.current_state == CurrentState::Playing
    }

    pub fn has_winner(&self) -> bool {
        self.game_state == GameState::Winner
    }

    pub fn is_winner(&self) -> bool {
        self.game_state == GameState::Tie
    }
}