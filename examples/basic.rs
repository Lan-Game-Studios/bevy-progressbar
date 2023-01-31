use bevy::prelude::*;
use bevy_progressbar::{Amount, ProgressBarBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_progressbar::ProgressBarPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
) {
    commands.spawn(
        ProgressBarBundle::new(10_000, 1000, 100, images)
            .add_section(1000, Color::RED)
            .add_section(2000, Color::BLUE)
            .add_section(4000, Color::GREEN)
    );
    println!("spawn successfull");
}

fn update(
    query: Query<(&Amount, &Transform)>
) {
    for (amount, transform) in query.iter() {
        println!("Amount {} Position {:?}", amount.0, transform.translation.truncate());
    }
}
