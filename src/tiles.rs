use crate::game_state::{GameState, GameStates};
use crate::movement::{
    create_movement_points, update_movement_points, MovementPoints, MovementPointsUpdateEvent,
};
use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::prelude::*;
use rand::Rng;


const FOCUS_SCALE: f32 = 0.1;
const SELECTED_SCALE: f32 = 2.0;
const BLOCKER_COLOR_VALUE: f32 = 0.7;

#[derive(Component, Debug, Clone)]
pub struct Tile{
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

#[derive(Event)]
pub struct TileSetupComplete;


pub fn setup_tiles(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut tile_setup_complete: EventWriter<TileSetupComplete>,
) {
    let len = 64.0;
    let height = 97.5;
    let sprite_size = Some(Vec2::new(len, height));
    let mut tile_costs = vec![1,2];
    let mut tile_res = generate_tiles();
    let mut rng = rand::thread_rng();


    commands
        .spawn((
            SpatialBundle::default(),
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
            const X_STEP: f32 = 32.0;
            const Y_START: f32 = -97.5;
            const Y_STEP: f32 = 97.5;
            const SPACING: f32 = 50.0;

            

            let mut counter = 0;
            let mut tile_res_index: usize = 30;

            for x in 0..5 {
                for y in 0..3 {
                    if (x == 0 || x == 4) && (y == 0 || y == 2) {
                        continue;
                    }
                    
                    if tile_costs.len() == 0{
                        break;
                    }

                    let rand_index = rng.gen_range(0..tile_costs.len());
                    let tile_cost = tile_costs[rand_index];
                    tile_costs.remove(rand_index);


                    let mut indices = Vec::new();

                    for (index, tile) in tile_res.iter_mut().enumerate(){
                        if tile.cost == tile_cost{
                            indices.push(index);
                        }
                    }

                    if indices.len() == 0{
                        break;
                    }

                    tile_res_index = rng.gen_range(0..indices.len());
                    let tile = &mut tile_res[tile_res_index];
                    

                    tile.number = counter;
                    tile.neighbours = get_neighbours(counter);
                    tile.current= if counter == 0 {true} else{ false};

                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: sprite_size,
                                color: Color::BLACK,
                                ..default()
                            },
                            // texture: asset_server.load("images/boovy.png"),
                            transform: Transform::from_xyz(
                                (X_START + (x as f32 * X_STEP)) + (x as f32 * SPACING),
                                (Y_START + (y as f32 * Y_STEP)) + (y as f32 * (SPACING / 3.0)),
                                -1.0,
                            ),
                            ..default()
                        },
                        tile.clone(),
                    ));

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
    mut tiles: Query<&mut Transform, With<Tile>>,
    mut blocker: Query<&mut Visibility, With<TileRevealBlocker>>,
    mut close_button: Query<(
        &mut Visibility,
        &TileRevealBlockerCloseButton,
        Without<TileRevealBlocker>,
    )>,
) {
    for mut tile in &mut tiles {
        if tile.scale.x > 1.0 {
            if tile.scale.x < FOCUS_SCALE + SELECTED_SCALE {
                tile.scale.x += SELECTED_SCALE;
                tile.scale.y += SELECTED_SCALE;
                tile.translation.z = 1.0;

                let mut blocker = blocker.single_mut();
                *blocker = Visibility::Visible;

                let mut close_button = close_button.single_mut();
                *close_button.0 = Visibility::Visible;
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
) {
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
        }
    }
}

#[derive(Debug,Clone)]
pub enum TileType {
    Plain,
    CurseMovementPoints,
    MovementPointsAdd,
    TurnReduction,
    RouteRestriction,
    MovementPointsSub,
    Blessing,
    StepBack,
}

pub fn get_neighbours(index: u32) -> Vec<i32>{
    match index{
        0 => vec![1,2,3],
        1 => vec![2,4,5],
        2 => vec![1,3,4,5,6],
        3 => vec![2,5,6],
        4 => vec![5,7,8],
        5 => vec![4,6,7,8,9],
        6 => vec![5,8,9],
        7 => vec![8,10],
        8 => vec![7,9,10],
        9 => vec![8,10],
        _ => vec![-1],
    }
}

pub fn on_tile_setup_complete(
    mut commands: Commands,
    mut events: EventReader<TileSetupComplete>,
    mut tiles: Query<(Entity, &mut Transform, &Tile)>,
    mut tile_selected: EventWriter<TileSelected>,
){
    for (entity, mut transform, mut tile) in &mut tiles {
        info!("Tile: {:?}", tile);
        if tile.current {
            info!("Current Tile: {:?}", tile);
            transform.scale.x += FOCUS_SCALE;
            transform.scale.y += FOCUS_SCALE;

            tile_selected.send(TileSelected(entity));
        }
    }
}

pub fn generate_tiles() -> Vec<Tile>{

    let mut tile_res = Vec::with_capacity(2);
         

    let mut tile_1_0 = Tile{
        cost : 1,
        description: String::from("You come across some ruins. There is something glittering inside.\n
        \"Don't touch that!\" Too late, you are cursed. -2 movement points on every turn."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::CurseMovementPoints,
        value: -2,
        duration: -1,
        current: false,
    };

    let mut tile_2_0 = Tile{
        cost : 2,
        description: String::from("You find yourself in a river.\n
        -1 turn to swim across the river."),
        number: 0,
        neighbours: vec![-1],
        tile_type: TileType::TurnReduction,
        value: -1,
        duration: 1,
        current: false,
    };

    tile_res.push(tile_1_0);
    tile_res.push(tile_2_0);

    tile_res
}