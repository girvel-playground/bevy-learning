use bevy::log::prelude::*;
use bevy::{
    prelude::*,
    sprite::Anchor,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

#[derive(Component)]
struct Controlled;

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
    }, Controlled));
}

const MOVEMENT_SPEED: f32 = 150.;

fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Controlled>>,
) {
    for mut transform in &mut query {
        if input.pressed(KeyCode::W) {
            transform.translation += MOVEMENT_SPEED * time.delta_seconds() * Vec3::Y;
        }
    }
}
