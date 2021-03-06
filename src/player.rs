#![allow(unused_imports)]
use crate::{
    collision::{enemy_collision, wall_collision},
    enemy::Enemy,
    wall::Wall,
    animation::{Animations, create_animation},
    PlayerSheet, TILE_SIZE,
    direction::Direction,
    Tree,
};

const PLAYER_SPEED: f32 = 5.0;
const PLAYER_ACTIVE: bool = true;

use core::ops::Deref;
use std::time::Duration;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use benimator::*;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player
{
    speed: f32,
    active: bool,
    just_moved: bool,
}

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app
	          .init_resource::<Animations>()
	          .add_startup_system_to_stage(StartupStage::PreStartup, create_animation)	    
            .add_startup_system( spawn_player.system())
            .add_system(player_camera.after("movement"))
            .add_system(player_movement.label("movement"))
	          .add_system(spawn_trees);
    }
}

fn spawn_player(
	animations: Res<Animations>,
	mut commands: Commands,
	mut texture : ResMut<Assets<TextureAtlas>>,
	assets: Res<AssetServer>,
 ){
    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: texture.add(TextureAtlas::from_grid(
	    assets.load("player-sheet.png"),
	    Vec2::new(9.0, 9.0),
	    2,
	    2,
	)),
        transform: Transform{
            translation: Vec3::new(0.0, 0.0, 900.0),
            scale: Vec3::splat(TILE_SIZE/7 as f32),
            ..Default::default()
        },
        ..Default::default()
    })
	//insert(Timer::from_seconds(5.0, true))
	.insert(animations.fast.clone())
	.insert(Play)
	.insert(Timer::from_seconds(5.0, true))
	.insert(Player{speed: 5.0, active: true, just_moved: true});
}



//player camera movement
fn player_camera
    (
    pq: Query<&Transform, With<Player>>,
    mut cam_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,

    )
{
    let pt = pq.single();
    let mut camt = cam_query.single_mut();
    camt.translation.x = pt.translation.x;
    camt.translation.y = pt.translation.y;
}


#[allow(unused)]
fn player_movement
    (
    mut player_query: Query<(Entity, &mut Transform), (With<Player>, Without<Wall>, Without<Tree>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    wall_stuff: Query<Entity, (With<Wall>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut texture: ResMut<Assets<TextureAtlas>>,
    assets: Res<AssetServer>,
 )
{
    let (mut _player, mut transform) = player_query.single_mut();
    let mut dir = Direction::No;

            let mut just_moved : bool = false;

            if !PLAYER_ACTIVE
            {
                return;
            }

            let mut y_delta = 0.0;
            if keyboard.pressed(KeyCode::Up)
            {
	            dir = Direction::Up;
                just_moved = true;
       
            }
            if keyboard.pressed(KeyCode::Down)
            {
	            dir = Direction::Down;
                just_moved = true;
        
            }

            if keyboard.pressed(KeyCode::Up) && keyboard.pressed(KeyCode::LShift)
            {
	            y_delta +=PLAYER_SPEED  * 1.01 * TILE_SIZE * time.delta_seconds();
                just_moved = true;
            }
            
            if keyboard.pressed(KeyCode::Down) && keyboard.pressed(KeyCode::LShift)
            {
	            y_delta -= PLAYER_SPEED * 1.01 * TILE_SIZE * time.delta_seconds();
                just_moved = true;
            }

            let mut x_delta = 0.0;
            if keyboard.pressed(KeyCode::Left)
            { 
	            dir = Direction::Left;
                just_moved = true;
            }

            if keyboard.pressed(KeyCode::Right)
            {
	            dir = Direction::Right;
                just_moved = true;
            }

            if keyboard.pressed(KeyCode::Left) && keyboard.pressed(KeyCode::LShift)
            {
	            x_delta -= PLAYER_SPEED * TILE_SIZE* 1.01 * time.delta_seconds();
                just_moved = true;
            }

            if keyboard.pressed(KeyCode::Right) && keyboard.pressed(KeyCode::LShift)
            {
	            x_delta += PLAYER_SPEED * 1.01 *TILE_SIZE * time.delta_seconds();
                just_moved = true;
            }

            match dir{
	            Direction::Up =>  y_delta += PLAYER_SPEED * TILE_SIZE * time.delta_seconds(),
	            Direction::Down => y_delta -= PLAYER_SPEED * TILE_SIZE * time.delta_seconds(),
	            Direction::Left => x_delta -= PLAYER_SPEED * TILE_SIZE * time.delta_seconds(),
	            Direction::Right => x_delta += PLAYER_SPEED * TILE_SIZE * time.delta_seconds(),
	            Direction::No => (),
            }
        
            let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
            if !wall_query
                .iter()
                .any(| &transform| wall_collision(target, transform.translation))
            {
                if x_delta != 0.0 {
                    just_moved = true;
                }
                transform.translation = target;
            }

 
            if !wall_query.iter().any(| &transform| wall_collision(target, transform.translation)) && keyboard.just_pressed(KeyCode::A)
            {
		        println!("Hello World");
            }
          

            let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
            if !wall_query.iter().any(|&transform| wall_collision(target, transform.translation)) {
                if y_delta != 0.0 {
                    just_moved = true;
                }
                    transform.translation = target;
            }
    
            if !wall_query.iter().any(|&transform| wall_collision(target, transform.translation)) && keyboard.just_pressed(KeyCode::D)
            {
                    println!("Hello World");                
            }
            println!("Hello World");

    
}

fn spawn_trees(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut tree_query: Query<(Entity, &Transform), With<Tree>>,
    keyboard: Res<Input<KeyCode>>,
    assets: Res<AssetServer>,
    animations: Res<Animations>,
    mut texture: ResMut<Assets<TextureAtlas>>,
){
    for  (mut _player, mut player_tf) in player_query.iter(){
	      for (mut tree, mut tree_tf) in tree_query.iter(){
	          let tree_collision = collide(
		            player_tf.translation,
		            Vec2::splat(TILE_SIZE * 0.9),
		            tree_tf.translation,
		            Vec2::splat(TILE_SIZE * 0.9),
	          );

	    

	          if tree_collision.is_some() && keyboard.pressed(KeyCode::D){
		            commands.entity(tree).despawn();
	          }
	
	      } 
            if keyboard.just_pressed(KeyCode::T){
         	
		        commands.spawn_bundle(SpriteSheetBundle{
		            texture_atlas: texture.add(TextureAtlas::from_grid(
			              assets.load("tree-sheet.png"),
			              Vec2::new(16.0, 16.0),
			              3,
			              3
		            )),
		            transform: Transform{
			          translation: player_tf.translation,
			          scale: Vec3::splat(TILE_SIZE/7 as f32),
			          ..Default::default()
		            }, 
		            ..Default::default()
		        })
		        .insert(animations.tree_speed.clone())
		        .insert(Play)
		        .insert(Timer::from_seconds(5.0, true))
		        .insert(Tree);
	      }
    }
    
}
