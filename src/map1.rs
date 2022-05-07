use bevy::prelude::*;
use crate::{WallSheet, TILE_SIZE, animation::Animations};
use std::{fs::File, io::*};
use benimator::*;

use crate::{
    enemy::Enemy,
    wall::{spawn_walls},
    wall::Wall,
    coin::Coin,
    water::Water,
};

pub struct Map1;

impl Plugin for Map1 {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map);
    }
}

fn create_map
    (
	mut commands: Commands,
	tree: Res<WallSheet>,
	assets: Res<AssetServer>,
	mut texture: ResMut<Assets<TextureAtlas>>,
	animations: Res<Animations>,
	mut keyboard: Res<Input<KeyCode>>,
	
    )
{
    let file = File::open("assets/map1.txt").expect("Map1 not found");
    let mut tiles = Vec::new();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_walls(
                    &mut commands,
                    &tree,
                    char as usize,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                );
		
		
                if char == '#' {
                    commands.entity(tile).insert(Wall);
                }
		

                if char == '@' {
                    commands.spawn_bundle(SpriteSheetBundle{
                        texture_atlas: texture.add(TextureAtlas::from_grid(
			    assets.load("ghost-sheet.png"),
			    Vec2::new(9.0, 9.0), 2, 2)
			),
			
                        transform: Transform{
                            translation: Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                            scale: Vec3::splat(TILE_SIZE/7 as f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(animations.slow.clone())
                    .insert(Play)
                    .insert(Timer::from_seconds(5.0, true))
                    .insert(Enemy);
                }
                    

                if char == '+' {
                    commands.entity(tile).insert(Enemy);
                }

		if char == '0'{
		      commands.spawn_bundle(SpriteBundle{
			  texture: assets.load("object.png"),
			  transform: Transform{
			      translation: Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
			      scale: Vec3::splat(TILE_SIZE/7 as f32),
			      ..Default::default()
			  },
			  ..Default::default()
		      }).insert(Coin);
		}


		if char  == '{'{
		    commands.spawn_bundle(SpriteSheetBundle{
			texture_atlas: texture.add(TextureAtlas::from_grid(
			    assets.load("water-sheet.png"),
			    Vec2::new(9.0, 9.0),
			    2,
			    2
			)),
			transform: Transform{
			    translation: Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
			    scale: Vec3::splat(TILE_SIZE/7 as f32),
			    ..Default::default()
			},
			..Default::default()
		    })
			.insert(animations.slow.clone())
			.insert(Play)
			.insert(Timer::from_seconds(5.0, true))
			.insert(Water);
		}
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Map1"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}
