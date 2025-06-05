use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_graph::{NodeRunError, RenderGraphApp, RenderLabel, ViewNode, ViewNodeRunner},
        render_resource::{
            binding_types::{sampler, texture_2d},
            *,
        },
        renderer::RenderDevice,
        texture::GpuImage,
        view::ViewTarget,
        RenderApp,
    },
};

pub struct PalettizationPlugin;

/// Add to the camera to enable the effect
#[derive(Component, ExtractComponent, Clone)]
pub struct PalettizationEffect {
    pub lut_image: Handle<Image>,
}

impl Plugin for PalettizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<PalettizationEffect>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            warn!("Could not get RenderApp, palettization post-processing will not work");
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PalettizationNode>>(Core3d, PalettizationLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    PalettizationLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            warn!("Could not get RenderApp, palettization post-processing will not work");
            return;
        };

        render_app.init_resource::<PalettizationPipeline>();
    }
}

#[derive(RenderLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct PalettizationLabel;

#[derive(Default)]
struct PalettizationNode;

impl ViewNode for PalettizationNode {
    type ViewQuery = (&'static ViewTarget, &'static PalettizationEffect);

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, palettization_effect): bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let palettization_pipeline = world.resource::<PalettizationPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(palettization_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let post_process_write = view_target.post_process_write();

        let Some(gpu_lut_image) = world
            .resource::<RenderAssets<GpuImage>>()
            .get(&palettization_effect.lut_image)
        else {
            warn!("No GPU LUT texture?");
            return Ok(());
        };

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &palettization_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process_write.source,
                gpu_lut_image.texture_view.into_binding(),
                &palettization_pipeline.sampler,
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process_write.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct PalettizationPipeline {
    pipeline_id: CachedRenderPipelineId,
    layout: BindGroupLayout,
    sampler: Sampler,
}

impl FromWorld for PalettizationPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: false }),
                    sampler(SamplerBindingType::NonFiltering),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor {
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..default()
        });

        let shader = world.load_asset("shaders/palettization.wgsl");

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("post_process_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                    zero_initialize_workgroup_memory: false,
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
