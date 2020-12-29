use bevy::{input::keyboard::{KeyboardInput, keyboard_input_system}, math::vec3, prelude::*};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(setup_sounds.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("merry_christmas.jpg");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)),
            ..Default::default()
        });
}

fn setup_sounds(asset_server: Res<AssetServer>, audio: Res<Audio>)  {
    let music = asset_server.load("mix_23m49s (audio-joiner.com).mp3");

        audio.play(music);
}