use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    enemy::Enemy,
    npc::Npc,
    player::Player,
    coin::Coin,
    TILE_SIZE,
};


const _HELLO: &str = "hello.png";
const _WHITE: &str = "white.png.png";

//text
#[allow(unused)]
#[derive(Component)]
pub struct MessageText;

pub fn wall_collision(
    player_pos: Vec3,
    wall_query: Vec3,
) -> bool {
    let collision = collide(
        player_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        wall_query,
        Vec2::splat(TILE_SIZE),
    );

    collision.is_some()
}

pub fn npc_collision(
    //unused for now
    _commands: Commands,
    //unused for now
    _asset_server: Res<AssetServer>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    npc_query: Query<(Entity, &Transform), With<Npc>>,
) -> () {
    for (_player_entity, player_tf) in player_query.iter() {
        for (_npc_entity, npc_tf) in npc_query.iter() {
            let collision = collide(
                player_tf.translation,
                Vec2::splat(TILE_SIZE * 0.9),
                npc_tf.translation,
                Vec2::splat(TILE_SIZE * 0.9),
            );
            if collision.is_some() {
                panic!("Collision");
            }
        }
    }
}


// for (_player_entity, player_tf) in player_query.iter(){

//     for (_npc_entity, npc_tf) in npc_query.iter(){
//         let collision = collide(
//             player_tf.translation,
//                    Vec2::splat(9.0),
//                    npc_tf.translation,
//                    Vec2::splat(9.0),

//         );

//         if let Some(_) = collision{
//             println!("Hello World");
//         }
//     }
// }


pub fn enemy_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    _kb: Res<Input<KeyCode>>,
) {
    
    
    for (_player_entity, player_tf) in player_query.iter()
    {
        for (_enemy_entity, enemy_tf) in enemy_query.iter()
	{
            let enemy_collision = collide(
                player_tf.translation,
                Vec2::splat(TILE_SIZE * 0.9),
                enemy_tf.translation,
                Vec2::splat(TILE_SIZE * 0.9),
            );

            if enemy_collision.is_some()
	    {
		commands.spawn_bundle(SpriteBundle{
		    texture: asset_server.load("you_died_2.png"),
		    transform: Transform{
			translation: player_tf.translation,
			scale: Vec3::splat(0.15),
			..Default::default()
		    },
		    ..Default::default()
		});

		
		
            }

        }
    }

}


pub fn obj_collision(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    object_query: Query<(Entity, &Transform),  With<Coin>>,
    kb: Res<Input<KeyCode>>,
){
    for (_player, player_tf) in player_query.iter(){
	for (object, object_tf) in object_query.iter(){
	    let object_collision = collide(
		player_tf.translation,
		Vec2::splat(TILE_SIZE * 0.9),
		object_tf.translation,
		Vec2::splat(TILE_SIZE * 0.9),
	    );

	    if object_collision.is_some(){
		println!("Object Collision");
	    }

	    if object_collision.is_some() && kb.pressed(KeyCode::X){
		commands.entity(object).despawn();
	    }

	    
	}
    }
}




/*fn conversation() {
    println!("Hello there, what can I call you?");
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            thread::sleep(Duration::from_secs(2));
            println!("Hi! {} I am the green ghost", name);
        }

        Err(e) => {
            println!("Error {}", e);
        }
    }

    println!("Hmm so why are you here?");
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => println!("Oh I see, I think it is a mistake for you to talk to me"),
        Err(e) => println!("Error {}", e),
    }

    println!("Ok...");
}*/
