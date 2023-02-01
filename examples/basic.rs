use bevy::prelude::*;
use bevy_progressbar::ProgressBarBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_progressbar::ProgressBarPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|node| {
            node.spawn(
                ProgressBarBundle::new(10_000, 1000, 100, &mut images)
                    .add_section(1000, Color::RED)
                    .add_section(2000, Color::BLUE)
                    .add_section(4000, Color::GREEN),
            );

            node.spawn(
                ProgressBarBundle::new(100, 1000, 20, &mut images)
                    .add_section(10, Color::TEAL)
                    .add_section(70, Color::INDIGO)
                    .add_section(20, Color::BEIGE),
            );

            node.spawn(
                ProgressBarBundle::new(100, 1000, 100, &mut images)
                    .add_section(6, Color::RED)
                    .add_section(32, Color::CYAN)
                    .add_section(21, Color::MIDNIGHT_BLUE),
            );
        });
}
