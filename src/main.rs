use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use rand::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;

const SPRITE_WIDTH: f32 = 60.0;
const SPRITE_HEIGHT: f32 = 30.0;

#[derive(Resource)]
struct GameRng(StdRng);

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
    current_interval: f32,
    decrease_Rate: f32,
}

#[derive(Component)]
struct RotateToPlayer{
    rotation_speed: f32,
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
        .add_systems(FixedUpdate, (player_movement_system, rotate_to_player_system),)
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
        RotateToPlayer{
            rotation_speed: f32::to_radians(45.0),
        },
    ));
}

fn player_movement_system(
    time: Res<Time>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Single<(&Player, &mut Transform)>,
) { 
    let (ship, mut transform) = query.into_inner();
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;
    
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        movement_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        movement_factor -= 1.0;
    }

    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_secs());
    
    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = movement_factor * ship.movement_speed * time.delta_secs();
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;
    
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

fn rotate_to_player_system(
    time: Res<Time>,
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_transform: Single<&Transform, With<Player>>
) {
    let player_translation = player_transform.translation.xy();
    
    for(config, mut enemy_transform) in &mut query {
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
        let forward_dot_player = enemy_forward.dot(to_player);
        
        if (forward_dot_player - 1.0).abs() < f32::EPSILON  {
            continue;
        }

        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();
        let right_dot_player = enemy_right.dot(to_player);
        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0));
        let rotation_angle = rotation_sign * (config.rotation_speed * time.delta_secs()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);
    }
}

