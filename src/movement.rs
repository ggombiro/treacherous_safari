use bevy::prelude::*;
use rand::Rng;

use crate::ui::MovementPointsText;

#[derive(Resource)]
pub struct MovementPoints(pub i32);

#[derive(Event)]
pub struct MovementPointsUpdateEvent(pub i32);


pub fn update_movement_points(
    mut commands: Commands,
    mut movement_points: ResMut<MovementPoints>,
    mut movement_points_update: EventReader<MovementPointsUpdateEvent>,
    mut texts: Query<&mut Text, With<MovementPointsText>>, 
) {

    let mut text = texts.single_mut();

    for ev in movement_points_update.read() {
        movement_points.0 += ev.0;
    }

    text.sections[0].value = format!("Points: {:?}", movement_points.0);
}


pub fn create_movement_points(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut movement_points_update: EventWriter<MovementPointsUpdateEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(-2..=5);

        movement_points_update.send(MovementPointsUpdateEvent(value));
    }
}

pub fn spawn_movement_card(mut commands: Commands){

}