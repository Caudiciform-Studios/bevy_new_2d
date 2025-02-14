//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::asset_tracking::LoadResource;

mod animation;
pub mod level;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        RonAssetPlugin::<Config>::new(&[".ron"]),
        animation::plugin,
        movement::plugin,
        player::plugin,
        level::plugin,
    ));

    app.load_resource_from_path::<Config>("config.ron");
}

/// Configuration data which can be used to tweak the game's properties.
/// When running in dev mode on desktop the values in this resource
/// will be automatically updated to match changes in the assets/config.ron
/// file otherwise changes will only take effect on restart.
#[derive(Debug, Clone, Asset, Reflect, Resource, serde::Deserialize)]
pub struct Config {
    max_player_speed: f32,
}
