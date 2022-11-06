use bevy::prelude::*;

use crate::catcher::{Catcher, MainCatcher};

const FONT_PATH: &str = "tiza.ttf";
const FONT_SIZE: f32 = 32.;

pub struct RulePlugin;

impl Plugin for RulePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(10., false)))
            .add_startup_system(setup_ui)
            .add_system(advance_game_timer)
            .add_system(update_game_count_text);
    }
}

pub struct GameTimer(pub Timer);

fn advance_game_timer(
    time: Res<Time>,
    main_catcher: Query<&Catcher, With<MainCatcher>>,
    mut game_timer: ResMut<GameTimer>,
    mut game_over_texts: Query<(&mut Visibility, &mut Text), With<GameOverText>>,
) {
    if game_timer.0.tick(time.delta()).just_finished() {
        for (mut visibility, mut text) in game_over_texts.iter_mut() {
            visibility.is_visible = true;

            if let Ok(main_catcher) = main_catcher.get_single() {
                let result_text = match main_catcher.count {
                    0 => "No Unko".to_string(),
                    1 => "1 Unko".to_string(),
                    count => format!("{count} Unkos"),
                };
                text.sections[0]
                    .value
                    .push_str(&format!("\n{}", result_text));
            }
        }
    }
}

#[derive(Component)]
pub struct GameTimerText;

#[derive(Component)]
pub struct GameOverText;

fn setup_ui(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Px(50.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|p| {
            let text_bundle = TextBundle::from_section(
                "0".to_string(),
                TextStyle {
                    font: server.load(FONT_PATH),
                    font_size: FONT_SIZE,
                    color: Color::BLACK,
                },
            );
            p.spawn_bundle(text_bundle).insert(GameTimerText);
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|p| {
            let font = server.load(FONT_PATH);
            let mut text_bundle = TextBundle::from_section(
                "GAME OVER\n".to_string(),
                TextStyle {
                    font,
                    font_size: FONT_SIZE,
                    color: Color::BLACK,
                },
            );
            text_bundle.visibility.is_visible = false;
            p.spawn_bundle(text_bundle).insert(GameOverText);
        });
}

fn update_game_count_text(
    game_timer: Res<GameTimer>,
    mut game_timer_texts: Query<&mut Text, With<GameTimerText>>,
) {
    for mut text in game_timer_texts.iter_mut() {
        text.sections[0].value = (game_timer.0.duration() - game_timer.0.elapsed())
            .as_secs()
            .to_string();
    }
}
