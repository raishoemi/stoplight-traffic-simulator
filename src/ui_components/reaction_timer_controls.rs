use bevy::prelude::*;

pub fn setup(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                border: UiRect::all(Val::Px(5.0)),
                top: Val::Percent(10.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            // border_radius: BorderRadius::MAX,
            ..default()
        })
        .with_children(|parent| {
            // Slider handle
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(60.0),
                        height: Val::Px(45.0),
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
                        top: Val::Percent(2.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    ..default()
                })
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

pub fn update(// mut slider_query: Query<(&mut Style, &mut VolumeSlider)>,
    // mut interaction_query: Query<&Interaction, Changed<Interaction>, With<VolumeSlider>>,
) {
    // let (mut slider_style, mut slider) = slider_query.single_mut();
    // for interaction in interaction_query.iter_mut() {
    //     match *interaction {
    //         Interaction::Pressed => {
    //             // Update the slider value and position based on user input
    //             // Here you would add logic to calculate the new value based on the mouse position
    //             // For simplicity, this example just sets it to a fixed value
    //             slider.value = 0.75;
    //             slider_style.left = Val::Px(slider.value * 200.0);
    //         }
    //         _ => {}
    //     }
    // }
}
