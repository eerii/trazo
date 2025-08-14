//! Displays the game menu.

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Main), init_main_menu)
        .add_systems(
            Update,
            on_button_interaction.run_if(in_state(GameState::Menu)),
        );
}

// Systems
// ---

/// Spawn the main menu.
fn init_main_menu(mut cmd: Commands) {
    // TODO: Create or search some helpful macros to simplify this.
    //       bsn! would be nice :)
    cmd.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.),
            ..default()
        },
        children![
            (Text::new("Menu"), TextFont {
                font_size: 32.,
                ..default()
            }),
            (
                Button,
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(32.), Val::Px(12.)),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                children![(Text::new("Play"), TextColor(Color::BLACK),)]
            )
        ],
        StateScoped(MenuState::Main),
    ));
}

/// Handles button interaction
fn on_button_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut button: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_color: Query<&mut TextColor>,
) -> Result {
    for (interaction, mut color, children) in &mut button {
        let mut text_color = text_color.get_mut(children[0])?;
        match *interaction {
            Interaction::Pressed => {
                *color = css::WHITE.into();
                *text_color = css::ROYAL_BLUE.into();
                next_state.set(GameState::Play);
            },
            Interaction::Hovered => {
                *color = css::ROYAL_BLUE.into();
                *text_color = css::WHITE.into();
            },
            Interaction::None => {
                *color = css::WHITE.into();
                *text_color = css::BLACK.into();
            },
        }
    }
    Ok(())
}
