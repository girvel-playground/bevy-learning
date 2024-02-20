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

struct TextSpriteFactory {
    style: TextStyle,
}

impl TextSpriteFactory {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            style: TextStyle {
                font: asset_server.load("fonts/Classic Console Neue.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            }
        }
    }

    fn create(&self, string: String) -> Text {
        Text {
            sections: vec![TextSection::new(string, self.style.clone())],
            ..Default::default()
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setup started");

    // 2d camera
    commands.spawn(Camera2dBundle::default());
   
    let sprite_factory = TextSpriteFactory::new(&asset_server);

    // player's "@"
    commands.spawn((Text2dBundle {
        text: sprite_factory.create(String::from("@")),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    }, Controlled));

    commands.spawn(Text2dBundle {
        text: sprite_factory.create(String::from("#")),
        transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
        ..default()
    });

    info!("Setup finished");
}

const MOVEMENT_SPEED: f32 = 150.;
const CONTROL_KEYS: &'static [(KeyCode, Vec3)] = &[
    (KeyCode::D, Vec3::X),
    (KeyCode::A, Vec3 { x: -1., y: 0., z: 0. }),
    (KeyCode::W, Vec3::Y),
    (KeyCode::S, Vec3 { x: 0., y: -1., z: 0. }),
];

fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Controlled>>,
) {
    for mut transform in &mut query {
        for &(key, axis) in CONTROL_KEYS {
            if input.pressed(key) {
                transform.translation += MOVEMENT_SPEED * time.delta_seconds() * axis;
            }
        }
    }
}
