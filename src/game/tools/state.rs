use crate::*;

//================================-================================-================================
pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_state::<AppState>();
    }
}

//================================-================================-================================
#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Gameplay,
    Death,
}