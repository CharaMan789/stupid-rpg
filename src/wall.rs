use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct WallSheet(Handle<TextureAtlas>);

pub struct WallPlugin;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Text{
    pub id: usize,
}

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_wall_sheet);
    }
}

pub fn spawn_walls(
    commands: &mut Commands,
    wall: &WallSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    assert!(index < 256, "Too big of an index");
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: wall.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn load_wall_sheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tree = assets.load("Ascii.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(tree, Vec2::splat(9.0), 16,16, Vec2::splat(2.0));
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(WallSheet(atlas_handle));
}



