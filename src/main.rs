use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::prelude::SpatialBundle,
    sprite::MaterialMesh2dBundle,
};

use bevy_tweening::{lens::*, *};
use std::time::Duration;

#[derive(Component)]
struct MyGameCamera;

#[derive(Component)]
struct Parent;

pub const CAMERA_MOVE_TIME: u64 = 100;

pub struct TransformProjectionLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<OrthographicProjection> for TransformProjectionLens {
    fn lerp(&mut self, target: &mut OrthographicProjection, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.scale = value;
        println!("tweening! {}", value)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, zoom_2d)
        .add_systems(Update, update_parent)
        .add_systems(Update, component_animator_system::<OrthographicProjection>)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        MyGameCamera,
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    let parent = commands
        .spawn((
            SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(Vec3::new(0., -500.0, 0.)),
                ..default()
            },
            Parent,
        ))
        .id();

    let space = 50.0;
    for i in 0..1000 {
        // Circle mesh
        let c1 = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                // 4. Put something bright in a dark environment to see the effect
                material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
                transform: Transform::from_translation(Vec3::new(-500., space * (i as f32), 0.)),
                ..default()
            })
            .id();

        // Hexagon mesh
        let c2 = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 5).into()).into(),
                // 4. Put something bright in a dark environment to see the effect
                material: materials.add(ColorMaterial::from(Color::rgb(6.25, 9.4, 9.1))),
                transform: Transform::from_translation(Vec3::new(500., space * (i as f32), 0.)),
                ..default()
            })
            .id();

        commands.entity(parent).push_children(&[c1, c2]);
    }

    // UI
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn update_parent(
    mut parent_position: Query<(Entity, &mut Transform), With<Parent>>,
    keycode: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (id, mut transform) in &mut parent_position {
        transform.translation.y -= 1.0;

        if keycode.pressed(KeyCode::Y) {
            transform.translation.y += 9.0;
        }
        if keycode.pressed(KeyCode::H) {
            transform.translation.y -= 8.0;
        }

        if keycode.just_pressed(KeyCode::P) {
            println!("passed here");
            let tween = Tween::new(
                // Use a quadratic easing on both endpoints.
                EaseFunction::QuadraticInOut,
                // Animation time (one way only; for ping-pong it takes 2 seconds
                // to come back to start).
                Duration::from_secs(5),
                // The lens gives the Animator access to the Transform component,
                // to animate it. It also contains the start and end values associated
                // with the animation ratios 0. and 1.
                TransformPositionLens {
                    start: transform.translation,
                    end: Vec3::new(0., transform.translation.y - 2000., 0.),
                },
            );

            commands.entity(id).insert(Animator::new(tween));
        }
    }
}

fn zoom_2d(
    mut q: Query<(Entity, &mut OrthographicProjection), With<MyGameCamera>>,
    keycode: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let (id, mut projection) = q.single_mut();

    if keycode.just_pressed(KeyCode::Key1) {
        projection.scale = 0.9;
        println!("camera scale = {}", projection.scale);
    }
    if keycode.just_pressed(KeyCode::Key2) {
        projection.scale = 1.4;
        println!("camera scale = {}", projection.scale);
    }
    if keycode.just_pressed(KeyCode::Key3) {
        projection.scale = 1.3;
        println!("camera scale = {}", projection.scale);
    }
    if keycode.just_pressed(KeyCode::Key4) {
        let ttt = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(CAMERA_MOVE_TIME),
            TransformProjectionLens {
                start: projection.scale,
                end: 0.9,
            },
        );

        commands.entity(id).insert(Animator::new(ttt));
        println!("camera scale = {}", projection.scale);
    }
    if keycode.just_pressed(KeyCode::Key5) {
        let ttt = Tween::new(
            EaseFunction::ExponentialInOut,
            Duration::from_millis(CAMERA_MOVE_TIME),
            TransformProjectionLens {
                start: projection.scale,
                end: 1.1,
            },
        );

        commands.entity(id).insert(Animator::new(ttt));
        println!("entity id = {}", id.index());
        println!("camera scale = {}", projection.scale);
    }
    if keycode.just_pressed(KeyCode::A) {
        projection.scale = 1.0;
    }
    if keycode.pressed(KeyCode::X) {
        projection.scale *= 0.9;
    }
    if keycode.pressed(KeyCode::Z) {
        projection.scale *= 1.1;
    }

    //println!("camera scale = {}", projection.scale);
}

//fn move_camera(mut camera: Query<&mut Transform, With<MyGameCamera>>) {}
