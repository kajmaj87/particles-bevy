use bevy::prelude::*;

fn setup(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

const PARTICLE_SCALE: f32 = 3.0;

fn mouse_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows
        .get_primary()
        .expect("Primary window does not exist!");
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let texture_handle = asset_server.load("rolly_happy.png");
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform {
                translation: Vec3::new(
                    window.cursor_position().unwrap().x - window.width()/2.0,
                    window.cursor_position().unwrap().y - window.height()/2.0,
                    0.0,
                ),
                scale: Vec3::splat(PARTICLE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        });
        println!("click at {:?}", window.cursor_position());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(mouse_handler.system())
        .run();
}
