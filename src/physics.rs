use bevy::prelude::*;

use crate::{Materials, ParticleCounter, ParticleSpawnEvent, ParticleType};

const PARTICLE_SCALE: f32 = 4.0;
const SIMULATION_LOOPS: u32 = 1;
pub struct PhysicsPlugin;

struct Velocity(Vec2);
struct ForceField {
    field: fn(Vec2, Vec2) -> Vec2,
}

struct PhysicsStage(SystemStage);

pub struct SimulationSpeed(pub u32);

impl Stage for PhysicsStage {
    fn run(&mut self, world: &mut World) {
        let loops = world.get_resource::<SimulationSpeed>().unwrap().0;
        for _ in 0..loops {
            self.0.run(world);
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(particle_spawner.system())
            .insert_resource(SimulationSpeed(1))
            .add_stage_after(
                CoreStage::Update,
                "physics",
                PhysicsStage(
                    SystemStage::parallel()
                        .with_system(update_positions.system())
                        .with_system(update_velocity.system()),
                ),
            );
    }
}

fn setup(mut commands: Commands) {
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
    materials: Res<Materials>,
    mut spawner: EventReader<ParticleSpawnEvent>,
    mut counter: ResMut<ParticleCounter>,
) {
    for e in spawner.iter() {
        let mut new_particle = commands.spawn();
        let particle_material;

        match e.particle_type {
            ParticleType::NEUTRAL => particle_material = materials.particle_neutral.clone_weak(),
            ParticleType::NEGATIVE => {
                particle_material = materials.particle_negative.clone_weak();
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
            material: particle_material,
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
