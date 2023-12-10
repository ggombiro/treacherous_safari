use crate::movement::{
    MovementCardsDrawnEvent, MovementCardsPlayedEvent, MovementPoints, MovementPointsUpdateEvent,
};
use crate::turns::{TurnsLeft, TurnsUpdateEvent};
use crate::ui::{GameOverText, WonText};
use bevy::text::{BreakLineOn, Text2dBounds, TextLayoutInfo};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor};
use bevy_mod_picking::prelude::*;
use rand::Rng;

const FOCUS_SCALE: f32 = 0.1;
const SELECTED_SCALE: f32 = 2.0;
const BLOCKER_COLOR_VALUE: f32 = 0.1;
const MOVEMENT_POINTS_INIT_VALUE: i32 = 0;
const TURNS_INIT_VALUE: i32 = 7;

#[derive(Component, Debug, Clone, Default)]
pub struct Tile {
    pub cost: u32,
    pub description: String,
    pub number: u32,
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

#[derive(Event)]
pub struct TileClosedEvent;

#[derive(Resource)]
pub struct VisitedTiles(pub Vec<u32>);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct TileHighlight(pub Entity);

#[derive(Component)]
pub struct TileCostText;

#[derive(Component)]
pub struct TileDescriptionText;

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
            On::<Pointer<Over>>::send_event::<OverTile>(),
            On::<Pointer<Out>>::send_event::<OffTile>(),
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

                    // tile.current = if counter == 0 { true } else { false };

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

                            parent.spawn((
                                Text2dBundle {
                                    text: Text::from_section(
                                        format!("{}", &tile.cost),
                                        TextStyle {
                                            font_size: 70.0,
                                            color: Color::rgb(1.0, 1.0, 0.2),
                                            ..default()
                                        },
                                    ),
                                    transform: Transform {
                                        translation: Vec3::new(0.0, 140.0, 1.0),
                                        ..default()
                                    },
                                    text_anchor: Anchor::TopCenter,
                                    ..default()
                                },
                                TileCostText,
                            ));

                            let other_box_size = Vec2::new(190.0, 350.0);

                            parent.spawn((
                                Text2dBundle {
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
                                },
                                TileDescriptionText,
                            ));

                            parent.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: sprite_size,
                                        ..default()
                                    },
                                    texture: asset_server.load("cardBack_blue2.png"),
                                    transform: Transform::from_xyz(0.0, 0.0, 1.1),
                                    ..default()
                                },
                                TileCover,
                                Pickable::IGNORE,
                            ));

                            let highlight_size = Some(Vec2::new(len + 20.0, height + 20.0));

                            parent
                                .spawn((
                                    SpatialBundle {
                                        transform: Transform::from_xyz(0.0, 0.0, -1.1),
                                        visibility: Visibility::Hidden,
                                        ..Default::default()
                                    },
                                    TileHighlight(parent.parent_entity()),
                                    Pickable::IGNORE,
                                ))
                                .with_children(|commands| {
                                    commands.spawn((SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: highlight_size,
                                            color: Color::FUCHSIA,
                                            ..default()
                                        },
                                        // texture: asset_server.load("images/boovy.png"),
                                        ..default()
                                    },));
                                });
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
                    top: Val::Percent(10.0),
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

    let len = 64.0 * 2.5;
    let height = 64.0 * 2.5;
    let piece_size = Some(Vec2::new(len, height));

    commands
        .spawn((SpatialBundle {
            transform: Transform::from_scale(Vec3 {
                x: 0.3,
                y: 0.3,
                z: 1.0,
            }),
            ..default()
        },))
        .with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: piece_size,
                        // color: Color::BLACK,
                        ..default()
                    },
                    texture: asset_server.load("pieceYellow_border01.png"),
                    transform: Transform::from_xyz(-256.0, 160.0, 0.0),
                    ..default()
                },
                Player,
                Pickable::IGNORE,
            ));
        });

    let len = 64.0 * 4.0;
    let height = 64.0 * 3.0;
    let piece_size = Some(Vec2::new(len, height));

    commands
        .spawn((SpatialBundle {
            transform: Transform::from_scale(Vec3 {
                x: 0.3,
                y: 0.3,
                z: 1.0,
            }),
            ..default()
        },))
        .with_children(|commands| {
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    custom_size: piece_size,
                    // color: Color::BLACK,
                    ..default()
                },
                texture: asset_server.load("pieceYellow_border12.png"),
                transform: Transform::from_xyz(1300.0, 240.0, 0.0),
                ..default()
            },));
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
    mut tiles: Query<(Entity, &mut Transform, &mut Tile, &Children)>,
    mut blocker: Query<&mut Visibility, With<TileRevealBlocker>>,
    mut close_button: Query<(
        &mut Visibility,
        &TileRevealBlockerCloseButton,
        Without<TileRevealBlocker>,
    )>,
    mut highlightables: Query<
        (&mut Visibility, &mut TileHighlight),
        (
            Without<TileRevealBlocker>,
            Without<TileRevealBlockerCloseButton>,
        ),
    >,
    mut tile_cover_query: Query<
        &mut Visibility,
        (
            With<TileCover>,
            Without<TileHighlight>,
            Without<TileRevealBlocker>,
            Without<TileRevealBlockerCloseButton>,
        ),
    >,
) {
    for (entity, mut transform, mut tile, mut children) in &mut tiles {
        if transform.scale.x > 1.0 {
            if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
                transform.scale.x += SELECTED_SCALE;
                transform.scale.y += SELECTED_SCALE;
                transform.translation.z = 1.0;

                let mut blocker = blocker.single_mut();
                *blocker = Visibility::Visible;

                let mut close_button = close_button.single_mut();
                *close_button.0 = Visibility::Visible;

                for (mut vis, mut highlight) in &mut highlightables {
                    if highlight.0 == entity {
                        *vis = Visibility::Hidden;
                    }
                }

                for child in children {
                    if let Ok(mut vis) = tile_cover_query.get_mut(*child) {
                        *vis = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct OverTile(Entity);

impl From<ListenerInput<Pointer<Over>>> for OverTile {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        OverTile(event.target)
    }
}

pub fn on_over_tile(
    mut commands: Commands,
    mut events: EventReader<OverTile>,
    mut tiles: Query<(Entity, &mut Transform), With<Selectable>>,
) {
    for ev in events.read() {
        for (entity, mut transform) in &mut tiles {
            if entity == ev.0 {
                if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
                    transform.scale.x += FOCUS_SCALE;
                    transform.scale.y += FOCUS_SCALE;
                }
            }
        }
    }
}

#[derive(Event)]
pub struct OffTile(Entity);

impl From<ListenerInput<Pointer<Out>>> for OffTile {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        OffTile(event.target)
    }
}

pub fn on_off_tile(
    mut commands: Commands,
    mut events: EventReader<OffTile>,
    mut tiles: Query<(Entity, &mut Transform), With<Selectable>>,
) {
    for ev in events.read() {
        for (entity, mut transform) in &mut tiles {
            if entity == ev.0 {
                if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
                    transform.scale.x -= FOCUS_SCALE;
                    transform.scale.y -= FOCUS_SCALE;
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
    mut tiles: Query<(Entity, &mut Transform, &mut Tile)>,
    mut blocker: Query<&mut Visibility, With<TileRevealBlocker>>,
    mut close_button: Query<(
        &mut Visibility,
        &TileRevealBlockerCloseButton,
        Without<TileRevealBlocker>,
    )>,
    mut player_query: Query<(&mut Transform, &Player), Without<Tile>>,
    mut visited_tiles: ResMut<VisitedTiles>,
    mut movement_points_update: EventWriter<MovementPointsUpdateEvent>,
    mut turns_update: EventWriter<TurnsUpdateEvent>,
    mut tile_closed: EventWriter<TileClosedEvent>,
) {
    let mut player = player_query.single_mut();

    let mut tile_clone = Tile { ..default() };

    for (entity, mut transform, mut tile) in &mut tiles {
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
            let mut diff = transform.translation - player.0.translation;

            player.0.translation.x += diff.x;
            player.0.translation.y += diff.y;

            tile_clone = tile.clone();

            tile.current = true;
            // commands.entity(entity).insert(Selectable);
        }
    }

    if tile_clone.current {
        return;
    }

    //check for add entity

    match tile_clone.tile_type {
        TileType::Plain => {}
        TileType::MovementPointsUpdate => {
            movement_points_update.send(MovementPointsUpdateEvent(tile_clone.value));

            if tile_clone.duration < 0 {
                //create or add to movement curse comp
            } else if tile_clone.duration > 0 {
            }
        }
        TileType::TurnUpdate => {
            turns_update.send(TurnsUpdateEvent(tile_clone.value));
        }
        TileType::Blessing => {
            //remove ailments components
        }
    }

    tile_closed.send(TileClosedEvent);
}

#[derive(Debug, Clone, Default)]
pub enum TileType {
    #[default]
    Plain,
    MovementPointsUpdate,
    TurnUpdate,
    Blessing,
}

pub fn get_neighbours(index: u32) -> Vec<u32> {
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
        _ => vec![0],
    }
}

pub fn on_tile_setup_complete(
    mut commands: Commands,
    mut events: EventReader<TileSetupComplete>,
    mut tiles: Query<(Entity, &mut Transform, &mut Tile, &Children)>,
    mut tile_selected: EventWriter<TileSelected>,
    mut tile_cover_query: Query<&mut Visibility, With<TileCover>>,
    mut movement_points_update: EventWriter<MovementPointsUpdateEvent>,
    mut turns_update: EventWriter<TurnsUpdateEvent>,
) {
    movement_points_update.send(MovementPointsUpdateEvent(MOVEMENT_POINTS_INIT_VALUE));
    turns_update.send(TurnsUpdateEvent(TURNS_INIT_VALUE));

    for (entity, mut transform, mut tile, mut children) in &mut tiles {
        if tile.number == 0 {
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

pub fn on_turn_ended(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Tile)>,
    mut highlightables: Query<(&mut Visibility, &mut TileHighlight)>,
    mut events: EventReader<MovementCardsPlayedEvent>,
    mut movement_points: ResMut<MovementPoints>,
    mut tile_closed: EventWriter<TileClosedEvent>,
    mut turns_update: EventWriter<TurnsUpdateEvent>,
    mut turns_left: ResMut<TurnsLeft>,
    mut game_over: Query<&mut Visibility, (With<GameOverText>, Without<WonText>, Without<TileHighlight>)>,
    mut game_won: Query<&mut Visibility, (With<WonText>, Without<GameOverText>, Without<TileHighlight>)>,
) {
    turns_update.send(TurnsUpdateEvent(-1));

    if turns_left.0 <= 0{
        let mut vis = game_over.single_mut();

        *vis = Visibility::Visible;
        return;
    }

    let mut costs_met = false;

    for (entity, mut tile) in &mut query {
        if tile.current {
            costs_met = if tile.cost as i32 <= movement_points.0 {
                true
            } else {
                false
            };

            if costs_met && tile.number == 10{
                let mut vis = game_won.single_mut();

                *vis = Visibility::Visible;
            }
        }
    }

    if costs_met {

        let mut tile_number = 0;

        for (entity, mut tile) in &mut query {
            commands.entity(entity).remove::<Selectable>();

            for (mut vis, mut highlight) in &mut highlightables {
                if highlight.0 == entity {
                    *vis = Visibility::Hidden;
                }
            }

            if tile.current {
                tile.current = false;
                tile_number = tile.number;
            }
        }

        let mut neighbours = get_neighbours(tile_number);

        for (entity, mut tile) in &mut query {
            if neighbours.contains(&tile.number) {
                commands.entity(entity).insert(Selectable);

                for (mut vis, mut highlight) in &mut highlightables {
                    if highlight.0 == entity {
                        *vis = Visibility::Visible;
                    }
                }
            }
        }
    } else {
        tile_closed.send(TileClosedEvent);
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
        tile_type: TileType::MovementPointsUpdate,
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
        tile_type: TileType::MovementPointsUpdate,
        value: 1,
        duration: 0,
        current: false,
    };

    let mut tile_1_2 = Tile {
        cost: 1,
        description: String::from("Lion fight. You survive. Lose 2 movement points."),
        number: 0,
        tile_type: TileType::MovementPointsUpdate,
        value: -2,
        duration: 0,
        current: false,
    };

    let mut tile_2_0 = Tile {
        cost: 2,
        description: String::from("River! You have to swim across, lose 1 turn."),
        number: 0,
        tile_type: TileType::TurnUpdate,
        value: -1,
        duration: 0,
        current: false,
    };

    let mut tile_2_1 = Tile {
        cost: 2,
        description: String::from(
            "Village with nice people. They give you a reed bed. You sleep well. Remove ailments or curses.",
        ),
        number: 0,
        tile_type: TileType::Blessing,
        value: 0,
        duration: 0,
        current: false,
    };

    let mut tile_2_2 = Tile {
        cost: 2,
        description: String::from("Cool breeze. Adventure time."),
        number: 0,
        tile_type: TileType::Plain,
        value: 0,
        duration: 0,
        current: false,
    };

    // let mut tile_3_0 = Tile {
    //     cost: 3,
    //     description: String::from("Native guides you. Go up or down 1 tile."),
    //     number: 0,
    //     neighbours: vec![-1],
    //     tile_type: TileType::RouteRestriction,
    //     value: 0,
    //     duration: 0,
    //     current: false,
    // };

    let mut tile_3_1 = Tile {
        cost: 3,
        description: String::from(
            "Snake bite. You are poisoned. Lose 1 movement point for 2 turns.",
        ),
        number: 0,
        tile_type: TileType::MovementPointsUpdate,
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
        tile_type: TileType::MovementPointsUpdate,
        value: 1,
        duration: 0,
        current: false,
    };

    let mut tile_4_0 = Tile {
        cost: 4,
        description: String::from("River! You have to swim across, lose 1 turn."),
        number: 0,
        tile_type: TileType::TurnUpdate,
        value: -1,
        duration: 0,
        current: false,
    };

    let mut tile_4_1 = Tile {
        cost: 4,
        description: String::from(
            "You find ruins with a shiny object. \"Stop!\" You touch it and get cursed. Lose 2 movement points each turn.",
        ),
        number: 0,
        tile_type: TileType::MovementPointsUpdate,
        value: -2,
        duration: -1,
        current: false,
    };

    let mut tile_4_2 = Tile {
        cost: 4,
        description: String::from(
            "Village with nice people. They give you a reed bed. You sleep well. Remove ailments or curses.",
        ),
        number: 0,
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
        tile_type: TileType::MovementPointsUpdate,
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
        tile_type: TileType::MovementPointsUpdate,
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
        tile_type: TileType::MovementPointsUpdate,
        value: 2,
        duration: 0,
        current: false,
    };

    let mut tile_6_0 = Tile {
        cost: 6,
        description: String::from("Lion fight. You survive. Lose 2 movement points."),
        number: 0,
        tile_type: TileType::MovementPointsUpdate,
        value: -2,
        duration: 0,
        current: false,
    };

    let mut tile_6_1 = Tile {
        cost: 6,
        description: String::from("Cool breeze. Adventure time."),
        number: 0,
        tile_type: TileType::Plain,
        value: 0,
        duration: 0,
        current: false,
    };

    // let mut tile_6_2 = Tile {
    //     cost: 6,
    //     description: String::from("Native guides you. Go forward 1 tile."),
    //     number: 0,
    //     neighbours: vec![-1],
    //     tile_type: TileType::RouteRestriction,
    //     value: 0,
    //     duration: 0,
    //     current: false,
    // };

    let mut tile_7_0 = Tile {
        cost: 7,
        description: String::from("River! You have to swim across, lose 1 turn."),
        number: 0,
        tile_type: TileType::TurnUpdate,
        value: -1,
        duration: 0,
        current: false,
    };

    let mut tile_7_1 = Tile {
        cost: 7,
        description: String::from(
            "Village with nice people. They give you a reed bed. You sleep well. Remove ailments or curses.",
        ),
        number: 0,
        tile_type: TileType::Blessing,
        value: -1,
        duration: 0,
        current: false,
    };

    let mut tile_7_2 = Tile {
        cost: 7,
        description: String::from("Lion fight. You survive. Lose 2 movement points."),
        number: 0,
        tile_type: TileType::MovementPointsUpdate,
        value: -2,
        duration: 0,
        current: false,
    };

    let mut tile_8_0 = Tile {
        cost: 8,
        description: String::from(
            "You befriend an elephant. You ride on its back. Gain 2 movement points.",
        ),
        number: 0,
        tile_type: TileType::MovementPointsUpdate,
        value: 2,
        duration: 0,
        current: false,
    };

    // let mut tile_8_1 = Tile {
    //     cost: 8,
    //     description: String::from("Native guides you. Go up or down 1 tile."),
    //     number: 0,
    //     neighbours: vec![-1],
    //     tile_type: TileType::RouteRestriction,
    //     value: 0,
    //     duration: 0,
    //     current: false,
    // };

    let mut tile_8_2 = Tile {
        cost: 8,
        description: String::from(
            "Snake bite. You are poisoned. Lose 1 movement point for 2 turns.",
        ),
        number: 0,
        tile_type: TileType::MovementPointsUpdate,
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
        tile_type: TileType::MovementPointsUpdate,
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
        tile_type: TileType::MovementPointsUpdate,
        value: 2,
        duration: 0,
        current: false,
    };

    let mut tile_9_2 = Tile {
        cost: 9,
        description: String::from("Cool breeze. Adventure time."),
        number: 0,
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
    // tile_res.push(tile_3_0);
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
    // tile_res.push(tile_6_2);
    tile_res.push(tile_7_0);
    tile_res.push(tile_7_1);
    tile_res.push(tile_7_2);
    tile_res.push(tile_8_0);
    // tile_res.push(tile_8_1);
    tile_res.push(tile_8_2);
    tile_res.push(tile_9_0);
    tile_res.push(tile_9_1);
    tile_res.push(tile_9_2);

    tile_res
}
