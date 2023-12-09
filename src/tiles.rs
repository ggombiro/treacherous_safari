use std::fmt::Alignment;

use crate::game_state::{GameState, GameStates};
use crate::movement::{
    create_movement_points, update_movement_points, MovementPoints, MovementPointsUpdateEvent,
};
use crate::turns::TurnsLeft;
use bevy::text::{BreakLineOn, Text2dBounds, TextLayoutInfo};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use rand::Rng;

const FOCUS_SCALE: f32 = 0.1;
const SELECTED_SCALE: f32 = 2.0;
const BLOCKER_COLOR_VALUE: f32 = 0.1;

#[derive(Component, Debug, Clone, Default)]
pub struct Tile {
    pub cost: u32,
    pub description: String,
    pub number: u32,
    pub neighbours: Vec<i32>,
    pub tile_type: TileType,
    pub value: i32,
    pub duration: i32,
    pub current: bool,
}

#[derive(Component)]
pub struct TileRevealBlocker;

#[derive(Component)]
pub struct TileRevealBlockerCloseButton;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct TileCover;

#[derive(Event)]
pub struct TileSetupComplete;

#[derive(Resource)]
pub struct VisitedTiles(pub Vec<u32>);

pub fn setup_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tile_setup_complete: EventWriter<TileSetupComplete>,
) {
    let len = 80.0 * 3.0;
    let height = 97.5 * 3.0;
    let sprite_size = Some(Vec2::new(len, height));
    let mut tile_res = generate_tiles();
    // let mut rng = rand::thread_rng();

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_scale(Vec3 {
                    x: 0.3,
                    y: 0.3,
                    z: 1.0,
                }),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<TileSelected>(),
            On::<Pointer<Over>>::target_component_mut::<Transform>(|_, transform| {
                if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
                    transform.scale.x += FOCUS_SCALE;
                    transform.scale.y += FOCUS_SCALE;
                }
            }),
            On::<Pointer<Out>>::target_component_mut::<Transform>(|_, transform| {
                if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
                    transform.scale.x -= FOCUS_SCALE;
                    transform.scale.y -= FOCUS_SCALE;
                }
            }),
        ))
        .with_children(|commands| {
            const X_START: f32 = -64.0;
            const X_STEP: f32 = 128.0;
            const Y_START: f32 = -97.5;
            const Y_STEP: f32 = 195.0;
            const SPACING: f32 = 150.0;

            let mut counter = 0;
            let mut tile_res_index: usize = 30;

            for x in 0..5 {
                for y in 0..3 {
                    if (x == 0 || x == 4) && (y == 0 || y == 2) {
                        continue;
                    }

                    let mut rng = rand::thread_rng();

                    tile_res_index = rng.gen_range(0..tile_res.len());
                    let tile = &mut tile_res[tile_res_index];

                    tile.number = counter;
                    tile.neighbours = get_neighbours(counter);
                    tile.current = if counter == 0 { true } else { false };

                    commands
                        .spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: sprite_size,
                                    // color: Color::BLACK,
                                    ..default()
                                },
                                texture: asset_server.load("cardBack_blue1.png"),
                                transform: Transform::from_xyz(
                                    (X_START + (x as f32 * X_STEP)) + (x as f32 * SPACING),
                                    (Y_START + (y as f32 * Y_STEP)) + (y as f32 * (SPACING)),
                                    -1.0,
                                ),
                                ..default()
                            },
                            tile.clone(),
                        ))
                        .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
                            let text_style = TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            };

                            parent.spawn(Text2dBundle {
                                text: Text::from_section(
                                    format!("{}", &tile.cost),
                                    TextStyle {
                                        font_size: 50.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ),
                                transform: Transform {
                                    translation: Vec3::new(0.0, 135.0, 1.0),
                                    ..default()
                                },
                                text_anchor: Anchor::TopCenter,
                                ..default()
                            });

                            let other_box_size = Vec2::new(190.0, 350.0);

                            parent.spawn(Text2dBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        format!("{}", &tile.description),
                                        TextStyle {
                                            font_size: 25.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                            ..default()
                                        },
                                    )],
                                    linebreak_behavior: BreakLineOn::WordBoundary,
                                    alignment: TextAlignment::Left,
                                },
                                text_2d_bounds: Text2dBounds {
                                    // Wrap text in the rectangle
                                    size: other_box_size,
                                },
                                transform: Transform {
                                    translation: Vec3::new(0.0, 75.0, 1.0),
                                    ..default()
                                },
                                text_anchor: Anchor::TopCenter,
                                ..default()
                            });

                            parent.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: sprite_size,
                                        ..default()
                                    },
                                    texture: asset_server.load("cardBack_blue1.png"),
                                    transform: Transform::from_xyz(0.0, 0.0, 1.1),
                                    ..default()
                                },
                                TileCover,
                                Pickable::IGNORE,
                            ));
                        });

                    counter += 1;
                }

                if tile_res_index < tile_res.len() {
                    tile_res.remove(tile_res_index);
                }
            }
        });

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_scale(Vec3::splat(100.0)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            TileRevealBlocker,
            PickableBundle::default(),
        ))
        .with_children(|commands| {
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    custom_size: sprite_size,
                    color: Color::rgba(
                        BLOCKER_COLOR_VALUE,
                        BLOCKER_COLOR_VALUE,
                        BLOCKER_COLOR_VALUE,
                        0.2,
                    ),
                    ..default()
                },
                // texture: asset_server.load("images/boovy.png"),
                ..default()
            },));
        });

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(10.0),
                    height: Val::Px(42.0),
                    margin: UiRect::top(Val::Percent(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    left: Val::Percent(85.0),
                    top: Val::Percent(5.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            On::<Pointer<Click>>::send_event::<TileSelectedBlockerClose>(),
            NoDeselect,
            TileRevealBlockerCloseButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "X",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ),
                    ..default()
                },
                Pickable::IGNORE,
            ));
        });

    tile_setup_complete.send(TileSetupComplete);
}

#[derive(Event)]
pub struct TileSelected(Entity);

impl From<ListenerInput<Pointer<Down>>> for TileSelected {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        TileSelected(event.target)
    }
}

pub fn on_tile_selected(
    mut commands: Commands,
    mut events: EventReader<TileSelected>,
    mut tiles: Query<(Entity, &mut Transform, &mut Tile)>,
    mut blocker: Query<&mut Visibility, With<TileRevealBlocker>>,
    mut close_button: Query<(
        &mut Visibility,
        &TileRevealBlockerCloseButton,
        Without<TileRevealBlocker>,
    )>,
    mut visited_tiles: ResMut<VisitedTiles>,
) {
    for (entity, mut transform, mut tile) in &mut tiles {
        info!("Scale {:?} -> {}", transform.scale, FOCUS_SCALE + SELECTED_SCALE);
        if transform.scale.x > 1.0 {
            if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
                transform.scale.x += SELECTED_SCALE;
                transform.scale.y += SELECTED_SCALE;
                transform.translation.z = 1.0;

                let mut blocker = blocker.single_mut();
                *blocker = Visibility::Visible;

                let mut close_button = close_button.single_mut();
                *close_button.0 = Visibility::Visible;

                if !visited_tiles.0.iter().any(|t| *t == tile.number){
                    visited_tiles.0.push(tile.number);
                }
            }
        }
    }
}

#[derive(Clone, Event)]
pub struct TileSelectedBlockerClose(Entity);

impl From<ListenerInput<Pointer<Click>>> for TileSelectedBlockerClose {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        TileSelectedBlockerClose(event.target)
    }
}

pub fn tile_selected_close(
    mut commands: Commands,
    mut events: EventReader<TileSelectedBlockerClose>,
    mut tiles: Query<(&mut Transform, &Tile)>,
    mut blocker: Query<&mut Visibility, With<TileRevealBlocker>>,
    mut close_button: Query<(
        &mut Visibility,
        &TileRevealBlockerCloseButton,
        Without<TileRevealBlocker>,
    )>,
    mut visited_tiles: ResMut<VisitedTiles>,
    mut movement_points: ResMut<MovementPoints>,
    mut turns_left: ResMut<TurnsLeft>,
) {

    let mut tile_clone  = Tile{..default()};

    for (mut transform, mut tile) in &mut tiles {
        if transform.scale.x > SELECTED_SCALE {
            transform.scale.x -= (FOCUS_SCALE + SELECTED_SCALE);
            transform.scale.y -= (FOCUS_SCALE + SELECTED_SCALE);
            transform.translation.z = -1.0;

            let mut blocker = blocker.single_mut();
            *blocker = Visibility::Hidden;

            let mut close_button = close_button.single_mut();
            *close_button.0 = Visibility::Hidden;

            info!("Tile: {:?}", tile);

            //move PC

            tile_clone = tile.clone();
        }
    }

    //check for add entity

    match tile_clone.tile_type{
        TileType::Plain => {},
        TileType::CurseMovementPoints => {
            info!("Curse movement");
            movement_points.0 -= tile_clone.value;

            if tile_clone.duration != 0{
                //create or add to movement curse comp
            }
        },
        TileType::MovementPointsAdd => {
            
            movement_points.0 += tile_clone.value;

            if tile_clone.duration > 0{

            }
        },
        TileType::TurnReduction => {
            info!("Turn reduction");
            turns_left.0 -= tile_clone.value;
        },
        TileType::RouteRestriction => {
            info!("Route restriction");
            // create route restriction component
        },
        TileType::MovementPointsSub => {
            info!("Movement sub");
            movement_points.0 -= tile_clone.value;

            if tile_clone.duration > 0{
                //create or add to movement sub comp
            }
        },
        TileType::Blessing => {
            info!("Blessing");
            //remove ailments components
        },
        TileType::StepBack => {
            info!("Step back");
            if visited_tiles.0.len() > 1{
                let prev_tile = visited_tiles.0[visited_tiles.0.len() - 2];

                for (_, mut tile) in &mut tiles {
                    if tile.number == prev_tile{
                        //move PC to this tile
                        break;
                    }
                }
            }
        },
    }
}

#[derive(Debug, Clone, Default)]
pub enum TileType {
    #[default] Plain,
    CurseMovementPoints,
    MovementPointsAdd,
    TurnReduction,
    RouteRestriction,
    MovementPointsSub,
    Blessing,
    StepBack,
}

pub fn get_neighbours(index: u32) -> Vec<i32> {
    match index {
        0 => vec![1, 2, 3],
        1 => vec![2, 4, 5],
        2 => vec![1, 3, 4, 5, 6],
        3 => vec![2, 5, 6],
        4 => vec![5, 7, 8],
        5 => vec![4, 6, 7, 8, 9],
        6 => vec![5, 8, 9],
        7 => vec![8, 10],
        8 => vec![7, 9, 10],
        9 => vec![8, 10],
        _ => vec![-1],
    }
}

pub fn on_tile_setup_complete(
    mut commands: Commands,
    mut events: EventReader<TileSetupComplete>,
    mut tiles: Query<(Entity, &mut Transform, &mut Tile, &Children)>,
    mut tile_selected: EventWriter<TileSelected>,
    mut tile_cover_query: Query<&mut Visibility, With<TileCover>>,
) {
    for (entity, mut transform, mut tile, mut children) in &mut tiles {
        info!("Tile: {:?}", tile);
        if tile.current {
            info!("Current Tile: {:?}", tile);
            for child in children {
                if let Ok(mut vis) = tile_cover_query.get_mut(*child) {
                    *vis = Visibility::Hidden;
                }
            }

            transform.scale.x += FOCUS_SCALE;
            transform.scale.y += FOCUS_SCALE;

            tile_selected.send(TileSelected(entity));
        }
    }
}

pub fn generate_tiles() -> Vec<Tile> {
    let mut tile_res = Vec::with_capacity(2);

    let mut tile_1_0 = Tile {
        cost: 1,
        description: String::from(
            "You find ruins with a shiny object. \"Stop!\" You touch it and get cursed. Lose 2 movement points each turn.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::CurseMovementPoints,
        value: -2,
        duration: -1,
        current: false,
    };

    let mut tile_1_1 = Tile {
        cost: 1,
        description: String::from(
            "Tree with strange fruit. You bite it, \"yum.\" Gain 1 movement point.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsAdd,
        value: 1,
        duration: 1,
        current: false,
    };

    let mut tile_1_2 = Tile {
        cost: 1,
        description: String::from("Lion fight. You survive. Go back one tile."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::StepBack,
        value: 1,
        duration: 0,
        current: false,
    };

    let mut tile_2_0 = Tile {
        cost: 2,
        description: String::from("River! You have to swim across, lose 1 turn."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::TurnReduction,
        value: -1,
        duration: 1,
        current: false,
    };

    let mut tile_2_1 = Tile {
        cost: 2,
        description: String::from(
            "Village with nice people. They give you a reed bed. You sleep well. No more ailments or curses.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::Blessing,
        value: 0,
        duration: 0,
        current: false,
    };

    let mut tile_2_2 = Tile {
        cost: 2,
        description: String::from("Cool breeze. Adventure time."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::Plain,
        value: 0,
        duration: 0,
        current: false,
    };

    let mut tile_3_0 = Tile {
        cost: 3,
        description: String::from("Native guides you. Go up or down 1 tile."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::RouteRestriction,
        value: 0,
        duration: 0,
        current: false,
    };

    let mut tile_3_1 = Tile {
        cost: 3,
        description: String::from(
            "Snake bite. You are poisoned. Lose 1 movement point for 2 turns.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsSub,
        value: -1,
        duration: 2,
        current: false,
    };

    let mut tile_3_2 = Tile {
        cost: 3,
        description: String::from(
            "Tree with strange fruit. You bite it, \"yum.\" Gain 1 movement point.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsAdd,
        value: 1,
        duration: 0,
        current: false,
    };

    let mut tile_4_0 = Tile {
        cost: 4,
        description: String::from("River! You have to swim across, lose 1 turn."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::TurnReduction,
        value: -1,
        duration: 1,
        current: false,
    };

    let mut tile_4_1 = Tile {
        cost: 4,
        description: String::from(
            "You find ruins with a shiny object. \"Stop!\" You touch it and get cursed. Lose 2 movement points each turn.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::CurseMovementPoints,
        value: -2,
        duration: -1,
        current: false,
    };

    let mut tile_4_2 = Tile {
        cost: 4,
        description: String::from(
            "Village with nice people. They give you a reed bed. You sleep well. No more ailments or curses.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::Blessing,
        value: -1,
        duration: 0,
        current: false,
    };

    let mut tile_5_0 = Tile {
        cost: 5,
        description: String::from(
            "You find ruins with a shiny object. \"Stop!\" You touch it and get cursed. Lose 2 movement points each turn.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::CurseMovementPoints,
        value: -2,
        duration: -1,
        current: false,
    };

    let mut tile_5_1 = Tile {
        cost: 5,
        description: String::from(
            "Tree with strange fruit. You bite it, \"yum.\" Gain 1 movement point.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsAdd,
        value: 1,
        duration: 0,
        current: false,
    };

    let mut tile_5_2 = Tile {
        cost: 5,
        description: String::from(
            "You befriend an elephant. You ride on its back. Gain 2 movement points.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsAdd,
        value: 2,
        duration: 0,
        current: false,
    };

    let mut tile_6_0 = Tile {
        cost: 6,
        description: String::from("Lion fight. You survive. Go back one tile."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::StepBack,
        value: 1,
        duration: 0,
        current: false,
    };

    let mut tile_6_1 = Tile {
        cost: 6,
        description: String::from("Cool breeze. Adventure time."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::Plain,
        value: 0,
        duration: 0,
        current: false,
    };

    let mut tile_6_2 = Tile {
        cost: 6,
        description: String::from("Native guides you. Go forward 1 tile."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::RouteRestriction,
        value: 0,
        duration: 0,
        current: false,
    };

    let mut tile_7_0 = Tile {
        cost: 7,
        description: String::from("River! You have to swim across, lose 1 turn."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::TurnReduction,
        value: -1,
        duration: 1,
        current: false,
    };

    let mut tile_7_1 = Tile {
        cost: 7,
        description: String::from(
            "Village with nice people. They give you a reed bed. You sleep well. No more ailments or curses.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::Blessing,
        value: -1,
        duration: 0,
        current: false,
    };

    let mut tile_7_2 = Tile {
        cost: 7,
        description: String::from("Lion fight. You survive. Go back one tile."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::StepBack,
        value: 1,
        duration: 0,
        current: false,
    };

    let mut tile_8_0 = Tile {
        cost: 8,
        description: String::from(
            "You befriend an elephant. You ride on its back. Gain 2 movement points.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsAdd,
        value: 2,
        duration: 0,
        current: false,
    };

    let mut tile_8_1 = Tile {
        cost: 8,
        description: String::from("Native guides you. Go up or down 1 tile."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::RouteRestriction,
        value: 0,
        duration: 0,
        current: false,
    };

    let mut tile_8_2 = Tile {
        cost: 8,
        description: String::from(
            "Snake bite. You are poisoned. Lose 1 movement point for 2 turns.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsSub,
        value: -1,
        duration: 2,
        current: false,
    };

    let mut tile_9_0 = Tile {
        cost: 9,
        description: String::from(
            "Snake bite. You are poisoned. Lose 1 movement point for 2 turns.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsSub,
        value: -1,
        duration: 2,
        current: false,
    };

    let mut tile_9_1 = Tile {
        cost: 9,
        description: String::from(
            "You befriend an elephant. You ride on its back. Gain 2 movement points.",
        ),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::MovementPointsAdd,
        value: 2,
        duration: 0,
        current: false,
    };

    let mut tile_9_2 = Tile {
        cost: 9,
        description: String::from("Cool breeze. Adventure time."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::Plain,
        value: 0,
        duration: 0,
        current: false,
    };

    tile_res.push(tile_1_0);
    tile_res.push(tile_1_1);
    tile_res.push(tile_1_2);
    tile_res.push(tile_2_0);
    tile_res.push(tile_2_1);
    tile_res.push(tile_2_2);
    tile_res.push(tile_3_0);
    tile_res.push(tile_3_1);
    tile_res.push(tile_3_2);
    tile_res.push(tile_4_0);
    tile_res.push(tile_4_1);
    tile_res.push(tile_4_2);
    tile_res.push(tile_5_0);
    tile_res.push(tile_5_1);
    tile_res.push(tile_5_2);
    tile_res.push(tile_6_0);
    tile_res.push(tile_6_1);
    tile_res.push(tile_6_2);
    tile_res.push(tile_7_0);
    tile_res.push(tile_7_1);
    tile_res.push(tile_7_2);
    tile_res.push(tile_8_0);
    tile_res.push(tile_8_1);
    tile_res.push(tile_8_2);
    tile_res.push(tile_9_0);
    tile_res.push(tile_9_1);
    tile_res.push(tile_9_2);

    tile_res
}
