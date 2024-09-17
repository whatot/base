use std::collections::HashMap;

use ggez::{
    audio::{self, SoundSource},
    Context,
};
use specs::{World, WorldExt};

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, audio::Source>,
}

pub fn initialize_sounds(world: &mut World, context: &mut Context) {
    let mut audio_source = world.write_resource::<AudioStore>();
    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let sound_name = sound.to_string();
        let sound_path = format!("/sounds/{}.wav", sound_name);
        let sound_source = audio::Source::new(context, sound_path).expect("expected sound loaded");

        audio_source.sounds.insert(sound_name, sound_source);
    }
}
impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: &String) {
        let _ = self
            .sounds
            .get_mut(sound)
            .expect("expected sound")
            .play_detached(context);
    }
}
