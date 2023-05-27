use bevy::prelude::*;
use crate::{FontSpec, Game, RunState};


#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(scoreboard)
            .add_system(button_interaction_system)
            .add_system(button_text_system);
    }
}

// part 16

fn setup_ui (mut commands: Commands, font_spec: Res<FontSpec>) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(50.0)),
            ..default()
        },
        ..default()
    })
        .with_children(|parent| {
            parent.spawn(TextBundle{
                text: Text::from_section(
                    "2048",
                    TextStyle {
                        font: font_spec.family.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    }
                ).with_alignment(TextAlignment::Center),
                ..default()
            });
            parent.spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(NodeBundle {
                   style: Style {
                       flex_direction: FlexDirection::Column,
                       align_items: AlignItems::Center,
                       margin: UiRect {
                           left: Val::Px(20.0),
                           right: Val::Px(20.0),
                           top: Val::Px(0.0),
                           bottom: Val::Px(0.0),
                       },
                       padding: UiRect::all(Val::Px(10.0)),
                       ..default()
                   },
                   background_color: NORMAL_BUTTON.into(),
                   ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Score",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 15.0,
                                color: Color::WHITE,
                            }
                        ).with_alignment(TextAlignment::Center),
                        ..default()
                    });
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "<score>",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            }
                        ).with_alignment(TextAlignment::Center),
                        ..default()
                    }).insert(ScoreDisplay);
                });
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                          "Best",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 15.0,
                                color: Color::WHITE,
                            }
                        ).with_alignment(TextAlignment::Center),
                      ..default()
                    });
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "<score>",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            }
                        ).with_alignment(TextAlignment::Center),
                       ..default()
                    }).insert(BestScoreDisplay);
                });
            });
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Button",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ),
                   ..default()
                });
            });
        });
}

//part 17

fn scoreboard(
    game: Res<Game>,
    mut query_score: ParamSet<(Query<&mut Text, With<ScoreDisplay>>, Query<&mut Text, With<BestScoreDisplay>>)>) {

    query_score.p0().single_mut().sections[0].value = game.score.to_string();
    query_score.p1().single_mut().sections[0].value = game.score_best.to_string();
}

// part 20
const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.75, 0.9);
const HOVERED_BUTTON: Color = Color::rgb(0.7, 0.7, 0.9);
const PRESSED_BUTTON: Color = Color::rgb(0.6, 0.6, 0.95);

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>)
    >,
    run_state: Res<State<RunState>>,
    mut next_state: ResMut<NextState<RunState>>,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON.into();
                match run_state.0 {
                    RunState::Playing => { next_state.set(RunState::GameOver); }
                    RunState::GameOver => { next_state.set(RunState::Playing); }
                }
            }
            Interaction::Hovered => { *background_color = HOVERED_BUTTON.into(); }
            Interaction::None => { *background_color = NORMAL_BUTTON.into(); }
        }
    }
}

fn button_text_system(
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    run_state: Res<State<RunState>>,
) {
    let children = button_query.single();
    let mut text = text_query.get_mut(*children.first().expect("button is to have only one child")).unwrap();
    match run_state.0 {
        RunState::Playing => { text.sections[0].value = "End Game".to_string(); }
        RunState::GameOver => { text.sections[0].value = "New Game".to_string(); }
    }
}
