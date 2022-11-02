use rand::prelude::*;
use bevy::{
    prelude::*,
    window::WindowMode
};

// CONSTANTS







// COMPONENTS





// EVENTS and AppState

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Setup,
    MainMenu,
    Loading,
    Running,
    Pause,
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Setup)
        .add_startup_system(window_setup)
        .run();
}

fn window_setup(
    mut windows: ResMut<Windows>,
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    for window in windows.iter_mut() {
        window.set_mode(WindowMode::BorderlessFullscreen);
        window.set_resizable(false);
        window.set_title(String::from("Project Bones"));
    }
}