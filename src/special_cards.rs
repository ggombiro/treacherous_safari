use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds, TextLayoutInfo},
};
use bevy_mod_picking::prelude::*;
use rand::Rng;

use crate::{tiles::TileClosedEvent, ui::MovementPointsText};

const CARDS_TO_DRAW: u32 = 8;
const FOCUS_SCALE: f32 = 0.1;
const SELECTED_SCALE: f32 = 2.0;
const BLOCKER_COLOR_VALUE: f32 = 0.1;

#[derive(Component)]
pub struct SpecialCardRevealBlocker;

#[derive(Component)]
pub struct SpecialCardRevealBlockerCloseButton;

#[derive(Event)]
pub struct MovementPointsUpdateEvent(pub i32);

#[derive(Event)]
pub struct DrawCardEvent;

#[derive(Event)]
pub struct SpecialCardPlayedEvent;

#[derive(Component)]
pub struct SpecialCardCover;

#[derive(Component)]
pub struct MovementCardDrawn;

#[derive(Component)]
pub struct MovementCardDiscarded;

#[derive(Component, Debug, Clone, Default)]
pub struct SpecialCard {
    pub name: String,
    pub tag: String,
    pub description: String,
    pub value: i32,
    pub card_type: CardType,
}

// pub fn on_special_card_closed_event(
//     mut commands: Commands, 
//     mut events: EventReader<TileClosedEvent>,
//     mut playable_cards_query: Query<(Entity, &mut Transform , &mut MovementCard), 
//     (Without<MovementCardDrawn>, Without<MovementCardDiscarded>)>,
//     mut cards_drawn: EventWriter<MovementCardsDrawnEvent>,
// ) {
//     let mut count = 0;

//     for (entity, mut transform, mut card) in &mut playable_cards_query{
//         if count == CARDS_TO_DRAW{
//             break;
//         }

//         transform.translation.x -= (DRAWN_CARDS_START - (DRAWN_CARDS_SPACE * count as f32));

//         let mut child = commands.spawn(MovementCardDrawn).id();

//         commands.entity(entity).add_child(child);

//         count += 1;
//     }

//     cards_drawn.send(MovementCardsDrawnEvent);
// }



pub fn setup_special_cards(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
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
            // On::<Pointer<Down>>::send_event::<SpecialCardSelected>(),
            // On::<Pointer<Over>>::target_component_mut::<Transform>(|_, transform| {
            //     if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
            //         transform.scale.x += FOCUS_SCALE;
            //         transform.scale.y += FOCUS_SCALE;
            //     }
            // }),
            // On::<Pointer<Out>>::target_component_mut::<Transform>(|_, transform| {
            //     if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
            //         transform.scale.x -= FOCUS_SCALE;
            //         transform.scale.y -= FOCUS_SCALE;
            //     }
            // }),
        ))
        .with_children(|commands| {
            const X_START: f32 = -1900.0;
            const X_STEP: f32 = 300.0;
            const Y_START: f32 = 0.0;
            const Y_STEP: f32 = 370.0;
            const SPACING: f32 = 150.0;

            let mut counter = 0;
            let mut card_res_index: usize = 30;

            for x in 0..4 {
                for y in 0..2 {

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
                                    (X_START + (x as f32 * X_STEP)) + (x as f32 * (SPACING/2.0)),
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
            // On::<Pointer<Click>>::send_event::<TileSelectedBlockerClose>(),
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

// pub fn on_special_card_selected(
//     mut commands: Commands,
//     mut events: EventReader<SpecialCardSelected>,
//     mut tiles: Query<(Entity, &mut Transform, &mut Tile)>,
//     mut blocker: Query<&mut Visibility, With<TileRevealBlocker>>,
//     mut close_button: Query<(
//         &mut Visibility,
//         &TileRevealBlockerCloseButton,
//         Without<TileRevealBlocker>,
//     )>,
//     mut visited_tiles: ResMut<VisitedTiles>,
// ) {
//     for (entity, mut transform, mut tile) in &mut tiles {
//         info!("Scale {:?} -> {}", transform.scale, FOCUS_SCALE + SELECTED_SCALE);
//         if transform.scale.x > 1.0 {
//             if transform.scale.x < FOCUS_SCALE + SELECTED_SCALE {
//                 transform.scale.x += SELECTED_SCALE;
//                 transform.scale.y += SELECTED_SCALE;
//                 transform.translation.z = 1.0;

//                 let mut blocker = blocker.single_mut();
//                 *blocker = Visibility::Visible;

//                 let mut close_button = close_button.single_mut();
//                 *close_button.0 = Visibility::Visible;

//                 if !visited_tiles.0.iter().any(|t| *t == tile.number){
//                     visited_tiles.0.push(tile.number);
//                 }
//             }
//         }
//     }
// }

#[derive(Debug, Clone, Default)]
pub enum CardType {
     #[default]DrawMovementCard,
    MovementPointsAddCard,
    MovementPointsAdd,
    TurnSub,
    TurnAdd,
    MovementPointsSubCard,
    MovementPointsSub,
    MovementPointsSubCardHighest,
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
        card_type: CardType::MovementPointsSubCardHighest,
    };

    let mut card_2 = SpecialCard {
        name: String::from("Motivity"),
        tag: String::from("<Nice>"),
        description: String::from("Double the movement points of the card in your hand with the least movement points"),
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
        value: 1,
        card_type: CardType::MovementPointsSub,
    };

    let mut card_5 = SpecialCard {
        name: String::from("Overdraw"),
        tag: String::from("<Naughty>"),
        description: String::from("Lose 2 movement points"),
        value: 2,
        card_type: CardType::MovementPointsSub,
    };

    let mut card_6 = SpecialCard {
        name: String::from("Tip the scales"),
        tag: String::from("<Nice>"),
        description: String::from("Add 2 movement points"),
        value: 2,
        card_type: CardType::MovementPointsAdd,
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
        card_type: CardType::TurnAdd,
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
        card_type: CardType::Erase
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
