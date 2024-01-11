use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{prelude::*, window::WindowResolution, pbr::NotShadowCaster,};

use bevy_mod_picking::{*, highlight::Highlight, prelude::On, events::{Click, Pointer}};

mod bullet;
mod target;
mod tower;
mod monney;
mod main_menu;

pub use bullet::*;
pub use target::*;
pub use tower::*;
pub use monney::*;
pub use main_menu::*;

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
        .add_plugins(MonneyPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(MainMenuPlugin)
        .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnExit(GameState::MainMenu), asset_loading)
        .add_systems(OnEnter(GameState::Gameplay), spawn_basic_scene)
        .add_systems(Update, camera_controls.run_if(in_state(GameState::Gameplay)))
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

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();
    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 3.;

    if keyboard.pressed(KeyCode::Up) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Down) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Right) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Left) {
        camera.translation += left * time.delta_seconds() * speed;
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn(PbrBundle { // The Ground
        mesh: meshes.add(Mesh::from(shape::Plane { size: 25., ..Default::default() })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    })
    .insert(Name::new("Ground"));

    commands.spawn((
        SceneBundle {
            scene: game_assets.tower_base_scene.clone(),
            ..Default::default()
        },
        Name::new("Tower")
    ));

    commands.spawn(( // Target
        SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..Default::default()
        },
        Target { speed: 0.3, path_index:0  },
        Health { value: 3},
        Name::new("Target"),
    ));

    commands.spawn(( // Target
        SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-3.0, 0.2, 1.5),
            ..Default::default()
        },
        Target { speed: 0.3, path_index: 0 },
        Health { value: 3},
        Name::new("Target"),
    ));

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.8, 0.0)),
        meshes.add(shape::Capsule::default().into()),
        NotShadowCaster,
        Highlight {
            hovered: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            pressed: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color.clone())),
            selected: Some(bevy_mod_picking::prelude::HighlightKind::Fixed(selected_collider_color)),
        },
        default_collider_color,
        PickableBundle::default(),
        On::<Pointer<Click>>::run(build_tower),
        Name::new("Tower_Base")
    )).with_children(|commands| {
        commands.spawn(SceneBundle {
            scene: game_assets.tower_base_scene.clone(),
            transform: Transform::from_xyz(0.0, -0.8, 0.0),
            ..Default::default()
        });
    });
}

#[derive(Resource, Clone)]
pub struct GameAssets {
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
}

fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Gameplay
}