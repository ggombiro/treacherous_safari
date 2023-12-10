use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds, TextLayoutInfo},
};
use rand::Rng;

use crate::{tiles::TileClosedEvent, ui::MovementPointsText};

const CARDS_TO_DRAW: u32 = 2;
const DRAWN_CARDS_START: f32 = 1400.0;
const DRAWN_CARDS_SPACE: f32 = 400.0;

#[derive(Resource)]
pub struct MovementPoints(pub i32);

#[derive(Event)]
pub struct MovementPointsUpdateEvent(pub i32);

#[derive(Event)]
pub struct DrawCardEvent;

#[derive(Event)]
pub struct MovementCardsDrawnEvent;

#[derive(Event)]
pub struct MovementCardsPlayedEvent;

#[derive(Component)]
pub struct MovementCardCover;

#[derive(Component)]
pub struct MovementCardDrawn;

#[derive(Component)]
pub struct MovementCardDiscarded;

#[derive(Component, Debug, Clone, Default)]
pub struct MovementCard {
    pub name: String,
    pub value: u32,
}

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

pub fn on_tile_closed_event(
    mut commands: Commands, 
    mut events: EventReader<TileClosedEvent>,
    mut playable_cards_query: Query<(Entity, &mut Transform , &mut MovementCard), 
    (Without<MovementCardDrawn>, Without<MovementCardDiscarded>)>,
    mut cards_drawn: EventWriter<MovementCardsDrawnEvent>,
) {
    let mut count = 0;

    for (entity, mut transform, mut card) in &mut playable_cards_query{
        if count == CARDS_TO_DRAW{
            break;
        }

        transform.translation.x -= (DRAWN_CARDS_START - (DRAWN_CARDS_SPACE * count as f32));

        let mut child = commands.spawn(MovementCardDrawn).id();

        commands.entity(entity).add_child(child);

        count += 1;
    }

    cards_drawn.send(MovementCardsDrawnEvent);
}

pub fn on_draw_card(
    mut commands: Commands, 
    mut events: EventReader<DrawCardEvent>,
    mut playable_cards_query: Query<(Entity, &mut Transform , &mut MovementCard), (Without<MovementCardDrawn>, Without<MovementCardDiscarded>)>
) {
    for (entity,mut transform, mut card) in &mut playable_cards_query{

        transform.translation.x -= (DRAWN_CARDS_START - (DRAWN_CARDS_SPACE * 2.0));

        let mut child = commands.spawn(MovementCardDrawn).id();

        commands.entity(entity).add_child(child);

        break;
    }
}


pub fn setup_movement_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let len = 300.0;
    let height = 450.0;
    let sprite_size = Some(Vec2::new(len, height));
    let mut card_res = generate_cards();

    commands
        .spawn((SpatialBundle {
            transform: Transform::from_scale(Vec3 {
                x: 0.4,
                y: 0.4,
                z: 1.0,
            }),
            ..default()
        },))
        .with_children(|commands| {
            const X_START: f32 = 1400.0;
            const Y_START: f32 = -600.0;

            let mut card_res_index: usize = 30;

            

            for _ in 0..15 {
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
                            texture: asset_server.load("cardBack_green1.png"),
                            transform: Transform::from_xyz(X_START, Y_START, -1.0),
                            ..default()
                        },
                        card.clone(),
                    ))
                    .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
                        parent.spawn(Text2dBundle {
                            text: Text::from_section(
                                format!("+{}", &card.name),
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
                        });

                        parent.spawn(Text2dBundle {
                            text: Text::from_section(
                                format!("+{}", &card.value),
                                TextStyle {
                                    font_size: 70.0,
                                    color: Color::rgb(1.0, 1.0, 0.2),
                                    ..default()
                                },
                            ),
                            transform: Transform {
                                translation: Vec3::new(0.0, 0.0, 1.0),
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
                                texture: asset_server.load("cardBack_green2.png"),
                                transform: Transform::from_xyz(0.0, 0.0, 1.1),
                                ..default()
                            },
                            MovementCardCover,
                        ));
                    });
            }

            if card_res_index < card_res.len() {
                card_res.remove(card_res_index);
            }
        });
}

pub fn generate_cards() -> Vec<MovementCard> {
    let mut card_res = Vec::with_capacity(2);

    let mut card_1 = MovementCard {
        name: String::from("Crawl"),
        value: 1,
    };

    let mut card_2 = MovementCard {
        name: String::from("Stride"),
        value: 2,
    };

    let mut card_3 = MovementCard {
        name: String::from("Brisk"),
        value: 3,
    };

    let mut card_4 = MovementCard {
        name: String::from("Dash"),
        value: 4,
    };

    let mut card_5 = MovementCard {
        name: String::from("Rush"),
        value: 5,
    };

    for _ in 0..5 {
        let mut card = card_1.clone();
        card_res.push(card);
    }

    for _ in 0..4 {
        let mut card = card_2.clone();
        card_res.push(card);
    }

    for _ in 0..3 {
        let mut card = card_3.clone();
        card_res.push(card);
    }

    for _ in 0..2 {
        let mut card = card_4.clone();
        card_res.push(card);
    }

    card_res.push(card_5);

    card_res
}
