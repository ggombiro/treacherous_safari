use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct MovementPoints(pub i32);


#[derive(Component)]
pub struct MovementPointsUpdate {
    value: i32,
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

pub fn spawn_movement_card(mut commands: Commands){

}