use std::{
    io::Write,
    process::{self, Child, Stdio},
};

use bevy::{
    log::tracing_subscriber::fmt::format,
    prelude::*,
    render::{
        Render, RenderApp,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_resource::{
            BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, MapMode, Origin3d,
            PollType, TexelCopyBufferInfo, TexelCopyBufferLayout, TexelCopyTextureInfo,
        },
        renderer::{RenderDevice, RenderQueue},
        view::ViewTarget,
    },
};

pub struct TapePlugin;

impl Plugin for TapePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<RecordScreen>::default());
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(Render, record);
        }
    }
}

#[derive(Component, Clone, Debug, ExtractComponent)]
pub struct RecordScreen {
    pub output_name: String,
}

#[derive(Resource)]
struct FFmpegChild {
    pub child: Child,
}

impl Default for RecordScreen {
    fn default() -> Self {
        Self {
            output_name: String::from("output"),
        }
    }
}

fn record(
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
            spawn_ffmpeg(w, h, commands, &record.output_name);
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

fn spawn_ffmpeg(width: u32, height: u32, mut commands: Commands, output_name: &str) {
    let res = format!("{}x{}", width, height);
    let file_name = format!("{}.mp4", output_name);
    let child = process::Command::new("ffmpeg")
        .args([
            "-y", "-f", "rawvideo", "-pix_fmt", "rgba", "-s", &res, "-r", "60", "-i", "-", "-c:v",
            "libx264", "-pix_fmt", "yuv420p", &file_name,
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("errr");
    commands.insert_resource(FFmpegChild { child: child });
}
