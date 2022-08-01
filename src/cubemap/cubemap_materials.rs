use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            OwnedBindingResource, PreparedBindGroup, RenderPipelineDescriptor, SamplerBindingType,
            ShaderRef, ShaderStages, SpecializedMeshPipelineError, TextureSampleType,
            TextureViewDimension,
        },
        renderer::RenderDevice,
        texture::FallbackImage,
    },
};

pub trait BaseColorTexture {
    fn base_color_texture(&self) -> &Option<Handle<Image>>;
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "9509a0f8-3c05-48ee-a13e-a93226c7f488"]
pub struct CubemapMaterial {
    pub base_color_texture: Option<Handle<Image>>,
}

impl BaseColorTexture for CubemapMaterial {
    fn base_color_texture(&self) -> &Option<Handle<Image>> {
        &self.base_color_texture
    }
}

impl Material for CubemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cubemap_unlit.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

impl AsBindGroup for CubemapMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        images: &RenderAssets<Image>,
        fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self>, AsBindGroupError> {
        cubemap_as_bind_group(self, layout, render_device, images, fallback_image)
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        cubemap_bind_group_layout(render_device, false)
    }
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "f8c3ecd4-6179-437e-a069-07875afde195"]
pub struct CubemapArrayMaterial {
    pub base_color_texture: Option<Handle<Image>>,
}

impl BaseColorTexture for CubemapArrayMaterial {
    fn base_color_texture(&self) -> &Option<Handle<Image>> {
        &self.base_color_texture
    }
}

impl Material for CubemapArrayMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cubemap_unlit.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor
            .vertex
            .shader_defs
            .push(String::from("CUBEMAP_ARRAY"));
        descriptor
            .fragment
            .as_mut()
            .unwrap()
            .shader_defs
            .push(String::from("CUBEMAP_ARRAY"));
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

impl AsBindGroup for CubemapArrayMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        images: &RenderAssets<Image>,
        fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self>, AsBindGroupError> {
        cubemap_as_bind_group(self, layout, render_device, images, fallback_image)
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        cubemap_bind_group_layout(render_device, true)
    }
}

pub fn cubemap_as_bind_group<T: AsBindGroup<Data = ()> + BaseColorTexture>(
    material: &T,
    layout: &BindGroupLayout,
    render_device: &RenderDevice,
    images: &RenderAssets<Image>,
    _fallback_image: &FallbackImage,
) -> Result<PreparedBindGroup<T>, AsBindGroupError> {
    let base_color_texture = material
        .base_color_texture()
        .as_ref()
        .ok_or(AsBindGroupError::RetryNextUpdate)?;
    let image = images
        .get(base_color_texture)
        .ok_or(AsBindGroupError::RetryNextUpdate)?;
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&image.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&image.sampler),
            },
        ],
        label: Some("cubemap_texture_material_bind_group"),
        layout,
    });

    Ok(PreparedBindGroup {
        bind_group,
        bindings: vec![
            OwnedBindingResource::TextureView(image.texture_view.clone()),
            OwnedBindingResource::Sampler(image.sampler.clone()),
        ],
        data: (),
    })
}

pub fn cubemap_bind_group_layout(render_device: &RenderDevice, is_array: bool) -> BindGroupLayout {
    render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        entries: &[
            // Cubemap Base Color Texture
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: if is_array {
                        TextureViewDimension::CubeArray
                    } else {
                        TextureViewDimension::Cube
                    },
                },
                count: None,
            },
            // Cubemap Base Color Texture Sampler
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: None,
    })
}