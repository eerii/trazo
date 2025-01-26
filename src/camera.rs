//! Handles the game rendering.

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Startup), init_camera);
}

// Components
// ---

/// The camera where the game is being rendered.
#[derive(Component)]
pub struct GameCamera;

// Systems
// ---

/// Spawn the main camera.
fn init_camera(mut cmd: Commands) {
    cmd.spawn((Camera2d, GameCamera));
}
