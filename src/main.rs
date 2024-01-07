use std::f32::consts::PI;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(1280., 720.),
                title: "Bevy Tower Defense".to_owned(),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .add_systems(Startup, (
            spawn_camera,
            spawn_basic_scene,
            asset_loading
        ))
        .add_systems(Update, (
            tower_shooting,
            bullet_dispawn
        ))
        .run();
}

#[derive(Reflect, Component, Default)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
pub struct Lifetime {
    timer: Timer,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    }).insert(Name::new("Light"));
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle { // The Ground
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5., ..Default::default() })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    })
    .insert(Name::new("Ground"));

    commands.spawn(PbrBundle { // A Tower
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1., ..Default::default() })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..Default::default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(1., TimerMode::Repeating),
    })
    .insert(Name::new("Tower"));
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<&mut Tower>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));
            
            commands.spawn((
                SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..Default::default()
                },
                Lifetime { timer: Timer::from_seconds(0.5, TimerMode::Once)},
                Name::new("Bullet")
            ));
        }
    }
}

fn bullet_dispawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Resource, Clone)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}