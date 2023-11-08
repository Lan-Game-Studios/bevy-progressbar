use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_progressbar::ProgressBarBundle;
use bevy_progressbar::ProgressBarMaterial;
use bevy_render::mesh::shape::Quad;
use bevy_sprite::Mesh2d;

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
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
         ) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Quad::new(Vec2::new(20., 20.)).into()).into(),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(ProgressBarMaterial {
            color: Color::BLUE,
        }),
        ..default()
    });
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
