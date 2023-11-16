use bevy_app::prelude::Plugin;
use bevy_asset::{load_internal_asset, prelude::Assets, Asset, Handle};
use bevy_ecs::prelude::{Bundle, Component, Query, ResMut};
use bevy_reflect::TypePath;
use bevy_render::{
    prelude::Color,
    render_resource::{AsBindGroup, Shader},
};
use bevy_ui::{node_bundles::MaterialNodeBundle, Style, UiMaterial, UiMaterialPlugin};
use bevy_utils::default;

pub const PROGRESS_BAR_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(8714649747086695632918559878778085427);
pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        load_internal_asset!(
            app,
            PROGRESS_BAR_HANDLE,
            "progress_shader.wgsl",
            Shader::from_wgsl
        );
        app.add_systems(bevy_app::Update, update_progress_bar)
            .add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default());
    }
}

/// The Progress Bar.
/// Has Different Colored section with relative size to each other
/// and a Color for the empty space
#[derive(Component)]
pub struct ProgressBar {
    /// The Progress
    /// a f32 between 0.0 and 1.0
    progress: f32,
    /// The Different Sections
    /// The amount is the space relative to the other Sections.
    pub sections: Vec<(u32, Color)>,
    /// The Color of the space that is not progressed to
    pub empty_color: Color,
}

impl ProgressBar {
    /// Creates a new ProgressBar
    ///
    /// # Examples
    /// ```
    /// use bevy_progressbar::ProgressBar;
    /// use bevy_render::prelude::Color;
    /// let bar = ProgressBar::new(vec![(10, Color::RED), (9, Color::BLUE)]);
    /// ```
    pub fn new(sections: Vec<(u32, Color)>) -> Self {
        Self {
            progress: 0.0,
            sections,
            empty_color: Color::NONE,
        }
    }
    /// Creates a new ProgressBar with a single section
    pub fn single(color: Color) -> Self {
        Self {
            progress: 0.0,
            sections: vec![(1, color)],
            empty_color: Color::NONE,
        }
    }

    /// Sets the progress of the bar
    ///
    /// # Arguments
    ///
    /// * `amount` - The Progress. gets clamped between 0.0 and 1.0
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_progressbar::ProgressBar;
    ///
    /// let mut bar = ProgressBar::default();
    /// bar.set_progress(0.5);
    /// assert_eq!(bar.get_progress(), 0.5);
    /// bar.set_progress(10.0);
    /// assert_eq!(bar.get_progress(), 1.0);
    /// ```
    pub fn set_progress(&mut self, amount: f32) -> &mut Self {
        self.progress = amount.clamp(0.0, 1.0);
        self
    }

    /// Returns the current progress
    pub fn get_progress(&self) -> f32 {
        self.progress
    }

    /// Increases the progress
    /// the new progress is at most 1.0
    ///
    /// # Examples
    /// ```
    /// use bevy_progressbar::ProgressBar;
    /// let mut bar = ProgressBar::default();
    /// bar.increase_progress(0.5);
    /// assert_eq!(bar.get_progress(), 0.5);
    /// bar.increase_progress(4.2);
    /// assert_eq!(bar.get_progress(), 1.0);
    /// ```
    pub fn increase_progress(&mut self, amount: f32) -> &mut Self {
        self.progress += amount;
        self.progress = self.progress.clamp(0.0, 1.0);
        self
    }

    /// Resets the progress to 0.0
    pub fn reset(&mut self) -> &mut Self {
        self.progress = 0.0;
        self
    }

    /// Returns true if the ProgressBar is is_finished
    ///
    /// # Examples
    /// ```
    /// use bevy_progressbar::ProgressBar;
    /// let mut bar = ProgressBar::default();
    /// assert_eq!(bar.is_finished(), false);
    /// bar.increase_progress(1.0);
    /// assert_eq!(bar.is_finished(), true);
    /// ```
    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn clear_sections(&mut self) -> &mut Self {
        self.sections.clear();
        self
    }

    pub fn add_section(&mut self, amount: u32, color: Color) -> &mut Self {
        self.sections.push((amount, color));
        self
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            progress: 0.0,
            sections: vec![],
            empty_color: Color::NONE,
        }
    }
}

#[derive(Bundle)]
pub struct ProgressBarBundle {
    progressbar: ProgressBar,
    material_node_bundle: MaterialNodeBundle<ProgressBarMaterial>,
}

impl ProgressBarBundle {
    pub fn new(
        style: Style,
        progressbar: ProgressBar,
        materials: &mut ResMut<Assets<ProgressBarMaterial>>,
    ) -> ProgressBarBundle {
        ProgressBarBundle {
            progressbar,
            material_node_bundle: MaterialNodeBundle {
                style,
                material: materials.add(ProgressBarMaterial::default()),
                ..default()
            },
        }
    }
}

/// The Material for the ProgressBar
/// uses a simple wgsl shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    empty_color: Color,
    #[uniform(1)]
    progress: f32,
    /// The color of each section
    #[storage(2)]
    sections_color: Vec<Color>,
    #[storage(3)]
    sections_start_percentage: Vec<f32>,
    /// the length of the `sections_color` / `sections_start_percentage` vec.
    /// needs to be set for the shader
    #[uniform(4)]
    sections_count: u32,
}

impl Default for ProgressBarMaterial {
    fn default() -> Self {
        Self {
            empty_color: Color::NONE,
            progress: 0.0,
            sections_color: vec![],
            sections_start_percentage: vec![],
            sections_count: 0,
        }
    }
}

impl ProgressBarMaterial {
    /// Updates the material to match the ProgressBar
    pub fn update(&mut self, bar: &ProgressBar) {
        self.empty_color = bar.empty_color;
        self.progress = bar.progress;
        self.sections_color = vec![];
        self.sections_start_percentage = vec![];
        let total_amount: u32 = bar.sections.iter().map(|(amount, _)| amount).sum();
        for (amount, color) in bar.sections.iter() {
            self.sections_start_percentage
                .push(1. / (total_amount as f32 / *amount as f32));
            self.sections_color.push(*color);
        }
        self.sections_count = bar.sections.len() as u32;
    }
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> bevy_render::render_resource::ShaderRef {
        PROGRESS_BAR_HANDLE.into()
    }
}

fn update_progress_bar(
    bar_query: Query<(&ProgressBar, &Handle<ProgressBarMaterial>)>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    for (bar, handle) in bar_query.iter() {
        let Some(material) = materials.get_mut(handle) else {
            continue;
        };

        material.update(bar);
    }
}
