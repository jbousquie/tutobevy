mod debug;
mod movement;
mod spaceship;
mod asteroids;
mod camera;
mod asset_loader;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use spaceship::SpaceshipPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .run();
}

