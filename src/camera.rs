use bevy::{prelude::*, input::{mouse::{MouseMotion, MouseWheel}, common_conditions::input_pressed}};

use crate::GameState;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (
                mouse_camera_zoom_system,
                mouse_camera_move_system.run_if(input_pressed(MouseButton::Right))
            ).run_if(in_state(GameState::Playing)));
    }
}
fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// TODO: It creates a lot of lag without reasons
pub fn mouse_camera_move_system(
    //time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    let sensibility = 0.5;
    for (mut transform, mut ortho) in query.iter_mut() {
        for ev in mouse_motion.iter() {
            let mut direction = Vec3::ZERO;

            direction -= Vec3::new(ev.delta.x as f32 * sensibility * -1.0, ev.delta.y as f32 * sensibility * 1.0, 0.0);

            debug!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

            let z = transform.translation.z;
            transform.translation += direction;
            transform.translation.z = z;
        }
    }
}


fn mouse_camera_zoom_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for ev in scroll_evr.iter() {
        let scroll = ev.y;
        
        for mut ortho in query.iter_mut() {
            ortho.scale = ortho.scale + (scroll * -0.1);
            debug!("Mouse wheel scrolled: {}, New scale: {}", scroll, ortho.scale);
        }
    }
}