use bevy::prelude::*;
use diagnostic_plugin::DiagnosticPlugin;

mod diagnostic_plugin;

const PARTICLE_SCALE: f32 = 3.0;

struct ParticleSpawnEvent(Vec2);

struct Velocity(Vec2);
struct ForceField {
    field: fn(Vec2) -> Vec2,
}

struct ParticleCounter(u32);

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn().insert(ForceField {
        field: |pos| -pos.distance_squared(Vec2::ZERO) / 1_000.0 * pos.signum(),
    });
}

fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.translation = position.translation
            + Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}

fn update_velocity(
    mut particles: Query<(&Transform, &mut Velocity)>,
    mut force_fields: Query<(&ForceField,)>,
    time: Res<Time>,
) {
    for (position, mut velocity) in particles.iter_mut() {
        for (force_field,) in force_fields.iter_mut() {
            let position = position.translation;
            let update =
                (force_field.field)(Vec2::new(position.x, position.y)) * time.delta_seconds();
            velocity.0 += update;
            println!("Updated by {}", update);
        }
    }
}

fn particle_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawner: EventReader<ParticleSpawnEvent>,
    mut counter: ResMut<ParticleCounter>,
) {
    for e in spawner.iter() {
        let texture_handle = asset_server.load("rolly_happy.png");
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture_handle.into()),
                transform: Transform {
                    translation: Vec3::new(e.0.x, e.0.y, 0.0),
                    scale: Vec3::splat(PARTICLE_SCALE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Velocity(Vec2::new(0.0, 0.0)));
        counter.0 += 1;
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
        .add_plugin(DiagnosticPlugin)
        .add_startup_system(setup.system())
        .insert_resource(ParticleCounter(0))
        .add_event::<ParticleSpawnEvent>()
        .add_system(mouse_handler.system())
        .add_system(particle_spawner.system())
        .add_system(update_positions.system())
        .add_system(update_velocity.system())
        .run();
}
