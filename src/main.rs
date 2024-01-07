use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{prelude::*, window::WindowResolution};

mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

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
        .add_plugins(TowerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(TargetPlugin)
        .add_systems(Startup, (
            spawn_camera,
            spawn_basic_scene,
            asset_loading
        ))
        .run();
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
        bullet_offset: Vec3::new(0.0, 0.2, 0.5)
    })
    .insert(Name::new("Tower"));

    commands.spawn(( // Target
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.4, ..Default::default()})),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..Default::default()
        },
        Target { speed: 0.3 },
        Health { value: 3},
        Name::new("Target"),
    ));

    commands.spawn(( // Target
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.4, ..Default::default()})),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-3.0, 0.2, 1.5),
            ..Default::default()
        },
        Target { speed: 0.3 },
        Health { value: 3},
        Name::new("Target"),
    ));
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