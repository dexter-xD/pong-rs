use bevy::{
    color::palettes::css::{DARK_GRAY, GREEN, RED},
    prelude::*,
    utils::HashMap,
    window::WindowResolution,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HIGHT: f32 = 720.;
const BALL_RADIUS: f32 = 25.;

fn main() {}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
