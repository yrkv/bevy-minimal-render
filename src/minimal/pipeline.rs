use bevy::prelude::*;

use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::render::render_resource::{
    CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState,
    PipelineCache, PrimitiveState, RenderPipelineDescriptor, TextureFormat,
};

#[derive(Resource)]
pub struct MinimalRenderPipeline {
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for MinimalRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        let shader = world.load_asset("minimal.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("custom_render_pipeline".into()),
                layout: vec![],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        // format: TextureFormat::bevy_default(),
                        format: TextureFormat::Rgba8UnormSrgb, // different for wayland/x11
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self { pipeline_id }
    }
}
