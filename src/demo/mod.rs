//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

use crate::asset_tracking::LoadResource;
use thiserror::Error;

mod animation;
pub mod level;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.register_asset_loader(ConfigAssetLoader).add_plugins((
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

#[derive(Default)]
struct ConfigAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum ConfigAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

impl AssetLoader for ConfigAssetLoader {
    type Asset = Config;
    type Settings = ();
    type Error = ConfigAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let config = ron::de::from_bytes::<Config>(&bytes)?;

        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
