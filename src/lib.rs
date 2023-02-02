use bevy_app::prelude::Plugin;
use bevy_asset::prelude::Assets;
use bevy_ecs::prelude::{Bundle, Component, Query, ResMut, With};
use bevy_render::{
    prelude::Color, prelude::Image, render_resource::Extent3d, texture::ImageSampler,
};
use bevy_ui::{
    prelude::{ImageBundle, Style, Val},
    UiImage,
};

const BACKGROUND_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct ProgressBarSize {
    width: u32,
    height: u32,
}

#[derive(Component, Default)]
pub struct ProgressBarSections(Vec<(u32, Color)>);

#[derive(Component)]
pub struct Amount(pub u32);

#[derive(Bundle)]
pub struct ProgressBarBundle {
    size: ProgressBarSize,
    amount: Amount,
    sections: ProgressBarSections,
    progressbar: ProgressBar,
    #[bundle]
    image_bundle: ImageBundle,
}

impl ProgressBarBundle {
    pub fn new(amount: u32, width: u32, height: u32, images: &mut ResMut<Assets<Image>>) -> Self {
        Self {
            size: ProgressBarSize { width, height },
            progressbar: ProgressBar,
            amount: Amount(amount),
            sections: ProgressBarSections::default(),
            image_bundle: ImageBundle {
                style: Style {
                    size: bevy_ui::Size {
                        width: Val::Px(width as f32),
                        height: Val::Px(height as f32),
                    },
                    ..Default::default()
                },
                image: images.add(Self::image(width, height)).into(),
                ..Default::default()
            },
        }
    }
}

impl ProgressBarBundle {
    fn image(width: u32, height: u32) -> Image {
        let format = bevy_render::render_resource::TextureFormat::Rgba32Float;
        let pixels = width * height;
        let mut pixel_data: Vec<[f32; 4]> = Vec::new();
        pixel_data.resize(pixels as usize, [1.0, 0.0, 0.0, 1.0]); // default to all pixels transparent
        let mut image_data: Vec<u8> = Vec::new();

        for pixel in pixel_data.iter() {
            for color_channel in pixel.iter() {
                image_data.extend_from_slice(&color_channel.to_le_bytes());
            }
        }

        let mut image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            bevy_render::render_resource::TextureDimension::D2,
            image_data,
            format,
        );

        image.sampler_descriptor = ImageSampler::nearest();

        image
    }

    pub fn add_section(mut self, amount: u32, color: Color) -> Self {
        let total = self
            .sections
            .0
            .iter()
            .fold(0, |acc, section| acc + section.0)
            + amount;

        if total > self.amount.0 {
            panic!(
                "The progressbar is overfilled {} of {} amount",
                total, self.amount.0
            );
        }

        self.sections.0.push((amount, color));

        self
    }

    pub fn clear_sections(&mut self) -> &mut Self {
        self.sections = ProgressBarSections::default();

        self
    }
}

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_system(update_image);
    }
}

fn update_image(
    query: Query<(&ProgressBarSize, &UiImage, &ProgressBarSections, &Amount), With<ProgressBar>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (size, ui_image, sections, amount) in query.iter() {
        let mut image = images.get_mut(&ui_image.0).expect(
            "Progressbar image missing, should have been created through the bundle creation",
        );

        let pixels = size.width * size.height;
        let mut pixel_data: Vec<[f32; 4]> = Vec::new();
        pixel_data.resize(pixels as usize, [0.0, 0.0, 0.0, 0.0]); // default to all pixels transparent
        let mut image_data: Vec<u8> = Vec::new();

        // transform absolute amount to percentage
        let relative_sections: Vec<(f32, Color)> = sections
            .0
            .iter()
            .map(|(absolute_amount, color)| {
                ((*absolute_amount as f32 / amount.0 as f32), *color)
            })
            .collect();

        for (i, mut pixel) in pixel_data.iter().enumerate() {
            let x: u32 = i as u32 % size.width;
            // let y: u32 = i / size.width;
            let x_percentage: f32 = x as f32 / size.width as f32;
            // let y_percentage: f32 = y as f32 / size.height as f32;
            let color: Color = relative_sections
                .iter()
                .fold(
                    (0f32, BACKGROUND_COLOR),
                    |(sum, selected_color), (percentage, color)| {
                        let new_sum = sum + percentage;
                        let current_color = if sum < x_percentage {
                            *color
                        } else {
                            selected_color
                        };
                        (new_sum, current_color)
                    },
                )
                .1;

            let color_floats = if x_percentage
                > relative_sections
                    .iter()
                    .map(|(percentage, _)| percentage)
                    .sum()
            {
                BACKGROUND_COLOR.as_rgba_f32()
            } else {
                color.as_rgba_f32()
            };

            pixel = &color_floats;

            for color_channel in pixel.iter() {
                image_data.extend_from_slice(&color_channel.to_le_bytes());
            }
        }

        image.data = image_data;
    }
}
