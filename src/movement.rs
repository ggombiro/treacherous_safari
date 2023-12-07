use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct MovementPoints(pub i32);


#[derive(Component)]
pub struct MovementPointsUpdate {
    value: i32,
}

#[derive(Event)]
pub struct MovementPointsUpdateEvent(Entity, i32);



pub fn update_movement_points(
    mut commands: Commands,
    mut movement_points: ResMut<MovementPoints>,
    mut movement_points_update: EventReader<MovementPointsUpdateEvent>,
) {

    for ev in movement_points_update.read() {
        eprintln!("Entity {:?} leveled up!", ev.0);
        movement_points.0 += ev.1;
        commands.entity(ev.0).despawn();
    }

    info!("Movement points updated: {:?}", movement_points.0);
}


pub fn create_movement_points(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut movement_points_update: EventWriter<MovementPointsUpdateEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(-2..=5);

        let entity = commands.spawn(MovementPointsUpdate {
            value,
        });

        movement_points_update.send(MovementPointsUpdateEvent(entity.id(), value));
    }
}

pub fn spawn_movement_card(mut commands: Commands){

}