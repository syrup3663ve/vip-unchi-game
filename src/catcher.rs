use bevy::prelude::*;
use rand::prelude::*;

const CATCHER_NUMS: usize = 5;

pub struct CatcherPlugin;

impl Plugin for CatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_catchers)
            .add_system(move_catchers);
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
            custom_size: Some(Vec2::splat(64.)),
            ..default()
        };
        let range_x = (-window.width() / 2.0)..=window.width() / 2.0;
        let x = rng.gen_range(range_x);
        let y = (window.height() / 2.) - i as f32 * 50. - 50.;
        let sprite_bundle = SpriteBundle {
            sprite,
            texture: server.load("unko-man.png"),
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        };

        let speed = [-500, -400, -300, 300, 400, 500].choose(&mut rng).unwrap();
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
    let h_width = windows.get_primary().unwrap().width() / 2.;
    for (mut transform, mut catcher) in catchers.iter_mut() {
        if transform.translation.x < -h_width || h_width < transform.translation.x {
            catcher.speed *= -1.;
        }

        transform.translation += Vec3::X * catcher.speed * time.delta_seconds();
    }
}