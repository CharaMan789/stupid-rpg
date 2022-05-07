use crate::{collision::npc_collision, TILE_SIZE};
use bevy::prelude::*;

pub struct NpcPlugin;

#[derive(Component)]
pub struct Npc;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_npc)
            .add_system(npc_collision);
    }
}

fn spawn_npc(mut commands: Commands, assets: Res<AssetServer>){
    commands.spawn_bundle(SpriteBundle{
        texture: assets.load("npc.png"),
        transform: Transform{
            translation: Vec3::new(1.0, 0.0, 900.0),
            scale: Vec3::splat(TILE_SIZE/7 as f32),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Npc);
}


