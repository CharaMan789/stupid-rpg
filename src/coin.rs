use crate::{collision::obj_collision, animation::Animations, TILE_SIZE, player::Player};
use bevy::prelude::*;
use benimator::*;

pub struct CoinPlugin;

#[derive(Component)]
pub struct Coin;


impl Plugin for CoinPlugin{
    fn build(&self, app: &mut App){
	app.add_system(obj_collision)
	    .add_system_to_stage(CoreStage::PostUpdate, no_bodies);
    }
}

fn no_bodies
    (
	query: Query<(), With<Coin>>,
	removed: RemovedComponents<Coin>,
	mut commands: Commands,
	animations: Res<Animations>,
	assets: Res<AssetServer>,
	mut texture: ResMut<Assets<TextureAtlas>>,
	mut player_query: Query<(Entity, &Transform), With<Player>>,
 )
{
    for (_player, player_tf) in player_query.iter(){
	if removed.iter().next().is_some() {
            if query.is_empty() {
		commands.spawn_bundle(SpriteSheetBundle
				  {
				      texture_atlas: texture.add(TextureAtlas::from_grid(
					  assets.load("you-win-sheet.png"),
					  Vec2::new(16.0, 16.0),
					  2,
					  2,
				      )),
				      transform: Transform
				      {
					  translation: player_tf.translation,
					  scale: Vec3::splat(0.20),
					  ..Default::default()
				      },
				      ..Default::default()
				  })
		.insert(animations.slow.clone())
		.insert(Play)
		.insert(Timer::from_seconds(5.0, true));
		
            }
	}

    }
}
