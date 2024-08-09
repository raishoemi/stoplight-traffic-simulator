use bevy::prelude::*;

use crate::car_fleet::car::{Car, ReactionTimer};

#[derive(Component)]
pub struct IncreaseReactionTimeButton;

#[derive(Component)]
pub struct DecreraseReactionTimeButton;

#[derive(Component)]
pub struct ReactionTimeValueText;

#[derive(Event)]
pub struct ReactionTimeChanged {
    pub delta: f32,
}

pub fn setup(parent: &mut ChildBuilder) {
    parent
        .spawn(TextBundle::from("").with_style(Style {
            top: Val::Percent(6.0),
            ..default()
        }))
        .insert(ReactionTimeValueText);
    parent
        .spawn(NodeBundle {
            style: Style {
                border: UiRect::all(Val::Px(5.0)),
                top: Val::Percent(8.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(60.0),
                        height: Val::Px(45.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    ..default()
                })
                .insert(IncreaseReactionTimeButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "+",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(60.0),
                        height: Val::Px(45.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    ..default()
                })
                .insert(DecreraseReactionTimeButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "-",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn buttons_listenerr(
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&IncreaseReactionTimeButton>,
            Option<&DecreraseReactionTimeButton>,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            Or<(
                With<IncreaseReactionTimeButton>,
                With<DecreraseReactionTimeButton>,
            )>,
        ),
    >,
    mut event_writer: EventWriter<ReactionTimeChanged>,
) {
    for (interaction, increase_button, decrease_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if increase_button.is_some() {
                    event_writer.send(ReactionTimeChanged { delta: 0.1 });
                } else if decrease_button.is_some() {
                    event_writer.send(ReactionTimeChanged { delta: -0.1 });
                }
            }
            _ => {}
        }
    }
}

/**
* Although I could've set the text when sending the `ReactionTimeChanged` event, I decided to
* to do it separately to decouple this logic. It may be possible the text will be updated different per-car.
*/
pub fn update_reaction_time_text(
    car_reaction_time_q: Query<(&Car, &ReactionTimer)>,
    mut text_q: Query<&mut Text, With<ReactionTimeValueText>>,
) {
    let (_, reaction_time) = car_reaction_time_q.iter().next().unwrap();
    let mut text = text_q.single_mut();
    let reaction_time_in_seconds = reaction_time.0.duration().as_secs_f32();
    text.sections[0].value = format!("Reaction Time: {:.1}s", reaction_time_in_seconds);
}
