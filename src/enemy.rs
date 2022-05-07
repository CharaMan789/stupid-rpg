use bevy::prelude::*;

#[allow(unused)]

use crate::{collision::enemy_collision, wall::spawn_walls, TILE_SIZE, player::Player};

const _ENEMY_SPEED: f32 = 0.04;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;


impl Plugin for EnemyPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system(enemy_collision);
    }
}


