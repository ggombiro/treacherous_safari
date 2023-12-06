use bevy::prelude::*;

pub enum GameStates  {
    TileReveal,
    SpecialCardSelection,
    SpecialCardReveal,
    MovementCardsPlay,
    Afflictions,
    TileSelection,
    PlayerMovement,
    TurnEnd,
}

#[derive(Resource)]
pub struct GameState(pub GameStates);