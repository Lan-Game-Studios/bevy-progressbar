use bevy_app::prelude::Plugin;
use bevy_asset::{prelude::Assets, Asset, Handle};
use bevy_ecs::prelude::{Bundle, Component, Query, ResMut};
use bevy_reflect::TypePath;
use bevy_render::{prelude::Color, render_resource::AsBindGroup};
use bevy_ui::{node_bundles::MaterialNodeBundle, Style, UiMaterial, UiMaterialPlugin};

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(bevy_app::Update, update_progress_bar)
            .add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default());
    }
}

#[derive(Component)]
pub struct ProgressBar {
    pub progress: f32,
    pub sections: Vec<(u32, Color)>,
}

impl ProgressBar {
    pub fn new(sections: Vec<(u32, Color)>) -> Self {
        Self {
            progress: 0.0,
            sections,
        }
    }
    pub fn increase_progress(&mut self, amount: f32) {
        self.progress += amount;
        self.progress = self.progress.clamp(0.0, 1.0);
    }

    pub fn reset(&mut self) {
        self.progress = 0.0;
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
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
                ..bevy_utils::default()
            },
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    pub empty_color: Color,
    #[uniform(1)]
    pub progress: f32,
    // check if these can be combined in a unit or struct
    #[storage(2)]
    pub segment: Vec<Color>,
    #[storage(3)]
    pub amount: Vec<f32>,
    #[uniform(4)]
    pub count: u32,
}

impl Default for ProgressBarMaterial {
    fn default() -> Self {
        Self {
            empty_color: Color::NONE,
            progress: 0.0,
            segment: vec![],
            amount: vec![],
            count: 0,
        }
    }
}

impl ProgressBarMaterial {
    pub fn update(&mut self, bar: &ProgressBar) {
        self.progress = bar.progress;
        self.segment = vec![];
        self.amount = vec![];
        let total_amount: u32 = bar.sections.iter().map(|(amount, _)| amount).sum();
        for (amount, color) in bar.sections.iter() {
            self.amount
                .push(1. / (total_amount as f32 / *amount as f32));
            self.segment.push(*color);
        }
        self.count = bar.sections.len() as u32;
    }
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> bevy_render::render_resource::ShaderRef {
        "shader/progress_shader.wgsl".into()
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
