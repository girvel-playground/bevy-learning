use bevy::log::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

#[derive(Component)]
struct Controlled;

struct CharacterSpriteFactory {
    style: TextStyle,
}

impl CharacterSpriteFactory {
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
   
    let sprite_factory = CharacterSpriteFactory::new(&asset_server);

    // player's "@"
    commands
        .spawn(Text2dBundle {
            text: sprite_factory.create(String::from("@")),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Controlled)
        .insert(Collider::cuboid(16., 16.));

    commands
        .spawn(Text2dBundle {
            text: sprite_factory.create(String::from("#")),
            transform: Transform::from_translation(Vec3::new(0., 100., 0.)),
            ..default()
        })
        .insert(Collider::cuboid(16., 16.));

    info!("Setup finished");
}

const MOVEMENT_SPEED: f32 = 150.;
const CONTROL_KEYS: &'static [(KeyCode, Vec2)] = &[
    (KeyCode::D, Vec2::X),
    (KeyCode::A, Vec2 { x: -1., y: 0. }),
    (KeyCode::W, Vec2::Y),
    (KeyCode::S, Vec2 { x: 0., y: -1. }),
];

fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Controlled>>,
) {
    for mut velocity in &mut query {
        velocity.linvel = Vec2::ZERO;
        for &(key, axis) in CONTROL_KEYS {
            if input.pressed(key) {
                velocity.linvel += MOVEMENT_SPEED * axis;
            }
        }
    }
}
