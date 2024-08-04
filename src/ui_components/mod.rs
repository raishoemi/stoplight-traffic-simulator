pub mod buttons_hover_effect;
pub mod reaction_timer_controls;
pub mod reset_simulation_button;

use bevy::prelude::*;

/**
 * There's a single `setup` for the UI components so they'll be in the same container
 */
pub fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    reset_simulation_button::setup(parent);
                    reaction_timer_controls::setup(parent);
                });
        });
}
