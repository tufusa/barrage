#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示

use std::f32::consts::PI;

use app_state::AppState;
use bevy::prelude::*;
use in_game::{
    bullet::{self, new_bullet::NewBullet, new_bullet_event_writer, new_bullet_timer},
    bullets::{self, StraightBullet},
};

mod app_state;
mod config;
mod font;
mod hierarchy;
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
                in_game::tracer::trace,
                in_game::enemy::boss::run,
                debug,
                hierarchy::sync::<in_game::enemy::Enemy>,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (new_bullet_event_writer::new_bullet_event_writer::<StraightBullet>,)
                .in_set(OnUpdate(AppState::InGame)),
        ) // Bullet spawn event writer
        .add_systems((new_bullet_timer::tick::<StraightBullet>,).in_set(OnUpdate(AppState::InGame))) // Bullet spawn timer
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
    in_game::bullet::new_bullet_timer::setup(&mut commands);
}

fn debug(mut new_bullet_event: EventWriter<NewBullet<StraightBullet>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Space) {
        new_bullet_event.send(NewBullet {
            bullet: StraightBullet {
                speed: 200.,
                angle: PI / 4.,
            },
            translation: Vec2::ZERO,
        })
    }
}
