use bevy::{prelude::*, render::render_resource::Texture};
use diagnostic_plugin::DiagnosticPlugin;
use physics::{PhysicsPlugin, SimulationSpeed};

mod diagnostic_plugin;
mod physics;

enum ParticleType {
    NEUTRAL,
    NEGATIVE,
}

struct ParticleSpawnEvent {
    position: Vec2,
    particle_type: ParticleType,
}

struct ParticleCounter(u32);

struct Materials {
    particle_neutral: Handle<MyColorMaterial>,
    particle_negative: Handle<MyColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_server.load("rolly_happy.png");

    commands.insert_resource(Materials {
        particle_neutral: materials.add(texture.clone().into()),
        particle_negative: materials.add(ColorMaterial {
            color: Color::RED,
            texture: texture.clone().into(),
        }),
    });

    commands.spawn_bundle(Camera2dBundle::default());
}

fn mouse_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut spawner: EventWriter<ParticleSpawnEvent>,
) {
    let window = windows
        .get_primary()
        .expect("Primary window does not exist!");
    if mouse_button_input.pressed(MouseButton::Left) {
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

fn simulation_speed_handler(keys: Res<Input<KeyCode>>, mut simulation_speed: ResMut<SimulationSpeed>) {
    let speed = simulation_speed.0;
    if keys.just_pressed(KeyCode::Plus) || keys.just_pressed(KeyCode::NumpadAdd){
        simulation_speed.0 = speed * 2;
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::NumpadSubtract) {
        if speed > 1 {
            simulation_speed.0 = speed / 2;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
        .insert_resource(ParticleCounter(0))
        .add_event::<ParticleSpawnEvent>()
        .add_system(mouse_handler)
        .add_system(simulation_speed_handler)
        .run();
}
