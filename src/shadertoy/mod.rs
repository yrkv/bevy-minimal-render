use bevy::prelude::*;

use bevy::app::Plugin;
use bevy::render::extract_component::{
    ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::render_graph::{RenderGraphApp, RenderLabel, RenderSubGraph, ViewNodeRunner};
use bevy::render::render_resource::ShaderType;
use bevy::render::RenderApp;

mod node;
use node::ShadertoyRenderNode;
mod pipeline;
use pipeline::ShadertoyRenderPipeline;

pub struct ShadertoyRenderGraphPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
pub struct ShadertoyRenderGraph;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct ShadertoyRenderNodeLabel;

impl Plugin for ShadertoyRenderGraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // The shadertoy inputs will be a component that lives in the main
            // world but will be extracted to the render world every frame.
            // This makes it possible to control the effect from the main world.
            // This plugin will take care of extracting it automatically.
            // It's important to derive [`ExtractComponent`] on [`ShadertoyInputs`]
            // for this plugin to work correctly.
            ExtractComponentPlugin::<ShadertoyInputs>::default(),
            // The inputs will also be the data used in the shader.
            // This plugin will prepare the component for the GPU by creating a uniform buffer
            // and writing the data to that buffer every frame.
            UniformComponentPlugin::<ShadertoyInputs>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_sub_graph(ShadertoyRenderGraph)
            .add_render_graph_node::<ViewNodeRunner<ShadertoyRenderNode>>(
                ShadertoyRenderGraph,
                ShadertoyRenderNodeLabel,
            );

        app.add_systems(Update, update_shadertoy_inputs);
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<ShadertoyRenderPipeline>();
    }
}

#[allow(non_snake_case)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct ShadertoyInputs {
    iResolution: Vec2,  // viewport resolution (in pixels)
    iTime: f32,         // shader playback time (in seconds)
    iMouse: Vec4,       // mouse pixel coords. xy: current (if MLB down), zw: click

    _webgl2_padding: f32,
    // TODO: the rest of it...
    /*
        uniform vec3      iResolution;           // viewport resolution (in pixels)
        uniform float     iTime;                 // shader playback time (in seconds)
    uniform float     iTimeDelta;            // render time (in seconds)
    uniform float     iFrameRate;            // shader frame rate
    uniform int       iFrame;                // shader playback frame
    uniform float     iChannelTime[4];       // channel playback time (in seconds)
    uniform vec3      iChannelResolution[4]; // channel resolution (in pixels)
        uniform vec4      iMouse;                // mouse pixel coords. xy: current (if MLB down), zw: click
    uniform samplerXX iChannel0..3;          // input channel. XX = 2D/Cube
    uniform vec4      iDate;                 // (year, month, day, time in seconds)
    uniform float     iSampleRate;           // sound sample rate (i.e., 44100)
    */
}

fn update_shadertoy_inputs(
    mut query: Query<(&mut ShadertoyInputs, &Camera)>,
    time: Res<Time>,
    mut evr_cursor: EventReader<CursorMoved>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    for (mut inputs, camera) in &mut query {
        if let Some(resolution) = camera.logical_viewport_size() {
            inputs.iResolution = resolution;
        }
        inputs.iTime = time.elapsed_seconds();
        for ev in evr_cursor.read() {
            if buttons.pressed(MouseButton::Left) {
                inputs.iMouse.x = ev.position.x;
                inputs.iMouse.y = ev.position.y;
            }
            if buttons.just_pressed(MouseButton::Left) {
                inputs.iMouse.z = ev.position.x;
                inputs.iMouse.w = ev.position.y;
            }
        }

    }
}
