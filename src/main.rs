use crate::game_state::{GameState, GameStates};
use crate::movement::{MovementPoints, update_movement_points, create_movement_points, MovementPointsUpdateEvent};
use bevy::prelude::*;


mod game_state;
mod movement;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Treacherous Safari".into(),
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState(GameStates::TileReveal))
        .insert_resource(MovementPoints(0))
        .add_systems(Startup, (setup))
        .add_event::<MovementPointsUpdateEvent>()
        .add_systems(
            Update,
            (
                create_movement_points,
                update_movement_points.run_if(on_event::<MovementPointsUpdateEvent>()),
            ),
        )
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    });
}