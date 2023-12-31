use bevy::{prelude::*, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use bevy::{app::AppExit, log::LogPlugin};
use tiles::{setup_tiles, on_tile_selected, tile_selected_close, on_tile_setup_complete,
    TileSelected, TileSelectedBlockerClose, TileSetupComplete, VisitedTiles, TileClosedEvent, OverTile, 
    on_over_tile, on_turn_ended, OffTile, on_off_tile};
use movement::{MovementPoints, update_movement_points, MovementPointsUpdateEvent, 
    on_tile_closed_event, setup_movement_cards, DrawCardEvent, on_draw_card, MovementCardsDrawnEvent, on_special_card_closed_event, MovementCardsPlayedEvent};
use game_state::{GameState, GameStates};
use turns::{TurnsLeft, TurnsUpdateEvent, update_turns_left};
use ui::setup_game_ui;
use special_cards::{setup_special_cards, on_movement_cards_drawn, SpecialCardPlayedEvent, SpecialCardSelected, 
    OverSpecialCard, OffSpecialCard, on_over_special_card, on_off_special_card, on_special_card_selected, 
    SpecialCardSelectedBlockerClose, selected_special_card_close, SpecialCardClosed};


mod game_state;
mod movement;
mod tiles;
mod turns;
mod ui;
mod special_cards;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
            .set(ImagePlugin::default_linear())
            .set(low_latency_window_plugin()),
            
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
            ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .insert_resource(GameState(GameStates::TileReveal))
        .insert_resource(TurnsLeft(0))
        .insert_resource(MovementPoints(0))
        .insert_resource(VisitedTiles(vec![0]))
        .add_systems(Startup, (
            setup,
            setup_game_ui,
            setup_special_cards,
            setup_movement_cards,
            setup_tiles,))
        .add_event::<MovementPointsUpdateEvent>()
        .add_event::<TileSelected>()
        .add_event::<TileSelectedBlockerClose>()
        .add_event::<TileSetupComplete>()
        .add_event::<TurnsUpdateEvent>()
        .add_event::<TileClosedEvent>()
        .add_event::<DrawCardEvent>()
        .add_event::<MovementCardsDrawnEvent>()
        .add_event::<SpecialCardSelected>()
        .add_event::<OverTile>()
        .add_event::<OffTile>()
        .add_event::<OverSpecialCard>()
        .add_event::<OffSpecialCard>()
        .add_event::<SpecialCardSelectedBlockerClose>()
        .add_event::<SpecialCardClosed>()
        .add_event::<MovementCardsPlayedEvent>()
        .add_systems(
            Update,
            (
                update_movement_points.run_if(on_event::<MovementPointsUpdateEvent>()),
                update_turns_left.run_if(on_event::<TurnsUpdateEvent>()),
                on_tile_selected.run_if(on_event::<TileSelected>()),
                tile_selected_close.run_if(on_event::<TileSelectedBlockerClose>()),
                on_tile_setup_complete.run_if(on_event::<TileSetupComplete>()),
                on_tile_closed_event.run_if(on_event::<TileClosedEvent>()),
                on_draw_card.run_if(on_event::<DrawCardEvent>()),
                on_over_tile.run_if(on_event::<OverTile>()),
                on_off_tile.run_if(on_event::<OffTile>()),
                on_movement_cards_drawn.run_if(on_event::<MovementCardsDrawnEvent>()),
                on_over_special_card.run_if(on_event::<OverSpecialCard>()),
                on_off_special_card.run_if(on_event::<OffSpecialCard>()),
                on_special_card_selected.run_if(on_event::<SpecialCardSelected>()),
                selected_special_card_close.run_if(on_event::<SpecialCardSelectedBlockerClose>()),
                on_special_card_closed_event.run_if(on_event::<SpecialCardClosed>()),
                on_turn_ended.run_if(on_event::<MovementCardsPlayedEvent>()),
            ),

        )
        .run()
}

pub fn setup(
    mut commands: Commands,
    mut logging_next_state: ResMut<NextState<debug::DebugPickingMode>>,
) {
    commands.spawn(Camera2dBundle::default());

    logging_next_state.set(debug::DebugPickingMode::Disabled);
}



// fn move_sprite(
//     time: Res<Time>,
//     mut sprite: Query<&mut Transform, (Without<Sprite>, With<Children>)>,
// ) {
//     let t = time.elapsed_seconds() * 0.1;
//     for mut transform in &mut sprite {
//         let new = Vec2 {
//             x: 50.0 * t.sin(),
//             y: 50.0 * (t * 2.0).sin(),
//         };
//         transform.translation.x = new.x;
//         transform.translation.y = new.y;
//     }
// }


// basically same as above, but does something different.
// #[derive(Clone, Event)]
// struct Shutdown;

// impl From<ListenerInput<Pointer<Click>>> for Shutdown {
//     fn from(_event: ListenerInput<Pointer<Click>>) -> Self {
//         Shutdown
//     }
// }

// fn shutdown(mut exit_events: EventWriter<bevy::app::AppExit>) {
//     exit_events.send(AppExit);
// }

// fn main() {
//     App::new()
//         .add_plugins(
//             DefaultPlugins
//                 .set(low_latency_window_plugin())
//                 .set(LogPlugin {
//                     filter: "bevy_mod_picking=trace".into(), // Show picking logs trace level and up
//                     level: Level::ERROR, // Show all other logs only at the error level and up
//                 }),
//         )
//         .add_plugins(DefaultPickingPlugins)
//         .add_event::<CycleLogging>()
//         .add_event::<Shutdown>()
//         .add_systems(Startup, (setup, setup_3d))
//         .add_systems(Update, update_button_colors)
//         // add our button-event response systems, set to only run when the
//         // respective events are triggered.
//         .add_systems(Update, cycle_logging.run_if(on_event::<CycleLogging>()))
//         .add_systems(Update, shutdown.run_if(on_event::<Shutdown>()))
//         .run();
// }

// Everything below this line is identical to what's in bevy_ui, except the event listener is passed
// to .add_button along with the text to display.
// ----------------------------------------------------------------------------

// Use the [`PickingInteraction`] state of each button to update its color.
// fn update_button_colors(
//     mut buttons: Query<(Option<&PickingInteraction>, &mut BackgroundColor), With<Button>>,
// ) {
//     for (interaction, mut button_color) in &mut buttons {
//         *button_color = match interaction {
//             Some(PickingInteraction::Pressed) => Color::rgb(0.35, 0.75, 0.35),
//             Some(PickingInteraction::Hovered) => Color::rgb(0.25, 0.25, 0.25),
//             Some(PickingInteraction::None) | None => Color::rgb(0.15, 0.15, 0.15),
//         }
//         .into();
//     }
// }

// fn setup(mut commands: Commands) {
//     let root = commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     width: Val::Px(500.0),
//                     flex_direction: FlexDirection::Column,
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::FlexStart,
//                     margin: UiRect::horizontal(Val::Auto),
//                     ..default()
//                 },
//                 ..default()
//             },
//             // *** Important! ***
//             //
//             // We need to use `Pickable::IGNORE` here so the root node doesn't block pointer
//             // interactions from reaching the 3d objects under the UI.
//             //
//             // This node, as defined, will stretch from the top to bottom of the screen, take the
//             // width of the buttons, but will be invisible. Try commenting out this line or changing
//             // it to see what happens.
//             Pickable::IGNORE,
//         ))
//         .id();

//     commands
//         .entity(root)
//         .add_button(
//             "Cycle Logging State",
//             On::<Pointer<Click>>::send_event::<CycleLogging>(),
//         )
//         .add_button("Quit", On::<Pointer<Click>>::send_event::<Shutdown>());
// }

// trait NewButton {
//     fn add_button(self, text: &str, on_click_action: On<Pointer<Click>>) -> Self;
// }

// impl<'w, 's, 'a> NewButton for EntityCommands<'w, 's, 'a> {
//     fn add_button(mut self, text: &str, on_click_action: On<Pointer<Click>>) -> Self {
//         let child = self
//             .commands()
//             .spawn((
//                 ButtonBundle {
//                     style: Style {
//                         width: Val::Percent(100.0),
//                         height: Val::Px(42.0),
//                         margin: UiRect::top(Val::Percent(2.0)),
//                         justify_content: JustifyContent::Center,
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     ..default()
//                 },
//                 // Add an onclick
//                 on_click_action,
//                 // Buttons should not deselect other things:
//                 NoDeselect,
//             ))
//             .with_children(|parent| {
//                 parent.spawn((
//                     TextBundle {
//                         text: Text::from_section(
//                             text,
//                             TextStyle {
//                                 font_size: 40.0,
//                                 color: Color::rgb(0.9, 0.9, 0.9),
//                                 ..default()
//                             },
//                         ),
//                         ..default()
//                     },
//                     // Text should not be involved in pick interactions.
//                     Pickable::IGNORE,
//                 ));
//             })
//             .id();
//         self.add_child(child);
//         self
//     }
// }