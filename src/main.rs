use bevy::prelude::*;
use diagnostic_plugin::DiagnosticPlugin;

mod diagnostic_plugin;

const PARTICLE_SCALE: f32 = 4.0;

enum ParticleType {
    NEUTRAL,
    NEGATIVE,
}

struct ParticleSpawnEvent {
    position: Vec2,
    particle_type: ParticleType,
}

struct Velocity(Vec2);
struct ForceField {
    field: fn(Vec2, Vec2) -> Vec2,
}

struct ParticleCounter(u32);

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn()
        .insert(ForceField {
            field: |pos, center| -pos.distance_squared(center) / 1_000.0 * (pos - center).signum(),
        })
        .insert(Transform::default());
}

fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.translation = position.translation
            + Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}

fn update_velocity(
    mut particles: Query<(Entity, &Transform, &mut Velocity)>,
    mut force_fields: Query<(Entity, &Transform, &ForceField)>,
    time: Res<Time>,
) {
    for (particle_ent, position, mut velocity) in particles.iter_mut() {
        for (force_ent, force_center, force_field) in force_fields.iter_mut() {
            // particle can also be a source of Force so it should not act on itself
            if particle_ent != force_ent {
                let position = Vec2::from(position.translation);
                let center = Vec2::from(force_center.translation);
                let update = (force_field.field)(position, center) * time.delta_seconds();
                velocity.0 += update;
            }
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
    let texture_handle = asset_server.load("rolly_happy.png");
    for e in spawner.iter() {
        let mut new_particle = commands.spawn();
        let particle_material;

        match e.particle_type {
            ParticleType::NEUTRAL => {
                particle_material = texture_handle.clone_weak().into();
            }
            ParticleType::NEGATIVE => {
                particle_material = ColorMaterial {
                    color: Color::RED,
                    texture: texture_handle.clone_weak().into(),
                };
                new_particle.insert(ForceField {
                    field: |pos, center| {
                        1.0 / (1.0 + pos.distance_squared(center))
                            * 1_000_000.0
                            * (pos - center).signum()
                    },
                });
            }
        }

        new_particle.insert_bundle(SpriteBundle {
            material: materials.add(particle_material),
            transform: Transform {
                translation: Vec3::new(e.position.x, e.position.y, 0.0),
                scale: Vec3::splat(PARTICLE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        });
        new_particle.insert(Velocity(Vec2::ZERO));
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
        println!("click left at {:?}", window.cursor_position());
        if let Some(click_position) = window.cursor_position() {
            let translation = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            spawner.send(ParticleSpawnEvent {
                position: click_position - translation,
                particle_type: ParticleType::NEUTRAL,
            });
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        println!("click right at {:?}", window.cursor_position());
        if let Some(click_position) = window.cursor_position() {
            let translation = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            spawner.send(ParticleSpawnEvent {
                position: click_position - translation,
                particle_type: ParticleType::NEGATIVE,
            });
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
