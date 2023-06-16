#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示

use bevy::prelude::*;

mod app_state;
mod config;
mod font;
mod plugins;
mod title;
mod in_game;

fn main() {
    App::new()
        .add_plugins(plugins::plugins())
        .add_state::<app_state::AppState>()
        .add_system(setup.on_startup())
        .add_system(title::setup.in_schedule(OnEnter(app_state::AppState::Title)))
        .add_system(title::cleanup.in_schedule(OnExit(app_state::AppState::Title)))
        .add_system(in_game::setup.in_schedule(OnEnter(app_state::AppState::InGame)))
        .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(font::UI(server.load("fonts/Roboto-Thin.ttf")));
}
