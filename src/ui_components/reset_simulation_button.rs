use bevy::prelude::*;

#[derive(Component)]
pub struct ResetSimluationButton;

#[derive(Event)]
pub struct ResetSimluation;

pub fn setup(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                top: Val::Percent(2.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::MAX,
            ..default()
        })
        .insert(ResetSimluationButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}

pub fn update(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<ResetSimluationButton>,
        ),
    >,
    mut event_writer: EventWriter<ResetSimluation>,
) {
    for (interaction, _, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                event_writer.send(ResetSimluation);
            },
            _ => {},
        }
    }
}
