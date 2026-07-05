use std::io::Write;

use crate::{FFmpegChild, RecordScreen, spawn_ffmpeg_command::spawn_ffmpeg::spawn_ffmpeg};
use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode, Origin3d,
            PollType, TexelCopyBufferInfo, TexelCopyBufferLayout, TexelCopyTextureInfo,
        },
        renderer::{RenderDevice, RenderQueue},
        view::ViewTarget,
    },
};

pub fn record(
    render_device: Res<RenderDevice>,
    views: Query<&ViewTarget>,
    render_queue: Res<RenderQueue>,
    record_screen: Query<&RecordScreen>,
    child: Option<ResMut<FFmpegChild>>,
    commands: Commands,
) {
    let view = match views.single() {
        Ok(v) => v,
        Err(_) => return,
    };

    let texture = view.main_texture();
    let size = texture.size();

    let w = size.width;
    let h = size.height;

    let bytes_per_row = (w * 4 + 256 - 1) & !(256 - 1);
    let buffer_size = ((w * h) * 4) as u64;

    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("buffer"),
        size: buffer_size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("encoder"),
    });

    encoder.copy_texture_to_buffer(
        TexelCopyTextureInfo {
            texture: texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: bevy::render::render_resource::TextureAspect::All,
        },
        TexelCopyBufferInfo {
            buffer: &buffer,
            layout: TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(h),
            },
        },
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );

    render_queue.submit(Some(encoder.finish()));

    let slice = buffer.slice(..);

    slice.map_async(MapMode::Read, |_| {});

    render_device.poll(PollType::wait_indefinitely());

    let data = slice.get_mapped_range();

    if child.is_none() {
        if let Ok(record) = record_screen.single() {
            println!("something");
            spawn_ffmpeg(
                w,
                h,
                commands,
                record.fps,
                &record.output_name,
                record.file_type,
            );
        }
    }

    if let Some(mut ffmpeg) = child {
        if let Some(stdin) = ffmpeg.child.stdin.as_mut() {
            for row in 0..size.height as usize {
                let s = row * bytes_per_row as usize;

                let e = s + (size.width * 4) as usize;

                stdin.write_all(&data[s..e]).unwrap();
            }
        }
    }

    drop(data);
    buffer.unmap();
}
