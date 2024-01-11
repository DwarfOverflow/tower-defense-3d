use bevy::prelude::*;

use crate::{Monney, TARGET_DEATH_MONNEY, GameState};

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Target>()
            .register_type::<Health>()
            .add_systems(Update, (
                move_targets,
                target_death,
                hurt_player
            ).run_if(in_state(GameState::Gameplay)))
            .insert_resource(TargetPath {
                waypoints: vec![
                    Vec2::new(6.0, 2.0),
                    Vec2::new(6.0, 6.0),
                    Vec2::new(9.0, 9.0),
                ]
            });
    }
}

#[derive(Component, Reflect, Default)]
pub struct Target {
    pub speed: f32,
    pub path_index: usize,
}

#[derive(Resource)]
pub struct TargetPath {
    waypoints: Vec<Vec2>,
}


#[derive(Component, Reflect, Default)]
pub struct Health {
    pub value: i32,
}

fn move_targets(
    mut targets: Query<(&mut Target, &mut Transform)>,
    path: Res<TargetPath>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in &mut targets {
        if target.path_index == path.waypoints.len() { return; }

        let delta = target.speed * time.delta_seconds();
        let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

        // Se rapprocher de la target
        if delta_target.length() > delta {
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();

            let y = transform.translation.y;
            transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y)
        } else {
            target.path_index += 1;
        }
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut monney: ResMut<Monney>,
) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
            monney.value += TARGET_DEATH_MONNEY;
        }
    }
}

fn hurt_player(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
    path: Res<TargetPath>,
) {
    for (entity, target) in &targets {
        if target.path_index >= path.waypoints.len() {
            commands.entity(entity).despawn_recursive();
        }
    }
}