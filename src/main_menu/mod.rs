mod components;
mod resources;
mod styles;
mod systems;

use resources::*;
use systems::layout::*;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::AppState;

use self::systems::interactions::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Audio Channel
            .add_audio_channel::<Background>()
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            // Systems
            .add_systems(
                Update,
                (interact_with_new_game_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}
