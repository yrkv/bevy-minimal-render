use bevy::prelude::*;

use bevy::app::Plugin;
use bevy::render::render_graph::{RenderGraphApp, RenderLabel, RenderSubGraph, ViewNodeRunner};
use bevy::render::RenderApp;

mod node;
use node::MinimalRenderNode;
mod pipeline;
use pipeline::MinimalRenderPipeline;

pub struct MinimalRenderGraphPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
pub struct MinimalRenderGraph;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct MinimalRenderNodeLabel;

impl Plugin for MinimalRenderGraphPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_sub_graph(MinimalRenderGraph)
            .add_render_graph_node::<ViewNodeRunner<MinimalRenderNode>>(
                MinimalRenderGraph,
                MinimalRenderNodeLabel,
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // Initialize the pipeline
        render_app.init_resource::<MinimalRenderPipeline>();
    }
}
