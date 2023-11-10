use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_progressbar::ProgressBar;
use bevy_progressbar::ProgressBarBundle;
use bevy_progressbar::ProgressBarMaterial;
use bevy_utils::Duration;

#[derive(Component)]
struct ExampleProgress;

#[derive(Component)]
struct ExampleUpdateSections(pub Timer);

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
        .add_systems(Update, (increase_progress, update_sections, close_on_esc))
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ProgressBarMaterial>>) {
    let bar = ProgressBar::new(vec![(200, Color::RED), (400, Color::BLUE)]);
    let style = Style {
        position_type: PositionType::Absolute,
        width: Val::Px(400.0),
        height: Val::Px(200.0),
        ..bevy_utils::default()
    };
    commands
        .spawn(ProgressBarBundle::new(style, bar, &mut materials))
        .insert(ExampleProgress);

    let mut bar = ProgressBar::new(vec![
        (200, Color::RED),
        (400, Color::BLUE),
        (300, Color::GREEN),
    ]);
    bar.set_progress(1.0);
    let style = Style {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Px(200.0),
        top: Val::Px(200.0),
        ..bevy_utils::default()
    };
    commands.spawn(ProgressBarBundle::new(style, bar, &mut materials));

    let mut bar = ProgressBar::new(vec![(200, Color::RED)]);
    bar.set_progress(1.0);
    let style = Style {
        position_type: PositionType::Absolute,
        width: Val::Px(400.0),
        height: Val::Px(200.0),
        top: Val::Px(400.0),
        ..bevy_utils::default()
    };
    commands
        .spawn(ProgressBarBundle::new(style, bar, &mut materials))
        .insert(ExampleUpdateSections(Timer::new(
            Duration::from_secs(2),
            TimerMode::Repeating,
        )));
}

fn increase_progress(mut query: Query<&mut ProgressBar, With<ExampleProgress>>) {
    for mut bar in query.iter_mut() {
        if bar.is_finished() {
            bar.reset();
        } else {
            bar.increase_progress(0.01);
        }
    }
}
fn update_sections(
    mut query: Query<(&mut ProgressBar, &mut ExampleUpdateSections)>,
    time: Res<Time>,
) {
    for (mut bar, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            match bar.sections.len() {
                0 => bar.add_section(200, Color::RED),
                1 => bar.add_section(100, Color::GRAY),
                2 => bar.add_section(150, Color::BLUE),
                3 => bar.add_section(400, Color::GREEN),
                _ => bar.clear_sections(),
            };
        }
    }
}
