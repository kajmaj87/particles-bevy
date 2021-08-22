use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

const PARTICLE_SCALE: f32 = 3.0;

struct ParticleSpawnEvent(Vec2);

fn particle_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawner: EventReader<ParticleSpawnEvent>,
) {
    for e in spawner.iter() {
        let texture_handle = asset_server.load("rolly_happy.png");
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform {
                translation: Vec3::new(e.0.x, e.0.y, 0.0),
                scale: Vec3::splat(PARTICLE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn mouse_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut spawner: EventWriter<ParticleSpawnEvent>,
) {
    let window = windows
        .get_primary()
        .expect("Primary window does not exist!");
    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("click at {:?}", window.cursor_position());
        if let Some(click_position) = window.cursor_position() {
            let translation = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            spawner.send(ParticleSpawnEvent(click_position - translation));
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_event::<ParticleSpawnEvent>()
        .add_system(mouse_handler.system())
        .add_system(particle_spawner.system())
        .run();
}
