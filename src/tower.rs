use bevy::{prelude::*, utils::FloatOrd};
use bevy_mod_picking::{prelude::{Listener, EntityEvent}, events::{Click, Pointer}};
use crate::{GameAssets, Target, Lifetime, Bullet};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Tower>()
            .add_systems(Update, tower_shooting);
    }
}

#[derive(Reflect, Component, Default)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    game_assets: Res<GameAssets>,
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
                            scene: game_assets.tomato_scene.clone(),
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

pub fn build_tower(
    event: Listener<Pointer<Click>>,
    entities: Query<(Entity, &Transform)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.entity(event.target()).despawn_recursive();
    for entity in &entities {
        if entity.0 == event.target {
            spawn_tomato_tower(commands, &assets, entity.1.translation.clone());
            return;
        }
    }
}

fn spawn_tomato_tower(mut commands: Commands, assets: &GameAssets, position: Vec3) -> Entity {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(
            position
        )),
        Name::new("Tomato_Tower"),
        Tower {
            shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.6, 0.0)
        }
    )).with_children(|commands| {
        commands.spawn(SceneBundle {
            scene: assets.tomato_tower_scene.clone(),
            transform: Transform::from_xyz(0.0, -0.8, 0.0),
            ..Default::default()
        });
    }).id()
}