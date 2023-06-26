#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示

use std::f32::consts::PI;

use app_state::AppState;
use bevy::prelude::*;
use in_game::{
    bullet::{self, NewBullet},
    bullets::{self, StraightBullet},
};

mod app_state;
mod config;
mod font;
mod in_game;
mod plugins;
mod title;

fn main() {
    App::new()
        .add_plugins(plugins::plugins())
        .add_state::<AppState>()
        .add_event::<NewBullet<StraightBullet>>()
        .add_system(setup.on_startup())
        .add_system(title::setup.in_schedule(OnEnter(AppState::Title)))
        .add_system(title::cleanup.in_schedule(OnExit(AppState::Title)))
        .add_system(in_game::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_systems(
            (
                in_game::game_timer::tick,
                in_game::delta::run,
                in_game::delta::run_timer,
                in_game::tracer::trace,
                debug,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems((bullets::straight::spawn,).in_set(OnUpdate(AppState::InGame))) // Bullet spawn
        .add_systems((bullet::run::<StraightBullet>,).in_set(OnUpdate(AppState::InGame))) // Bullet run
        .add_systems((bullet::despawn::<StraightBullet>,).in_set(OnUpdate(AppState::InGame))) // Bullet despawn
        .add_system(in_game::cleanup.in_schedule(OnExit(AppState::InGame)))
        .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
    commands.insert_resource(font::UI(server.load("fonts/Roboto-Thin.ttf")));
    in_game::game_timer::setup(&mut commands);
}

fn debug(mut new_bullet_event: EventWriter<NewBullet<StraightBullet>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Space) {
        println!("pressed");
        new_bullet_event.send(NewBullet {
            bullet: StraightBullet {
                speed: 100.,
                angle: PI / 4.,
            },
            translation: Vec3::ZERO,
        })
    }
}
