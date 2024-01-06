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
        .add_systems(Startup, (
            spawn_camera,
            spawn_basic_scene
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

    commands.spawn(PbrBundle { // A Cube
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1., ..Default::default() })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..Default::default()
    })
    .insert(Name::new("Cube"));
}