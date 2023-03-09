# Bevy Progressbar

Generate progressbars in bevy ui from simple values. The progressbars can have multiple sections.

## Install

```rust
cargo add bevy-progressbar
```

## Usage

> See examples for better understanding

```rust
fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(
        // progressbar size, width, height
        ProgressBarBundle::new(10_000, 1000, 100, &mut images)
            .add_section(1000, Color::RED)
            .add_section(2000, Color::BLUE)
            .add_section(4000, Color::GREEN),

        ProgressBarBundle::new(10_000, 1000, 100, &mut images)
            .add_section(1000, Color::RED)
            .add_section(2000, Color::BLUE)
            .add_section(4000, Color::GREEN),
    );
}
```

| Version | Bevy Version |
|---------|--------------|
| 0.2.1   | 0.9          |
| 0.3.0   | 0.10         |

## TODO

- [ ] optimize the image creation, by just doing one line and let the gpu use nearst neighbour to scale the texture
- [ ] (optional) render an outline of the progessbar 
- [ ] (optional) allow or rounded corners
