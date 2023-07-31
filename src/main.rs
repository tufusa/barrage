#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示

use app_state::AppState;
use bevy::{prelude::*, window::*};

mod app_state;
mod config;
mod font;
mod hierarchy;
mod in_game;
mod plugins;
mod reset;
mod title;
mod ui;
mod utility;

fn main() {
    App::new()
        .add_plugins((
            plugins::Base,
            plugins::in_game::NewBulletEvents,
            plugins::in_game::InGameUpdateSystems,
        ))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        // .add_systems(OnEnter(AppState::Title), title::setup)
        .add_systems(Update, title::check_next.run_if(in_state(AppState::Title)))
        .add_systems(OnExit(AppState::Title), title::cleanup)
        .add_systems(OnEnter(AppState::InGame), in_game::setup)
        .add_systems(OnExit(AppState::InGame), in_game::cleanup)
        .add_systems(
            Update,
            reset::check_reset.run_if(in_state(AppState::GameClear)),
        )
        .add_systems(
            Update,
            reset::check_reset.run_if(in_state(AppState::GameOver)),
        )
        .add_systems(Update, debug)
        .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>, mut window_query: Query<&mut Window>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
    commands.insert_resource(font::UI(server.load("fonts/Roboto-Thin.ttf")));
    in_game::game_timer::setup(&mut commands);
    window_query.single_mut().cursor.icon = CursorIcon::Crosshair;
}

fn debug(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
}
