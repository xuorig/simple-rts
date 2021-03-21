use bevy::prelude::*;

use std::collections::HashMap;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animation_system.system());
    }
}

pub struct Animations {
    pub current_animation: String,
    pub animations: HashMap<String, Animation>,
}

impl Animations {
    pub fn new(current_animation: String, animations: HashMap<String, Animation>) -> Self {
        Self {
            current_animation,
            animations,
        }
    }
}

pub struct Animation {
    current_index: usize,
    sprite_indices: Vec<usize>,
}

impl Animation {
    pub fn new(sprite_indices: Vec<usize>) -> Self {
        Self {
            current_index: 0,
            sprite_indices,
        }
    }
}

fn animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut Animations, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut animations, mut sprite) in query.iter_mut() {
        timer.tick(time.delta().as_secs_f32());

        if timer.finished() {
            let current_state = animations.current_animation.clone();
            let animation = animations.animations.get_mut(&current_state);

            if let Some(animation) = animation {
                let next_index =
                    ((animation.current_index + 1) % animation.sprite_indices.len()) as usize;
                sprite.index = animation.sprite_indices[next_index] as u32;
                animation.current_index = next_index;
            }
        }
    }
}
