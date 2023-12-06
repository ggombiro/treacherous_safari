use crate::game_state::{GameState, GameStates};
use bevy::prelude::*;
use rand::Rng;

mod game_state;


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
        .add_systems(
            Update,
            (
                create_movement_points,
                update_movement_points.after(create_movement_points),
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

pub fn update_movement_points(
    mut commands: Commands,
    mut movement_points: ResMut<MovementPoints>,
    mut points_updates: Query<(Entity, &MovementPointsUpdate)>,
) {
    for (update_entity, movement_points_update) in &mut points_updates {
        movement_points.0 += movement_points_update.value;
        commands.entity(update_entity).despawn();
    }

    info!("Movement points updated: {:?}", movement_points.0);
}


pub fn create_movement_points(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();

        commands.spawn(MovementPointsUpdate {
            value: rng.gen_range(-2..=5),
        });
    }
}

#[derive(Resource)]
pub struct MovementPoints(pub i32);


#[derive(Component)]
pub struct MovementPointsUpdate {
    value: i32,
}