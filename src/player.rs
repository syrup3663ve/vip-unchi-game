use bevy::prelude::*;

const SPEED: f32 = 500.;
const PLAYER_SIZE: [f32; 2] = [72., 72.];
const PLAYER_IMAGE_PATH: &str = "unko-spawner.png";
const UNKO_IMAGE_PATH: &str = "unko.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(move_player)
            .add_system(fire_unko)
            .add_system(move_unko);
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(mut commands: Commands, server: Res<AssetServer>) {
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(PLAYER_SIZE.into()),
            ..default()
        },
        texture: server.load(PLAYER_IMAGE_PATH),
        ..default()
    };
    commands
        .spawn_bundle(sprite_bundle)
        .insert(Player)
        .insert(UnkoSpawner {
            timer: Timer::from_seconds(0.5, false),
        });
}

fn move_player(
    time: Res<Time>,
    inputs: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let mut dir = Vec3::ZERO;
    if inputs.pressed(KeyCode::A) {
        dir.x -= 1.;
    }
    if inputs.pressed(KeyCode::D) {
        dir.x += 1.;
    }
    if inputs.pressed(KeyCode::W) {
        dir.y += 1.;
    }
    if inputs.pressed(KeyCode::S) {
        dir.y -= 1.;
    }

    let mut transform = players.get_single_mut().unwrap();
    transform.translation += dir * time.delta_seconds() * SPEED;

    let window = windows.get_primary().unwrap();
    let h_width = window.width() / 2.;
    let h_height = window.height() / 2.;
    transform.translation = transform.translation.clamp(
        Vec3::new(-h_width, -h_height, 0.),
        Vec3::new(h_width, -h_height + 100., 0.),
    );
}

#[derive(Component)]
pub struct UnkoSpawner {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Unko;

fn fire_unko(
    mut commands: Commands,
    time: Res<Time>,
    inputs: Res<Input<KeyCode>>,
    server: Res<AssetServer>,
    mut spawners: Query<(&mut UnkoSpawner, &Transform)>,
) {
    for (mut spawner, trans) in spawners.iter_mut() {
        spawner.timer.tick(time.delta());

        if spawner.timer.finished() && inputs.pressed(KeyCode::LShift) {
            let sprite_bundle = SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(48.)),
                    ..default()
                },
                texture: server.load(UNKO_IMAGE_PATH),
                transform: *trans,
                ..default()
            };
            commands.spawn_bundle(sprite_bundle).insert(Unko);

            spawner.timer.reset();
        }
    }
}

fn move_unko(
    mut commands: Commands,
    time: Res<Time>,
    mut unkos: Query<(Entity, &mut Transform), With<Unko>>,
) {
    for (e, mut transform) in unkos.iter_mut() {
        transform.translation += Vec3::Y * time.delta_seconds() * SPEED;

        if is_outside(&transform.translation) {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn is_outside(translation: &Vec3) -> bool {
    let area = -1000f32..1000f32;
    !area.contains(&translation.x) || !area.contains(&translation.y)
}
