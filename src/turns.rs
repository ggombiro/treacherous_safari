use bevy::prelude::*;
use crate::game_state::{GameState, GameStates};

// mod game_state;


#[derive(Resource)]
pub struct TurnsLeft(pub i32);

#[derive(Component)]
pub struct TurnsLeftUpdate {
    value: i32,
}

pub fn update_turns_left(
    mut commands: Commands,
    mut turns_left: ResMut<TurnsLeft>,
    mut game_state: ResMut<GameState>,
    turns_left_updates: Query<(Entity, &TurnsLeftUpdate)>
){
    match game_state.0{
        GameStates::TurnEnd => {
            turns_left.0 -= 1;
            game_state.0 = GameStates::TileReveal;
        },
        _ => {},
    }

    for (turns_left_entity, turns_left_update) in &turns_left_updates{
        turns_left.0 += turns_left_update.value;
        commands.entity(turns_left_entity).despawn();
    }
}