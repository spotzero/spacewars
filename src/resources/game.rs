#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
    Menu,
    Playing,
    Pause,
    Loading,
}

pub struct Game {
    pub current_state: CurrentState,
}

impl Game {
    pub fn is_playing(&self) -> bool {
        self.current_state == CurrentState::Playing
    }
}