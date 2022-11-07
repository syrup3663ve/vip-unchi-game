use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use rand::prelude::*;

const GRASS_FONT_SIZE: f32 = 12.;

mod catcher;
mod player;
mod rule;

fn main() {
    let mut app = App::new();

    if !cfg!(debug_assertions) {
        app.insert_resource(LogSettings {
            filter: "error".into(),
            level: Level::ERROR,
        });
    }

    app.insert_resource(WindowDescriptor {
        width: 480.,
        height: 480.,
        title: "Vip-Unchi".to_string(),
        resizable: false,
        canvas: Some("canvas".to_string()),
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(catcher::CatcherPlugin)
    .add_plugin(rule::RulePlugin)
    .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window = windows.get_primary().unwrap();
    let font = server.load(rule::FONT_PATH);
    let mut rng = thread_rng();
    let mut grasses = vec![];
    for y in ((-window.width() as i32 / 2)..=(window.width() as i32 / 2)).step_by(15) {
        for x in ((-window.height() as i32 / 2)..=(window.height() as i32 / 2)).step_by(15) {
            let r = rng.gen_range(0.0..=0.2);
            let g = rng.gen_range(0.5..=1.);
            let b = rng.gen_range(0.0..=0.2);
            let grass = Text2dBundle {
                text: Text::from_section(
                    "w",
                    TextStyle {
                        font: font.clone(),
                        font_size: GRASS_FONT_SIZE,
                        color: Color::rgb(r, g, b),
                    },
                ),
                transform: Transform::from_xyz(x as f32, y as f32, 0.),
                ..default()
            };
            grasses.push(grass);
        }
    }

    commands.spawn_batch(grasses);
}
