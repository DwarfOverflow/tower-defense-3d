use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::{prelude::*, window::WindowResolution, utils::FloatOrd};

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
        .register_type::<Target>()
        .register_type::<Lifetime>()
        .add_systems(Startup, (
            spawn_camera,
            spawn_basic_scene,
            asset_loading
        ))
        .add_systems(Update, (
            tower_shooting,
            bullet_dispawn,
            move_targets,
            move_bullets,
            target_death,
            bullet_collision
        ))
        .run();
}

#[derive(Reflect, Component, Default)]
pub struct Tower {
    pub shooting_timer: Timer,
    bullet_offset: Vec3,
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

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                commands.entity(tower_ent).with_children(|commands| {
                    commands.spawn((
                        SceneBundle {
                            scene: bullet_assets.bullet_scene.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..Default::default()
                        },
                        Lifetime { timer: Timer::from_seconds(0.5, TimerMode::Once)},
                        Bullet {
                            direction,
                            speed: 2.5
                        },
                        Name::new("Bullet")
                    ));
                });
            }
        }
    }
}

#[derive(Component, Reflect, Default)]
pub struct Bullet {
    direction: Vec3,
    speed: f32,
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

fn move_bullets(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>
) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
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

#[derive(Component, Reflect, Default)]
pub struct Target {
    speed: f32,
}

#[derive(Component, Reflect, Default)]
pub struct Health {
    value: i32,
}

fn move_targets(
    mut targets: Query<(&Target, &mut Transform)>,
    time: Res<Time>,
) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>
) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    for (bullet, bullet_transform) in &bullets {
        for (mut health, target_transform) in &mut targets {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}