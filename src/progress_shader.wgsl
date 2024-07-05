#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> empty_color: vec4<f32>;
@group(1) @binding(1)
var<uniform> progress: f32;
@group(1) @binding(2)
var<storage, read> segments: array<vec4<f32>>;
@group(1) @binding(3)
var<storage, read> amount: array<f32>;
@group(1) @binding(4)
var<uniform> count: u32;

@fragment
fn fragment(
    mesh: UiVertexOutput,
) -> @location(0) vec4<f32> {
    if progress < mesh.uv.x {
      return empty_color;
    }
    var current_amount: f32 = 0.0;
    for (var i = 0u; i < count; i++) {
        current_amount += amount[i] * progress;
        if current_amount > mesh.uv.x {
            return segments[i];
        }
    }
    return empty_color;
}
