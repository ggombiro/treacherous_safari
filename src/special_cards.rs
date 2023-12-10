use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds, TextLayoutInfo},
};
use bevy_mod_picking::prelude::*;
use rand::Rng;

use crate::{
    movement::{
        CardPointsText, DrawCardEvent, MovementCard, MovementCardDiscarded, MovementCardDrawn,
        MovementCardsDrawnEvent, MovementPointsUpdateEvent,
    },
    tiles::{Tile, TileClosedEvent, TileCostText, TileDescriptionText, TileType},
    turns::TurnsUpdateEvent,
    ui::MovementPointsText,
};

const CARDS_TO_DRAW: u32 = 8;
const FOCUS_SCALE: f32 = 0.1;
const SELECTED_SCALE: f32 = 1.4;
const BLOCKER_COLOR_VALUE: f32 = 0.1;

const X_START: f32 = -1700.0;
const X_STEP: f32 = 300.0;
const Y_START: f32 = -200.0;
const Y_STEP: f32 = 370.0;
const SPACING: f32 = 150.0;

const X_FINAL: f32 = -1300.0;
const Y_FINAL: f32 = -800.0;
const FINAL_SCALE: f32 = 1.3;

#[derive(Component)]
pub struct SpecialCardRevealBlocker;

#[derive(Component)]
pub struct SpecialCardRevealBlockerCloseButton;

#[derive(Event)]
pub struct SpecialCardPlayedEvent;

#[derive(Component)]
pub struct SpecialCardCover;

#[derive(Component)]
pub struct SpecialCardDiscarded;

#[derive(Component)]
pub struct SpecialCardSelectable;

#[derive(Event)]
pub struct SpecialCardClosed;

#[derive(Component)]
pub struct SpecialCardHighlight(pub Entity);

#[derive(Component, Debug, Clone, Default)]
pub struct SpecialCard {
    pub name: String,
    pub tag: String,
    pub description: String,
    pub value: i32,
    pub card_type: CardType,
}

pub fn setup_special_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let len = 300.0;
    let height = 450.0;
    let sprite_size = Some(Vec2::new(len, height));
    let mut card_res = generate_cards();

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
            On::<Pointer<Down>>::send_event::<SpecialCardSelected>(),
            On::<Pointer<Over>>::send_event::<OverSpecialCard>(),
            On::<Pointer<Out>>::send_event::<OffSpecialCard>(),
        ))
        .with_children(|commands| {
            let mut counter = 0;
            let mut card_res_index: usize = 30;

            for x in 0..4 {
                for y in 0..2 {
                    if card_res.len() == 0 {
                        continue;
                    }

                    let mut rng = rand::thread_rng();

                    card_res_index = rng.gen_range(0..card_res.len());
                    let card = &mut card_res[card_res_index];

                    commands
                        .spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: sprite_size,
                                    // color: Color::BLACK,
                                    ..default()
                                },
                                texture: asset_server.load("cardBack_red1.png"),
                                transform: Transform::from_xyz(
                                    (X_START + (x as f32 * X_STEP)) + (x as f32 * (SPACING / 2.0)),
                                    (Y_START + (y as f32 * Y_STEP)) + (y as f32 * (SPACING)),
                                    -1.0,
                                ),
                                ..default()
                            },
                            card.clone(),
                        ))
                        .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
                            parent.spawn(Text2dBundle {
                                text: Text::from_section(
                                    format!("{}", &card.name),
                                    TextStyle {
                                        font_size: 30.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ),
                                transform: Transform {
                                    translation: Vec3::new(0.0, 200.0, 1.0),
                                    ..default()
                                },
                                text_anchor: Anchor::TopCenter,
                                ..default()
                            });

                            parent.spawn(Text2dBundle {
                                text: Text::from_section(
                                    format!("{}", &card.tag),
                                    TextStyle {
                                        font_size: 30.0,
                                        color: Color::rgb(1.0, 1.0, 0.2),
                                        ..default()
                                    },
                                ),
                                transform: Transform {
                                    translation: Vec3::new(0.0, 150.0, 1.0),
                                    ..default()
                                },
                                text_anchor: Anchor::TopCenter,
                                ..default()
                            });

                            let other_box_size = Vec2::new(190.0, 350.0);

                            parent.spawn(Text2dBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        format!("{}", &card.description),
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
                                    texture: asset_server.load("cardBack_red2.png"),
                                    transform: Transform::from_xyz(0.0, 0.0, 1.1),
                                    ..default()
                                },
                                SpecialCardCover,
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
                                    SpecialCardHighlight(parent.parent_entity()),
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

                if card_res_index < card_res.len() {
                    card_res.remove(card_res_index);
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
            SpecialCardRevealBlocker,
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
            On::<Pointer<Click>>::send_event::<SpecialCardSelectedBlockerClose>(),
            NoDeselect,
            SpecialCardRevealBlockerCloseButton,
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
}

#[derive(Event)]
pub struct SpecialCardSelected(Entity);

impl From<ListenerInput<Pointer<Down>>> for SpecialCardSelected {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        SpecialCardSelected(event.target)
    }
}

pub fn on_special_card_selected(
    mut commands: Commands,
    mut events: EventReader<SpecialCardSelected>,
    mut cards: Query<(Entity, &mut Transform, &mut SpecialCard, &Children), Without<SpecialCardDiscarded>>,
    mut card_cover_query: Query<
        &mut Visibility,
        (
            With<SpecialCardCover>,
            Without<SpecialCardRevealBlocker>,
            Without<SpecialCardRevealBlockerCloseButton>,
        ),
    >,
    mut blocker: Query<&mut Visibility, With<SpecialCardRevealBlocker>>,
    mut close_button: Query<(
        &mut Visibility,
        &SpecialCardRevealBlockerCloseButton,
        Without<SpecialCardRevealBlocker>,
    )>,
    mut highlightables: Query<
        (&mut Visibility, &mut SpecialCardHighlight),
        (
            Without<SpecialCardRevealBlocker>,
            Without<SpecialCardRevealBlockerCloseButton>,
            Without<SpecialCardCover>,
        ),
    >,
) {
    for (entity, mut transform, mut tile, mut children) in &mut cards {
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
                    if let Ok(mut vis) = card_cover_query.get_mut(*child) {
                        *vis = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct OverSpecialCard(Entity);

impl From<ListenerInput<Pointer<Over>>> for OverSpecialCard {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        OverSpecialCard(event.target)
    }
}

pub fn on_over_special_card(
    mut commands: Commands,
    mut events: EventReader<OverSpecialCard>,
    mut tiles: Query<(Entity, &mut Transform), With<SpecialCardSelectable>>,
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
pub struct OffSpecialCard(Entity);

impl From<ListenerInput<Pointer<Out>>> for OffSpecialCard {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        OffSpecialCard(event.target)
    }
}

pub fn on_off_special_card(
    mut commands: Commands,
    mut events: EventReader<OffSpecialCard>,
    mut tiles: Query<(Entity, &mut Transform), With<SpecialCardSelectable>>,
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

pub fn on_movement_cards_drawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpecialCard), Without<SpecialCardDiscarded>>,
    mut highlightables: Query<(&mut Visibility, &mut SpecialCardHighlight)>,
    mut events: EventReader<MovementCardsDrawnEvent>, //should listen to turn ended event
) {
    let mut tile_number = 0;

    for (entity, mut card) in &mut query {
        commands.entity(entity).insert(SpecialCardSelectable);

        for (mut vis, mut highlight) in &mut highlightables {
            if highlight.0 == entity {
                *vis = Visibility::Visible;
            }
        }
    }
}

#[derive(Clone, Event)]
pub struct SpecialCardSelectedBlockerClose(Entity);

impl From<ListenerInput<Pointer<Click>>> for SpecialCardSelectedBlockerClose {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        SpecialCardSelectedBlockerClose(event.target)
    }
}

pub fn selected_special_card_close(
    mut commands: Commands,
    mut events: EventReader<SpecialCardSelectedBlockerClose>,
    mut cards: Query<(Entity, &mut Transform, &mut SpecialCard), Without<MovementCard>>,
    mut blocker: Query<&mut Visibility, With<SpecialCardRevealBlocker>>,
    mut close_button: Query<(
        &mut Visibility,
        &SpecialCardRevealBlockerCloseButton,
        Without<SpecialCardRevealBlocker>,
    )>,
    mut special_card_closed: EventWriter<SpecialCardClosed>,
    mut highlightables: Query<
        (&mut Visibility, &mut SpecialCardHighlight),
        (
            Without<SpecialCardRevealBlocker>,
            Without<SpecialCardRevealBlockerCloseButton>,
        ),
    >,
    mut draw_card_event: EventWriter<DrawCardEvent>,
    mut movement_points_update: EventWriter<MovementPointsUpdateEvent>,
    mut turns_update: EventWriter<TurnsUpdateEvent>,
    mut tiles: Query<
        (Entity, &mut Transform, &mut Tile, &Children),
        (
            Without<SpecialCard>,
            Without<MovementCard>,
            Without<MovementCardDrawn>,
        ),
    >,
    mut tile_cost_texts: Query<&mut Text, With<TileCostText>>,
    mut tile_desc_texts: Query<
        &mut Text,
        (
            With<TileDescriptionText>,
            Without<TileCostText>,
            Without<CardPointsText>,
        ),
    >,
    mut card_points_texts: Query<&mut Text, (With<CardPointsText>, Without<TileCostText>)>,
    mut drawn_cards_query: Query<
        (Entity, &mut Transform, &mut MovementCard, &Children),
        (
            With<MovementCardDrawn>,
            Without<MovementCardDiscarded>,
            Without<Tile>,
        ),
    >,
) {
    let mut card_clone = SpecialCard { ..default() };

    for (entity, mut transform, mut card) in &mut cards {
        commands.entity(entity).remove::<SpecialCardSelectable>();

        for (mut vis, mut highlight) in &mut highlightables {
            if highlight.0 == entity {
                *vis = Visibility::Hidden;
            }
        }

        if transform.scale.x > SELECTED_SCALE {
            transform.scale.x = FINAL_SCALE;
            transform.scale.y = FINAL_SCALE;
            transform.translation.z = -1.0;

            let mut blocker = blocker.single_mut();
            *blocker = Visibility::Hidden;

            let mut close_button = close_button.single_mut();
            *close_button.0 = Visibility::Hidden;

            info!("Tile: {:?}", card);

            commands.entity(entity).insert(SpecialCardDiscarded);

            let diff_x = X_FINAL - transform.translation.x;
            let diff_y = Y_FINAL - transform.translation.y;

            transform.translation.x += diff_x;
            transform.translation.y += diff_y;

            let mut rng = rand::thread_rng();

            transform.rotate_z(rng.gen_range(-0.1..=0.1));

            card_clone = card.clone();
        }
    }

    //check for add entity

    match card_clone.card_type {
        CardType::DrawMovementCard => {
            draw_card_event.send(DrawCardEvent(card_clone.value as u32));
        }
        CardType::MovementPointsUpdate => {
            movement_points_update.send(MovementPointsUpdateEvent(card_clone.value));
        }
        CardType::TurnUpdate => {
            turns_update.send(TurnsUpdateEvent(card_clone.value));
        }
        CardType::MovementPointsSubHighest => {
            let mut m_card = drawn_cards_query
                .iter()
                .map(|mut m| (m.0, m.2.value))
                .max_by_key(|&m| m.1)
                .unwrap();

            for (entity, _, mut movement_card, mut children) in &mut drawn_cards_query {
                if entity == m_card.0 {
                    movement_card.value -= card_clone.value as u32;
                }

                for child in children {
                    if let Ok(mut text) = card_points_texts.get_mut(*child) {
                        text.sections[0].value = format!("+{}", movement_card.value);
                    }
                }
            }
        }
        CardType::CurrentTileCostDirectChange => {
            for (_, _, mut tile, mut children) in &mut tiles {
                if tile.current {
                    tile.cost = card_clone.value as u32;

                    for child in children {
                        if let Ok(mut text) = tile_cost_texts.get_mut(*child) {
                            text.sections[0].value = format!("{}", tile.cost);
                        }
                    }
                    break;
                }
            }
        }
        CardType::CurrentTileCostIndirectChange => {
            for (_, _, mut tile, mut children) in &mut tiles {
                if tile.current {
                    tile.cost += card_clone.value as u32;

                    for child in children {
                        if let Ok(mut text) = tile_cost_texts.get_mut(*child) {
                            text.sections[0].value = format!("{}", tile.cost);
                        }
                    }
                    break;
                }
            }
        }
        CardType::MovementPointsMultiplyLeastCard => {
            let mut m_card = drawn_cards_query
                .iter()
                .map(|mut m| (m.0, m.2.value))
                .min_by_key(|&m| m.1)
                .unwrap();

            for (entity, _, mut movement_card, mut children) in &mut drawn_cards_query {
                if entity == m_card.0 {
                    movement_card.value *= card_clone.value as u32;
                }

                for child in children {
                    if let Ok(mut text) = card_points_texts.get_mut(*child) {
                        text.sections[0].value = format!("+{}", movement_card.value);
                    }
                }
            }
        }
        CardType::MovementPointsReductionAllCards => {
            for (_, _, mut movement_card, mut children) in &mut drawn_cards_query {
                movement_card.value = card_clone.value as u32;

                for child in children {
                    if let Ok(mut text) = card_points_texts.get_mut(*child) {
                        text.sections[0].value = format!("+{}", movement_card.value);
                    }
                }
            }
        }
        CardType::Erase => {
            for (_, _, mut tile, mut children) in &mut tiles {
                if tile.current {
                    tile.cost += card_clone.value as u32;
                    tile.tile_type = TileType::Plain;

                    for child in children {
                        if let Ok(mut text) = tile_cost_texts.get_mut(*child) {
                            text.sections[0].value = format!("{}", tile.cost);
                        }
                    }

                    for child in children {
                        if let Ok(mut text) = tile_desc_texts.get_mut(*child) {
                            text.sections[0].value = format!("Erased!");
                        }
                    }
                    break;
                }
            }
        }
        _ => {
            info!("Else")
        }
    }

    special_card_closed.send(SpecialCardClosed);
}

#[derive(Debug, Clone, Default)]
pub enum CardType {
    #[default]
    DrawMovementCard,
    MovementPointsUpdate,
    TurnUpdate,
    MovementPointsSubHighest,
    CurrentTileCostDirectChange,
    CurrentTileCostIndirectChange,
    MovementPointsMultiplyLeastCard,
    MovementPointsReductionAllCards,
    Erase,
}

pub fn generate_cards() -> Vec<SpecialCard> {
    let mut card_res = Vec::with_capacity(2);

    let mut card_1 = SpecialCard {
        name: String::from("Scathe"),
        tag: String::from("<Naughty>"),
        description: String::from("Take off 2 movement points from movement card with the highest movement points in your hand"),
        value: 2,
        card_type: CardType::MovementPointsSubHighest,
    };

    let mut card_2 = SpecialCard {
        name: String::from("Motivity"),
        tag: String::from("<Nice>"),
        description: String::from(
            "Double the movement points of the card with the least points in your hand",
        ),
        value: 2,
        card_type: CardType::MovementPointsMultiplyLeastCard,
    };

    let mut card_3 = SpecialCard {
        name: String::from("Lucky Draw"),
        tag: String::from("<Nice>"),
        description: String::from("Draw another movement card"),
        value: 1,
        card_type: CardType::DrawMovementCard,
    };

    let mut card_4 = SpecialCard {
        name: String::from("Aw Snap!"),
        tag: String::from("<Naughty>"),
        description: String::from("Lose 1 movement point"),
        value: -1,
        card_type: CardType::MovementPointsUpdate,
    };

    let mut card_5 = SpecialCard {
        name: String::from("Overdraw"),
        tag: String::from("<Naughty>"),
        description: String::from("Lose 2 movement points"),
        value: -2,
        card_type: CardType::MovementPointsUpdate,
    };

    let mut card_6 = SpecialCard {
        name: String::from("Tip the scales"),
        tag: String::from("<Nice>"),
        description: String::from("Add 2 movement points"),
        value: 2,
        card_type: CardType::MovementPointsUpdate,
    };

    let mut card_7 = SpecialCard {
        name: String::from("Paid For"),
        tag: String::from("<Nice>"),
        description: String::from("Change the cost of the current tile to 0"),
        value: 0,
        card_type: CardType::CurrentTileCostDirectChange,
    };

    let mut card_8 = SpecialCard {
        name: String::from("Mutator"),
        tag: String::from("<Naughty or Nice>"),
        description: String::from("Randomly mutate the cost of the current tile"),
        value: -1,
        card_type: CardType::CurrentTileCostDirectChange,
    };

    let mut card_9 = SpecialCard {
        name: String::from("Arduous"),
        tag: String::from("<Naughty>"),
        description: String::from("Change the cost of the current tile to 10"),
        value: 10,
        card_type: CardType::CurrentTileCostDirectChange,
    };

    let mut card_10 = SpecialCard {
        name: String::from("Twice Lucky?"),
        tag: String::from("<Naughty or Nice>"),
        description: String::from("Discard current hand and draw new cards"),
        value: 2,
        card_type: CardType::DrawMovementCard,
    };

    let mut card_11 = SpecialCard {
        name: String::from("Torpid"),
        tag: String::from("<Naughty>"),
        description: String::from("Reduce all movement points of the cards in your hand to 1"),
        value: 1,
        card_type: CardType::MovementPointsReductionAllCards,
    };

    let mut card_12 = SpecialCard {
        name: String::from("Second Chance"),
        tag: String::from("<Nice>"),
        description: String::from("Add a turn"),
        value: 1,
        card_type: CardType::TurnUpdate,
    };

    let mut card_13 = SpecialCard {
        name: String::from("Inflation"),
        tag: String::from("<Naughty>"),
        description: String::from("Increase the current tile cost by 1"),
        value: 1,
        card_type: CardType::CurrentTileCostIndirectChange,
    };

    let mut card_14 = SpecialCard {
        name: String::from("Erase"),
        tag: String::from("<Nice>"),
        description: String::from("Erase the cost and effects of the current tile"),
        value: 0,
        card_type: CardType::Erase,
    };

    card_res.push(card_1);
    card_res.push(card_2);
    card_res.push(card_3);
    card_res.push(card_4);
    card_res.push(card_5);
    card_res.push(card_6);
    card_res.push(card_7);
    card_res.push(card_8);
    card_res.push(card_9);
    card_res.push(card_10);
    card_res.push(card_11);
    card_res.push(card_12);
    card_res.push(card_13);
    card_res.push(card_14);

    card_res
}
