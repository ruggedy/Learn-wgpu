// what us the difference between ..... and .....View
// i.e GenericImage and GenericImageView
use image::{DynamicImage, GenericImageView};

use anyhow::*;

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,

            // All textures are stored as 3D, we represent our 2B texture
            // by setting depth to 1.
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,

            // This is the same as with the SurfaceConfig. It
            // specifies what the texture formats can be used to
            // create TextureViews for this texture. The base
            // texture format (Rgba8UnormSrgb in this case)  is
            // always supported. Note that using a different
            // texture format is not supproted on the WebGL2
            // platform
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // The old way of writing texture to data was to copy the pixel data to a buffer and the copy it
        // to the texture. Using write_texture( as shown above) is a bit more efficient as it uses one buffer less.
        //
        //
        // let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Temp Buffer"),
        //     contents: &rgba,
        //     usage: wgpu::BufferUsages::COPY_SRC,
        // });

        // let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        //     label: Some("texture_buffer_copy_encoder"),
        // });

        // encoder.copy_buffer_to_texture(
        //     wgpu::TexelCopyBufferInfo {
        //         buffer: &buffer,
        //         layout: wgpu::TexelCopyBufferLayout {
        //             offset: 0,
        //             bytes_per_row: Some(4 * dimensions.0),
        //             rows_per_image: Some(dimensions.1),
        //         },
        //     },
        //     wgpu::TexelCopyTextureInfo {
        //         texture: &texture,
        //         mip_level: 0,
        //         origin: wgpu::Origin3d::ZERO,
        //         aspect: wgpu::TextureAspect::All,
        //     },
        //     texture_size,
        // );
        //
        // queue.submit(std::iter::once(encoder.finish()));
        //

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            view,
            sampler,
            texture,
        })
    }
}
