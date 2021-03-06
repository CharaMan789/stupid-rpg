mod collision;
mod debug; 
mod enemy;
mod map1;
mod npc;
mod player;
mod wall;
mod coin;
mod water;
mod animation;
mod direction;
use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode};
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use map1::Map1;
use npc::NpcPlugin;
use player::PlayerPlugin;
use wall::{WallPlugin, WallSheet};
use coin::CoinPlugin;
use benimator::*;

//fn main() {
//  App::new()
//    .add_plugins(DefaultPlugins)
//  .add_startup_system(murder_system)
//.run();
//}

//fn murder_system(mut stop: EventWriter<AppExit>) {
//// stop.send(AppExit);
//}

pub const _BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
pub const BG: Color = Color::rgb(0.8, 0.8, 0.8);
pub const RES: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;
pub struct PlayerSheet(Handle<TextureAtlas>);

#[derive(Component)]
pub struct Tree;

fn main()
{
    let h = 900.0;
    App::new()
        .insert_resource(ClearColor(BG))
        .insert_resource(WindowDescriptor {
            width: h * RES,
            height: h,
            title: "Game".to_string(),
            vsync: true,
            resizable: false,
	        mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_startup_system(setup)
	.add_plugin(CoinPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(NpcPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(Map1)
	.add_system(bevy::input::system::exit_on_esc_system)
        .add_plugins(DefaultPlugins)
	.add_plugin(AnimationPlugin::default())
        .run();
}

fn setup(mut commands: Commands)
{
    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.top = 1.0;
    cam.orthographic_projection.bottom = -1.0;
    cam.orthographic_projection.right = 1.0 * RES;
    cam.orthographic_projection.left = -1.0 * RES;

    cam.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(cam);
}





















