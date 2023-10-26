// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::io::Cursor;

use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow},
    winit::WinitWindows,
};
use echapp_city_builder::GamePlugin;
use winit::window::Icon;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Immediate,
                    resizable: false,
                    title: "City Builder".into(),
                    ..default()
                }),
                ..default()
            }), //.set(ImagePlugin::default_nearest()),
        )
        .add_plugins(GamePlugin)
        .add_systems(Startup, (setup, set_window_icon))
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
