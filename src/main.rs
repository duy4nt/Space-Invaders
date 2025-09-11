use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use rand::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;

const SPRITE_WIDTH: f32 = 60.0;
const SPRITE_HEIGHT: f32 = 30.0;

#[derive(Resources)]
struct GameRng(StdRng);

struct Player{
    name: String,
    health: f32,
}

#[derive(Resources)]
struct SpawnTimer {
    timer: Timer,
    current_interval: f32,
    decrease_Rate: f32,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: "Space Invaders".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_movement_system,),)
        .run();
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut score: u16 = 0;

    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(format!("Score: {}", score)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }
    ));

    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    commands.spawn((Sprite::from_image(
        asset_server.load("player.png")),
        Transform::from_xyz(0.0,-( WINDOW_HEIGHT/2.0) + SPRITE_HEIGHT, 1.0),
        Player {
            movement_speed: 300.0,
            rotation_speed: f32::to_radians(360.0)
        },
    ));

    commands.spawn((Sprite::from_image(
        asset_server.load("yellow.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn player_movement_system(
    time: Res<Time>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform,&Player), With<Player>>,
) {
    for (mut player_transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        } 
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }
    
        player_transform.translation += direction * player.movement_speed * time.delta_secs();
    }
}
