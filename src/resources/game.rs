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
