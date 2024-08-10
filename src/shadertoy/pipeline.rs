use bevy::prelude::*;

use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::render::render_resource::BindGroupLayout;
use bevy::render::{
    render_resource::{
        binding_types::uniform_buffer, BindGroupLayoutEntries, CachedRenderPipelineId,
        ColorTargetState, ColorWrites, FragmentState, MultisampleState, PipelineCache,
        PrimitiveState, RenderPipelineDescriptor, ShaderStages, TextureFormat,
    },
    renderer::RenderDevice,
};

use crate::ShadertoyInputs;

#[derive(Resource)]
pub struct ShadertoyRenderPipeline {
    // store the binding layout in our pipeline resource
    pub layout: BindGroupLayout,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for ShadertoyRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        // create a bind group layout defining what data we'll send to the shader
        let layout = world.resource::<RenderDevice>().create_bind_group_layout(
            "shadertoy_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (uniform_buffer::<ShadertoyInputs>(true),),
            ),
        );

        // let shader = world.load_asset("shadertoy.wgsl");
        let shader = world.load_asset("shadertoy_sphere_occlusion.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("shadertoy_render_pipeline".into()),

                    // add the binding layout here
                    layout: vec![layout.clone()],

                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::Rgba8UnormSrgb,
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            pipeline_id,
        }
    }
}
