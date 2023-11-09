use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_progressbar::ProgressBar;
use bevy_progressbar::ProgressBarBundle;
use bevy_progressbar::ProgressBarMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_progressbar::ProgressBarPlugin)
        .add_systems(
            Startup,
            (
                |mut commands: Commands| {
                    commands.spawn(Camera2dBundle::default());
                },
                setup,
            ),
        )
        .add_systems(Update, (increase_progress, close_on_esc))
        .run();
}

fn setup(
    mut commands: Commands,
    // mut images: ResMut<Assets<Image>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    let bar = ProgressBar::new(vec![(200, Color::RED), (400, Color::BLUE)]);
    let style = Style {
        position_type: PositionType::Absolute,
        width: Val::Px(250.0),
        height: Val::Px(250.0),
        ..bevy_utils::default()
    };
    commands.spawn(ProgressBarBundle::new(style, bar, &mut materials));
}

fn increase_progress(mut query: Query<&mut ProgressBar>) {
    for mut bar in query.iter_mut() {
        if bar.is_finished() {
            bar.reset();
        } else {
            bar.increase_progress(0.01);
        }
    }
}
