use bevy::{
    color::palettes::css::RED,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{
        camera::{CameraMainTextureUsages, CameraRenderGraph},
        primitives::Frustum,
        view::VisibleEntities,
    },
    sprite::MaterialMesh2dBundle,
    window::PresentMode,
};

mod minimal;
use minimal::*;
mod shadertoy;
use shadertoy::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some("mahiya-bevy".into()),
                resolution: (1000., 1000.).into(),
                // present_mode: PresentMode::AutoNoVsync,
                present_mode: PresentMode::AutoVsync,
                decorations: false,
                ..default()
            }),
            ..default()
        }),
        FrameTimeDiagnosticsPlugin::default(),
        LogDiagnosticsPlugin::default(),
    ));

    //app.add_plugins(MinimalRenderGraphPlugin);
    app.add_plugins(ShadertoyRenderGraphPlugin);

    // change which shader is used in the pipeline.rs file

    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            // The "shadertoy" one adds uniforms like time and mouse position
            //camera_render_graph: CameraRenderGraph::new(MinimalRenderGraph),
            camera_render_graph: CameraRenderGraph::new(ShadertoyRenderGraph),
            camera: Camera {
                clear_color: Color::WHITE.into(),
                ..default()
            },
            ..Default::default()
        },
        ShadertoyInputs::default(),
    ));
}

// unused -- I was trying
fn setup_other(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::from(RED)),
        ..default()
    });
}
