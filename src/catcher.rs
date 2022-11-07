use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use rand::prelude::*;

use crate::player::Unko;

const CATCHER_IMAGE_PATH: &str = "unko-man.png";
const CATCHER_SIZE: [f32; 2] = [64., 64.];
const CATCHER_NUMS: usize = 5;

pub struct CatcherPlugin;

impl Plugin for CatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_catchers)
            .add_system(move_catchers)
            .add_system(catch_unko);
    }
}

fn setup_catchers(windows: Res<Windows>, mut commands: Commands, server: Res<AssetServer>) {
    let window = windows.get_primary().unwrap();

    let mut rng = thread_rng();
    let main_catcher_idx = rng.gen_range(0..CATCHER_NUMS - 1);

    for (idx, i) in (0..CATCHER_NUMS).enumerate() {
        let color = match idx == main_catcher_idx {
            true => Color::rgba(1., 0.5, 0.5, 0.75),
            false => Color::default(),
        };
        let sprite = Sprite {
            color,
            custom_size: Some(CATCHER_SIZE.into()),
            ..default()
        };
        let range_x = (-window.width() / 2.0)..=window.width() / 2.0;
        let x = rng.gen_range(range_x);
        let y = (window.height() / 2.) - i as f32 * 50. - 50.;
        let sprite_bundle = SpriteBundle {
            sprite,
            texture: server.load(CATCHER_IMAGE_PATH),
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        };

        let speed = [-500, -400, -300, 300, 400, 500]
            .choose(&mut rng)
            .unwrap_or(&300);
        let e = commands
            .spawn_bundle(sprite_bundle)
            .insert(Catcher::new(*speed as f32))
            .id();

        if idx == main_catcher_idx {
            commands.entity(e).insert(MainCatcher);
        }
    }
}

#[derive(Component)]
pub struct MainCatcher;

#[derive(Component)]
pub struct Catcher {
    pub speed: f32,
    pub count: usize,
}

impl Catcher {
    pub fn new(speed: f32) -> Self {
        Self { speed, count: 0 }
    }
}

fn move_catchers(
    windows: Res<Windows>,
    time: Res<Time>,
    mut catchers: Query<(&mut Transform, &mut Catcher)>,
) {
    let window = windows.get_primary().unwrap();

    for (mut transform, mut catcher) in catchers.iter_mut() {
        let h_width = window.width() / 2.0;
        let x = (transform.translation.x + catcher.speed * time.delta_seconds())
            .clamp(-h_width, h_width);
        transform.translation.x = x;

        if transform.translation.x <= -h_width || h_width <= transform.translation.x {
            catcher.speed *= -1.;
        }
    }
}

fn catch_unko(
    mut commands: Commands,
    mut catchers: Query<(&Transform, &mut Catcher, &Sprite)>,
    unkos: Query<(Entity, &Transform, &Sprite), With<Unko>>,
) {
    for (c_trans, mut catcher, c_spr) in catchers.iter_mut() {
        for (u_e, u_trans, u_spr) in unkos.iter() {
            if let Some(collision) = collide(
                c_trans.translation,
                c_spr.custom_size.unwrap_or_default(),
                u_trans.translation,
                u_spr.custom_size.unwrap_or_default(),
            ) {
                match collision {
                    Collision::Inside => {}
                    _ => {
                        catcher.count += 1;
                        commands.entity(u_e).despawn_recursive();
                    }
                }
            }
        }
    }
}
