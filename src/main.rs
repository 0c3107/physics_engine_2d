use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

const CIRCLE_RADIUS: f32 = 32.;
const CIRCLE_SPEED_MULTIPLIER: f32 = 32.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_circle)
        .add_system(circle_movement.after(confine_circle_movement))
        .add_system(confine_circle_movement)
        .add_system(gravity)
        .add_system(reset_acceleration_on_wall_hit.before(confine_circle_movement))
        .run();
}

#[derive(Component)]
struct Circle {}

#[derive(Component)]
struct Acceleration {
    vertical: f32,
    horizontal: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(CIRCLE_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Circle {},
        Acceleration {
            vertical: 0.,
            horizontal: 0.,
        },
    ));
}

fn circle_movement(mut circle_query: Query<(&mut Transform, &Acceleration)>, time: Res<Time>) {
    for (mut transform, acceleration) in circle_query.iter_mut() {
        let direction = Vec3::new(
            acceleration.horizontal * CIRCLE_SPEED_MULTIPLIER * time.delta_seconds(),
            acceleration.vertical * CIRCLE_SPEED_MULTIPLIER * time.delta_seconds(),
            0.,
        );
        transform.translation += direction;
    }
}

fn gravity(mut circle_query: Query<&mut Acceleration>, time: Res<Time>) {
    for mut acceleration in circle_query.iter_mut() {
        acceleration.vertical -= 9.81 * time.delta_seconds();
    }
}

fn confine_circle_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut circle_query: Query<&mut Transform, With<Circle>>,
) {
    for mut circle_transform in circle_query.iter_mut() {
        let window = window_query.get_single().unwrap();
        let x_min = window.width() / -2. + CIRCLE_RADIUS;
        let x_max = window.width() / 2. - CIRCLE_RADIUS;
        let y_min = window.height() / -2. + CIRCLE_RADIUS;
        let y_max = window.height() / 2. - CIRCLE_RADIUS;

        let mut translation = circle_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        };

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        };
        circle_transform.translation = translation;
    }
}

fn reset_acceleration_on_wall_hit(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut circle_query: Query<(&mut Acceleration, &Transform), With<Circle>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = window.width() / -2. + CIRCLE_RADIUS;
    let x_max = window.width() / 2. - CIRCLE_RADIUS;
    let y_min = window.height() / -2. + CIRCLE_RADIUS;
    let y_max = window.height() / 2. - CIRCLE_RADIUS;

    for (mut accel, transform) in circle_query.iter_mut() {
        if transform.translation.x < x_min || transform.translation.x > x_max {
            accel.horizontal = 0.;
        };

        if transform.translation.y < y_min || transform.translation.y > y_max {
            accel.vertical = 0.;
        };
    }
}
