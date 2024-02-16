use bevy::{
    prelude::*,
    sprite::Anchor,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, stupid_falling)
        .run();
}

#[derive(Component)]
struct StupidFalling;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    // 2d camera
    commands.spawn(Camera2dBundle::default());
   
    // player's "@"
    let only_style = TextStyle {
        font,
        font_size: 24.0,
        color: Color::WHITE,
    };

    commands.spawn((Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(
                String::from("@"),
                only_style,
            )],
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::ZERO),
        text_anchor: Anchor::Center,
        ..default()
    }, StupidFalling));
}

fn stupid_falling(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<StupidFalling>>,
) {
    for mut transform in &mut query {
        transform.translation.y -= 50. * time.delta_seconds();
    }
}

