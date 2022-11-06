use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 480.,
        height: 480.,
        title: "Vip-Unchi".to_string(),
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .run();
}
