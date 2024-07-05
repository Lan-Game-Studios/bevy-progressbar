use bevy::dev_tools::ui_debug_overlay::UiDebugOptions;
use bevy::prelude::*;
use bevy_color::palettes::tailwind;
use bevy_progressbar::ProgressBar;
use bevy_progressbar::ProgressBarBundle;
use bevy_progressbar::ProgressBarMaterial;
use bevy_utils::{default, Duration};

#[derive(Component)]
struct ExampleProgress;

#[derive(Component)]
struct ExampleUpdateSections(pub Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_progressbar::ProgressBarPlugin))
        .add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
        .add_systems(Startup, (setup,))
        .add_systems(Update, (increase_progress, update_sections, close_on_esc))
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
    mut options: ResMut<UiDebugOptions>,
) {
    options.enabled = true;
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Grid,
                padding: UiRect::all(Val::Px(36.0)),
                row_gap: Val::Px(12.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|wrapper| {
            for (index, bar) in [
                ProgressBar::new(vec![
                    (200, tailwind::RED_500.into()),
                    (400, tailwind::BLUE_500.into()),
                ]),
                ProgressBar::new(vec![
                    (200, tailwind::RED_500.into()),
                    (400, tailwind::BLUE_500.into()),
                    (300, tailwind::GREEN_500.into()),
                ])
                .set_progress(1.0)
                .clone(),
                ProgressBar::new(vec![(200, tailwind::RED_500.into())])
                    .set_progress(1.0)
                    .clone(),
            ]
            .into_iter()
            .enumerate()
            {
                let mut entity_command = wrapper.spawn(ProgressBarBundle::new(bar, &mut materials));

                if index == 0 {
                    entity_command.insert(ExampleProgress);
                }

                if index == 2 {
                    entity_command.insert(ExampleUpdateSections(Timer::new(
                        Duration::from_secs(2),
                        TimerMode::Repeating,
                    )));
                }
            }
        });
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
                0 => bar.add_section(200, tailwind::RED_500.into()),
                1 => bar.add_section(100, tailwind::GRAY_500.into()),
                2 => bar.add_section(150, tailwind::BLUE_500.into()),
                3 => bar.add_section(400, tailwind::GREEN_500.into()),
                _ => bar.clear_sections(),
            };
        }
    }
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
