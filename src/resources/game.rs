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
    pub end_time: f64,
    pub stopped: bool,
}

impl Game {
    pub fn is_playing(&self) -> bool {
        self.current_state == CurrentState::Playing
    }

    pub fn game_over(&self) -> bool {
        self.game_state != GameState::Playing
    }
}
