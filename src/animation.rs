#![allow(unused_imports)]
use bevy::prelude::*;
use benimator::*;
use core::ops::Deref;
use std::time::Duration;

#[derive(Default)]			   
pub struct Animations{			   
    pub slow: Handle<SpriteSheetAnimation>, 
    pub none: Handle<SpriteSheetAnimation>,
    pub fast: Handle<SpriteSheetAnimation>,
    pub medium: Handle<SpriteSheetAnimation>,
    pub tree_speed: Handle<SpriteSheetAnimation>,
}



pub fn create_animation(mut handles: ResMut<Animations>, mut assets: ResMut<Assets<SpriteSheetAnimation>>){
    handles.fast = assets.add(SpriteSheetAnimation::from_range(
	0..=3,
	Duration::from_millis(200),
    ));

    handles.slow = assets.add(SpriteSheetAnimation::from_range(
	0..=3,
	Duration::from_millis(500),
    ));

    handles.medium = assets.add(SpriteSheetAnimation::from_range(
	0..=3,
	Duration::from_millis(300),
    ));
    handles.none = assets.add(SpriteSheetAnimation::from_range(
            0..=3,
            Duration::from_secs(1000),
    ));

    handles.tree_speed = assets.add(SpriteSheetAnimation::from_range(
	0..=8,
	Duration::from_secs(6),
    ));
}



