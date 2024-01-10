use bevy::prelude::*;

pub const BEGIN_MONNEY: i64 = 150;
pub const TARGET_DEATH_MONNEY: i64 = 25;
pub const TOWER_COST: i64 = 100;

pub struct MonneyPlugin;
impl Plugin for MonneyPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Monney>()
            .insert_resource(Monney { value: BEGIN_MONNEY });

    }
}

#[derive(Resource, Reflect)]
pub struct Monney {
    pub value: i64,
}