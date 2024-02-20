# Bevy Progressbar

Generate progressbars in bevy ui from simple values. The progressbars can have multiple sections.

## Install

```rust
cargo add bevy-progressbar
```

## Usage

> See examples for better understanding

```rust
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ProgressBarMaterial>>) {
    let bar = ProgressBar::new(vec![(1000, Color::RED),(2000, Color::BLUE),(4000, Color::GREEN)]);
    let style = Style {
        position_type: PositionType::Absolute,
        width: Val::Px(400.0),
        height: Val::Px(200.0),
        top: Val::Px(400.0),
        ..bevy_utils::default()
    };
    commands.spawn(
        ProgressBarBundle::new(style, bar, &mut materials);
    );
}
```

| Version | Bevy Version |
|---------|--------------|
| 0.2.1   | 0.9          |
| 0.3.0   | 0.10         |
| 0.4.0   | 0.11         |
| 0.5.0   | 0.12         |
| 0.6.0   | 0.12         |
| 0.7.0   | 0.13         |

## TODO
- [x] optimize the image creation, by just doing one line and let the gpu use nearst neighbour to scale the texture
- [ ] (optional) render an outline of the progessbar 
- [ ] (optional) allow or rounded corners
