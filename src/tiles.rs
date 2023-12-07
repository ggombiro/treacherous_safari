use crate::game_state::{GameState, GameStates};
use crate::movement::{MovementPoints, update_movement_points, create_movement_points, MovementPointsUpdateEvent};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, sprite::Anchor, ecs::system::EntityCommands};
use bevy_mod_picking::prelude::*;


const FOCUS_SCALE: f32 = 0.1;
const SELECTED_SCALE: f32 = 2.0;


#[derive(Component)]
pub struct Tile;

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
            PickableBundle::default(),
            On::<Pointer<Down>>::target_component_mut::<Transform>(|_,transform| {
                tile_selected(transform);
            }),
            On::<Pointer<Over>>::target_component_mut::<Transform>(|_,transform| {
                if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE{
                    transform.scale.x += FOCUS_SCALE;
                    transform.scale.y += FOCUS_SCALE;
                }
            }),
            On::<Pointer<Out>>::target_component_mut::<Transform>(|_,transform| {
                if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE{
                    transform.scale.x -= FOCUS_SCALE;
                    transform.scale.y -= FOCUS_SCALE;
                }
            }),
        ))
        .with_children(|commands| {
            
            const X_START: f32 = -64.0;
            const X_STEP: f32 = 32.0;
            const Y_START: f32 = -97.5;
            const Y_STEP: f32 = 97.5;
            const SPACING: f32 = 50.0;
            
            for x in 0..5{
                for y in 0..3{
                    if (x == 0 || x == 4) && (y == 0 || y == 2){
                        continue;
                    }

                    commands.spawn((
                        SpriteBundle {
                        sprite: Sprite {
                            custom_size: sprite_size,
                            color: Color::BLACK,
                            ..default()
                        },
                        // texture: asset_server.load("images/boovy.png"),
                        transform: Transform::from_xyz((X_START + (x as f32 * X_STEP)) + (x as f32 * SPACING),
                         (Y_START + (y as f32 * Y_STEP)) + (y as f32 * (SPACING/3.0)), -1.0),
                        ..default()
                    },
                    Tile
                ));
    
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
pub struct TileSelected;

// impl From<ListenerInput<Pointer<Down>>> for TileSelected {
//     fn from(_event: ListenerInput<Pointer<Down>>) -> Self {
//         TileSelected
//     }
// }

// #[derive(Event)]
// pub struct TileEntered(Entity, f32);

// impl From<ListenerInput<Pointer<Over>>> for TileEntered {
//     fn from(event: ListenerInput<Pointer<Over>>) -> Self {
//         TileEntered(event.target, event.hit.depth)
//     }
// }

// #[derive(Event)]
// pub struct TileExited(Entity, f32);

// impl From<ListenerInput<Pointer<Out>>> for TileExited {
//     fn from(event: ListenerInput<Pointer<Out>>) -> Self {
//         TileExited(event.target, event.hit.depth)
//     }
// }

pub fn tile_selected(transform: &mut  Transform) {
    if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE{
        transform.scale.x += SELECTED_SCALE;
        transform.scale.y += SELECTED_SCALE;
    }
}

pub fn on_tile_selected(
    mut commands: Commands,
    mut events: EventReader<TileSelected>,
){
    info!("On tile selected");
    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(500.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    margin: UiRect::horizontal(Val::Auto),
                    ..default()
                },
                ..default()
            },
            // *** Important! ***
            //
            // We need to use `Pickable::IGNORE` here so the root node doesn't block pointer
            // interactions from reaching the 3d objects under the UI.
            //
            // This node, as defined, will stretch from the top to bottom of the screen, take the
            // width of the buttons, but will be invisible. Try commenting out this line or changing
            // it to see what happens.
            // Pickable::IGNORE,
        ))
        .id();

    commands
        .entity(root)
        .add_button(
            "Close",
            On::<Pointer<Click>>::send_event::<CycleLogging>(),
        );
}

#[derive(Clone, Event)]
struct CycleLogging(Entity);

impl From<ListenerInput<Pointer<Click>>> for CycleLogging {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        CycleLogging(event.target) // you could use this to choose between different buttons
    }
}

// change log verbosity by cycling through the DebugPickingMode state
fn cycle_logging(
) {
    
}

trait NewButton {
    fn add_button(self, text: &str, on_click_action: On<Pointer<Click>>) -> Self;
}

impl<'w, 's, 'a> NewButton for EntityCommands<'w, 's, 'a> {
    fn add_button(mut self, text: &str, on_click_action: On<Pointer<Click>>) -> Self {
        let child = self
            .commands()
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(42.0),
                        margin: UiRect::top(Val::Percent(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                // Add an onclick
                on_click_action,
                // Buttons should not deselect other things:
                NoDeselect,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section(
                            text,
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        ..default()
                    },
                    // Text should not be involved in pick interactions.
                    Pickable::IGNORE,
                ));
            })
            .id();
        self.add_child(child);
        self
    }
}

// pub fn tile_entered(
//     mut commands: Commands,
//     mut events: EventReader<TileEntered>,
//     mut pointer_over_fire: EventWriter<PointerOver>,
// ) {
//     for event in events.read() {

//         pointer_over_fire.send(PointerOver(event.0));
//         // info!(
//         //     "{:?}, pointer has entered",
//         //     event.0
//         // );
//     }
// }

// pub fn tile_exited(mut events: EventReader<TileExited>) {
//     for event in events.read() {
//         info!(
//             "{:?}, pointer has exited",
//             event.0
//         );
//     }
// }

// #[derive(Event)]
// pub struct PointerOver(Entity);

// pub fn focus_tile(
//     mut commands: Commands,
//     mut events: EventReader<PointerOver>,
//     mut focused_tiles: Query<(Entity, &mut Transform, With<Tile>)>
// ) {

//     info!("Focus called");
    
//     for (entity, mut transform, _) in &mut focused_tiles{

//         info!("All {:?}", entity);

        

//         for event in events.read() {

            
//             if entity.index() == event.0.index() {
//                 info!("Looking for  {:?}", event.0);
//             }

//             // info!("Focus received");
//             // transform.scale.x += 10.0;
//             // transform.scale.y += 10.0;
//         }
//     }


// }