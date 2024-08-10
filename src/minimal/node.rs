use bevy::prelude::*;

use bevy::ecs::query::QueryItem;
use bevy::render::{
    render_graph::{NodeRunError, RenderGraphContext, ViewNode},
    render_resource::{PipelineCache, RenderPassDescriptor},
    renderer::RenderContext,
    view::ViewTarget,
};

use super::pipeline::MinimalRenderPipeline;

#[derive(Default)]
pub struct MinimalRenderNode {}

impl ViewNode for MinimalRenderNode {
    type ViewQuery = (&'static ViewTarget,);

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (view_target,): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        // This needs to run at least once to clear the background color, even if there's no pipeline (yet)
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("minimal_render_pass"),
            color_attachments: &[Some(view_target.out_texture_color_attachment(None))],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // The pipeline cache is a cache of all previously created pipelines.
        // It is required to avoid creating a new pipeline each frame,
        // which is expensive due to shader compilation.
        let pipeline_cache = world.resource::<PipelineCache>();

        let render_pipeline = world.resource::<MinimalRenderPipeline>();

        // Get the pipeline from the cache
        let Some(pipeline) = pipeline_cache.get_render_pipeline(render_pipeline.pipeline_id) else {
            return Ok(());
        };

        render_pass.set_render_pipeline(pipeline);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
