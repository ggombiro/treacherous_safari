use crate::game_state::{GameState, GameStates};
use crate::movement::{MovementPoints, update_movement_points, create_movement_points, MovementPointsUpdateEvent};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, sprite::Anchor};
use bevy_mod_picking::prelude::*;

pub fn setup_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    let len = 64.0;
    let height = 97.5;
    let sprite_size = Some(Vec2::new(len, height));

    commands
        .spawn((
            SpatialBundle::default(),
            On::<Pointer<Down>>::send_event::<DoSomethingComplex>(),
        ))
        .with_children(|commands| {
            
            const X_START: f32 = -128.0;
            const X_STEP: f32 = 32.0;
            const Y_START: f32 = -97.5;
            const Y_STEP: f32 = 97.5;
            const SPACING: f32 = 50.0;
            
            for x in 0..4{
                for y in 0..3{
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: sprite_size,
                            color: Color::BLACK,
                            ..default()
                        },
                        // texture: asset_server.load("images/boovy.png"),
                        transform: Transform::from_xyz((X_START + (x as f32 * X_STEP)) + (x as f32 * SPACING),
                         (Y_START + (y as f32 * Y_STEP)) + (y as f32 * (SPACING/3.0)), -1.0),
                        ..default()
                    });
    
                }
            }

                // spawn black square behind sprite to show anchor point
                
                // commands.spawn(SpriteBundle {
                //     sprite: Sprite {
                //         custom_size: sprite_size,
                //         color: Color::RED,
                //         anchor: anchor.to_owned(),
                //         ..default()
                //     },
                //     // texture: asset_server.load("images/boovy.png"),
                //     // 3x3 grid of anchor examples by changing transform
                //     transform: Transform::from_xyz(i * len - len, j * len - len, 0.0)
                //         .with_scale(Vec3::splat(1.0 + (i - 1.0) * 0.2))
                //         .with_rotation(Quat::from_rotation_z((j - 1.0) * 0.2)),
                //     ..default()
                // });
        });
}

#[derive(Event)]
pub struct DoSomethingComplex(Entity, f32);

impl From<ListenerInput<Pointer<Down>>> for DoSomethingComplex {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        DoSomethingComplex(event.target, event.hit.depth)
    }
}

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
pub fn receive_greetings(mut greetings: EventReader<DoSomethingComplex>) {
    for event in greetings.read() {
        info!(
            "Hello {:?}, you are {:?} depth units away from the pointer",
            event.0, event.1
        );
    }
}