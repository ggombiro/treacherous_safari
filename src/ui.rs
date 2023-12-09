use bevy::prelude::*;

pub fn setup_game_ui(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                left: Val::Percent(80.0),
                top: Val::Percent(3.0),
                width: Val::Percent(20.0),
                height: Val::Percent(8.0),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: Color::AQUAMARINE .into(),
            ..default()
        },
    ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                text: Text::from_section(
                    "Points:",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::DARK_GRAY,
                        ..default()
                    },
                ),
                ..default()
            },));
        });

    commands
        .spawn((NodeBundle {
            style: Style {
                top: Val::Percent(3.0),
                width: Val::Percent(20.0),
                height: Val::Percent(8.0),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: Color::AQUAMARINE .into(),
            ..default()
        },
    ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                text: Text::from_section(
                    "Turns:",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::DARK_GRAY,
                        ..default()
                    },
                ),
                ..default()
            },));
        });
}
