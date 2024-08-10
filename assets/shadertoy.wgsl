#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput


struct ShadertoyInputs {
    iResolution: vec2<f32>,
    iTime: f32,
    iMouse: vec4<f32>,
    _webgl2_padding: f32,
}
@group(0) @binding(0) var<uniform> inputs: ShadertoyInputs;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let col: vec3<f32> = 0.5 + 0.5*cos(inputs.iTime + in.uv.xyx+vec3<f32>(0,2,4));

    return vec4<f32>(col, 1.0);
}