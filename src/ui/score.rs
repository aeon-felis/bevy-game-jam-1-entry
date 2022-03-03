use bevy::prelude::*;

use crate::global_types::PlayerStatus;
use crate::loading::FontAssets;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score_display);
        app.add_system(update_score_display);
    }
}

#[derive(Component)]
struct ScoreDisplayText;

fn setup_score_display(mut commands: Commands, font_assets: Res<FontAssets>) {
    let mut cmd = commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(0.0), Val::Px(90.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            position: Rect {
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        color: Color::YELLOW.into(),
        ..Default::default()
    });
    let text_style = TextStyle {
        font: font_assets.fira_sans.clone(),
        font_size: 30.0,
        color: Color::WHITE.into(),
        ..Default::default()
    };
    cmd.with_children(|commands| {
        let mut cmd = commands.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Distance: ".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\n".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "Time: ".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\n".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "Place: ".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: " out of ".to_owned(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Left,
                },
            },
            ..Default::default()
        });
        cmd.insert(ScoreDisplayText);
    });
}

fn update_score_display(
    mut query: Query<&mut Text, With<ScoreDisplayText>>,
    player_status: Res<PlayerStatus>,
) {
    for mut score_text in query.iter_mut() {
        score_text.sections[1].value = format!("{:.1}m", player_status.distance_traveled);
        score_text.sections[4].value = player_status.format_time();
        score_text.sections[7].value = player_status.place().to_string();
        score_text.sections[9].value = player_status.total_runners().to_string();
    }
}
