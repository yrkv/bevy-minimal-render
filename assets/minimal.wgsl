
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput


@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>( in.uv, 0.0, 1.0 );
}