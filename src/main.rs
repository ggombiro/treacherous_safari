use crate::game_state::{GameState, GameStates};
use crate::movement::{MovementPoints, update_movement_points, create_movement_points, MovementPointsUpdateEvent};
use crate::tiles::{DoSomethingComplex, receive_greetings, setup_tiles};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, sprite::Anchor};
use bevy_mod_picking::prelude::*;

mod game_state;
mod movement;
mod tiles;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
            ))
        .insert_resource(GameState(GameStates::TileReveal))
        .insert_resource(MovementPoints(0))
        .add_systems(Startup, (
            setup,
            setup_tiles,))
        .add_event::<MovementPointsUpdateEvent>()
        .add_event::<DoSomethingComplex>()
        .add_systems(
            Update,
            (
                create_movement_points,
                update_movement_points.run_if(on_event::<MovementPointsUpdateEvent>()),
                receive_greetings.run_if(on_event::<DoSomethingComplex>()),
            ),
        )
        .run()
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

}



fn move_sprite(
    time: Res<Time>,
    mut sprite: Query<&mut Transform, (Without<Sprite>, With<Children>)>,
) {
    let t = time.elapsed_seconds() * 0.1;
    for mut transform in &mut sprite {
        let new = Vec2 {
            x: 50.0 * t.sin(),
            y: 50.0 * (t * 2.0).sin(),
        };
        transform.translation.x = new.x;
        transform.translation.y = new.y;
    }
}