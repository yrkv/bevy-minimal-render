use bevy::prelude::*;

use bevy::ecs::query::QueryItem;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_resource::BindGroupEntries;
use bevy::render::{
    render_graph::{NodeRunError, RenderGraphContext, ViewNode},
    render_resource::{PipelineCache, RenderPassDescriptor},
    renderer::RenderContext,
    view::ViewTarget,
};

use crate::ShadertoyInputs;

use super::pipeline::ShadertoyRenderPipeline;

#[derive(Default)]
pub struct ShadertoyRenderNode {}

impl ViewNode for ShadertoyRenderNode {
    type ViewQuery = (
        &'static ViewTarget,
        // This makes sure the node only runs on cameras with the ShadertoyInputs component.
        // Since this is in the render world, needs ExtractComponentPlugin::<ShadertoyInputs>
        &'static ShadertoyInputs,
        // As there could be multiple components sent to the GPU (one per camera),
        // we need to get the index of the one that is associated with the current view.
        &'static DynamicUniformIndex<ShadertoyInputs>,
    );

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (view_target, _shadertoy_inputs, inputs_index): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        // Get the shadertoy inputs as a uniform binding, using UniformComponentPlugin
        let inputs_uniforms = world.resource::<ComponentUniforms<ShadertoyInputs>>();
        let Some(inputs_binding) = inputs_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let pipeline_cache = world.resource::<PipelineCache>();
        let render_pipeline = world.resource::<ShadertoyRenderPipeline>();

        // create a bind group for the render device
        let bind_group = render_context.render_device().create_bind_group(
            "shadertoy_bind_group",
            &render_pipeline.layout,
            // It's important for this to match the BindGroupLayout defined in the pipeline
            &BindGroupEntries::sequential((inputs_binding.clone(),)),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("shadertoy_render_pass"),
            color_attachments: &[Some(view_target.out_texture_color_attachment(None))],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let Some(pipeline) = pipeline_cache.get_render_pipeline(render_pipeline.pipeline_id) else {
            return Ok(());
        };

        // set the relevant bind group
        render_pass.set_bind_group(0, &bind_group, &[inputs_index.index()]);

        render_pass.set_render_pipeline(pipeline);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
