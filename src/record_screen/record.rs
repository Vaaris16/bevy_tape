//! The [record] function runs as a system in the render world.
//!
//! It queries [RecordScreen] to extract recording parameters and forwards
//! them to [spawn_ffmpeg], which is responsible for spawning and managing
//! the FFmpeg process.
//!
//! The function allocates a GPU-backed readback buffer, copies the rendered
//! texture into system memory, extracts RGBA pixel data, and streams each
//! frame into FFmpeg for encoding.
use std::io::Write;

use crate::{
    RecordScreen,
    spawn_ffmpeg_command::spawn_ffmpeg::{FFmpegChild, spawn_ffmpeg},
};
use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode, Origin3d,
            PollType, TexelCopyBufferInfo, TexelCopyBufferLayout, TexelCopyTextureInfo,
            TextureAspect,
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
    mut frame: Local<i32>,
) {
    let record = match record_screen.single() {
        Ok(record) => record,
        Err(_) => return,
    };

    let min_frame = record.min_secs as u32 * record.fps;

    if *frame < min_frame as i32 {
        *frame += 1;
        return;
    }

    if let Some(mx) = record.max_secs {
        let max_frame = mx as u32 * record.fps;
        if *frame as u32 >= max_frame {
            return;
        }
    }

    // Retrieve the active view and its main render texture
    let view = match views.single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let texture = view.main_texture();
    let size = texture.size();

    let w = size.width;
    let h = size.height;

    // GPU readback requires row alignment to 256 bytes
    let bytes_per_row = ((w * 4) + 256 - 1) & !(256 - 1);
    let buffer_size = (w * h) * 4;

    // Create a GPU buffer for texture readback
    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("record_readback_buffer"),
        size: buffer_size as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Encode a copy from the render texture into the readback buffer
    let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("record_encoder"),
    });

    encoder.copy_texture_to_buffer(
        TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
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

    // Submit GPU work
    render_queue.submit(Some(encoder.finish()));

    // Map the buffer and block until the GPU copy completes
    let slice = buffer.slice(..);
    slice.map_async(MapMode::Read, |_| {});

    if let Err(e) = render_device.poll(PollType::wait_indefinitely()) {
        eprintln!("Poll Error: {e}");
    }

    let data = slice.get_mapped_range();

    // Spawn FFmpeg on the first captured frame
    if child.is_none() {
        spawn_ffmpeg(w, h, commands, record);
    }

    // Stream frame data to FFmpeg, skipping padded bytes per row
    if let Some(mut ffmpeg) = child
        && let Some(stdin) = ffmpeg.child.stdin.as_mut()
    {
        for row in 0..h as usize {
            let s = row * bytes_per_row as usize;
            let e = s + (w * 4) as usize;

            stdin
                .write_all(&data[s..e])
                .expect("failed to write buffer");
        }
    }

    *frame += 1;

    // Release the mapped buffer
    drop(data);
    buffer.unmap();
}
