use crate::*;

//================================-================================-================================
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugin(InputManagerPlugin::<InputAction>::default());
    }
}

//================================-================================-================================
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveBack,
    MoveForward,
    Crouch,
    Jump,
    ModSlow,
    ModFast,
    MenuEscape,
    MenuInventory,
    PrimaryAction,
    SecondaryAction,
    TertiaryAction,
    ArrowLeft,
    ArrowRight,
    ArrowDown,
    ArrowUp,
}

//================================-================================-================================
pub fn get_default_input_map(

) -> InputMap<InputAction> {
    let mut input_map = InputMap::default();
    input_map.insert(KeyCode::A, InputAction::MoveLeft)
        .insert(KeyCode::D, InputAction::MoveRight)
        .insert(KeyCode::S, InputAction::MoveBack)
        .insert(KeyCode::W, InputAction::MoveForward)
        .insert(KeyCode::C, InputAction::Crouch)
        .insert(KeyCode::Space, InputAction::Jump)
        .insert(KeyCode::LControl, InputAction::ModSlow)
        .insert(KeyCode::LShift, InputAction::ModFast)
        .insert(KeyCode::Escape, InputAction::MenuEscape)
        .insert(KeyCode::Tab, InputAction::MenuInventory)
        .insert(MouseButton::Left, InputAction::PrimaryAction)
        .insert(MouseButton::Right, InputAction::SecondaryAction)
        .insert(MouseButton::Middle, InputAction::TertiaryAction)
        .insert(KeyCode::Left, InputAction::ArrowLeft)
        .insert(KeyCode::Right, InputAction::ArrowRight)
        .insert(KeyCode::Down, InputAction::ArrowDown)
        .insert(KeyCode::Up, InputAction::ArrowUp);

    input_map
}