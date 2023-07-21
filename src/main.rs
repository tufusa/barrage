#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示

use app_state::AppState;
use bevy::prelude::*;

mod app_state;
mod config;
mod font;
mod hierarchy;
mod in_game;
mod plugins;
mod title;

fn main() {
    App::new()
        .add_plugin(plugins::Base)
        .add_state::<AppState>()
        .add_plugin(plugins::in_game::NewBulletEvents)
        .add_system(setup.on_startup())
        .add_system(title::setup.in_schedule(OnEnter(AppState::Title)))
        .add_system(title::cleanup.in_schedule(OnExit(AppState::Title)))
        .add_system(in_game::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_plugin(plugins::in_game::InGameUpdateSystems)
        .add_system(in_game::cleanup.in_schedule(OnExit(AppState::InGame)))
        .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>, mut window_query: Query<&mut Window>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
    commands.insert_resource(font::UI(server.load("fonts/Roboto-Thin.ttf")));
    in_game::game_timer::setup(&mut commands);
    window_query.single_mut().cursor.visible = false;
}
