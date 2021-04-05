use crate::animation::{Animation, Animations};
use bevy::prelude::*;

use std::collections::HashMap;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

pub struct Unit {
    pub selected: bool,
    pub velocity: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
}

fn spawn_unit(
    commands: &mut Commands,
    translation: Vec3,
    texture_atlas_handle: Handle<TextureAtlas>,
) {
    let mut animations = HashMap::<String, Animation>::new();

    animations.insert("idle".to_string(), Animation::new(vec![1, 2, 3, 4]));
    animations.insert("moving".to_string(), Animation::new(vec![4, 5, 6, 7, 8, 9]));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation,
                scale: Vec3::new(1.25, 1.25, 999.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: 0,
                color: Color::WHITE,
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Animations::new("idle".to_string(), animations))
        .with(MoveOrder { path: vec![] })
        .with(Unit {
            selected: false,
            velocity: Vec2::zero(),
            max_speed: 100.0,
            max_force: 250.0,
        });
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dino.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 24, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    spawn_unit(
        commands,
        Vec3::new(100.0, 100.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_unit(
        commands,
        Vec3::new(0.0, 0.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_unit(
        commands,
        Vec3::new(-100.0, -100.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_unit(
        commands,
        Vec3::new(23.0, 42.0, 500.0),
        texture_atlas_handle.clone(),
    );
}

pub struct MoveOrder {
    pub path: Vec<Vec2>,
}
