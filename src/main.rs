use bevy::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_movement_system,),)
        .run();
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
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
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            movement_speed: 300.0,
        },
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
