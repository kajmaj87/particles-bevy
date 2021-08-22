use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::ParticleCounter;

pub struct DiagnosticPlugin;

const FONT_SIZE: f32 = 20.0;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup.system())
            .add_system(fps_counter.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Particle Count: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::rgb(0.0, 1.0, 0.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::rgb(0.0, 1.0, 1.0),
                    },
                },
                TextSection {
                    value: "\nAverage FPS: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::rgb(0.0, 1.0, 0.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::rgb(0.0, 1.0, 1.0),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn fps_counter(
    diagnostics: Res<Diagnostics>,
    counter: Res<ParticleCounter>,
    mut query: Query<&mut Text>,
) {
    println!("Inside counter");
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        println!("Inside frame");
        if let Some(average) = fps.average() {
            for mut text in query.iter_mut() {
                text.sections[1].value = format!("{}", counter.0);
                text.sections[3].value = format!("{:.2}", average);
                println!("Setting new texts for {} and {}", counter.0, average);
            }
        }
    };
}
