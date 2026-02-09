use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerActions>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerActions {
    Jump,
    Left,
    Right,
    Forward,
    Back,
}

impl PlayerActions {
    pub const DIRECTIONS: [Self; 4] = [
        PlayerActions::Forward,
        PlayerActions::Back,
        PlayerActions::Left,
        PlayerActions::Right,
    ];

    pub fn direction(self) -> Option<Dir3> {
        match self {
            PlayerActions::Forward => Some(Dir3::NEG_Z),
            PlayerActions::Back => Some(Dir3::Z),
            PlayerActions::Left => Some(Dir3::NEG_X),
            PlayerActions::Right => Some(Dir3::X),
            _ => None,
        }
    }

    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(PlayerActions::Jump, KeyCode::Space);
        input_map.insert(PlayerActions::Forward, KeyCode::KeyW);
        input_map.insert(PlayerActions::Back, KeyCode::KeyS);
        input_map.insert(PlayerActions::Left, KeyCode::KeyA);
        input_map.insert(PlayerActions::Right, KeyCode::KeyD);

        input_map
    }
}
