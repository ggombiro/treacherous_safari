use bevy::prelude::*;
use crate::{game_state::{GameState, GameStates}, ui::TurnsLeftText};

// mod game_state;


#[derive(Resource)]
pub struct TurnsLeft(pub i32);

#[derive(Event)]
pub struct TurnsUpdateEvent(pub i32);


pub fn update_turns_left(
    mut commands: Commands,
    mut turns_left: ResMut<TurnsLeft>,
    mut turns_update: EventReader<TurnsUpdateEvent>,
    mut texts: Query<&mut Text, With<TurnsLeftText>>, 
){
    let mut text = texts.single_mut();

    for ev in turns_update.read() {
        turns_left.0 += ev.0;
    }

    text.sections[0].value = format!("Points: {:?}", turns_left.0);
}