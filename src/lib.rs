//! # bevy_tape
//!
//! bevy_tape is a plugin built for screen recording in the [bevy](https://crates.io/crates/bevy)
//! game engine.
//!
//! bevy_tape captures the main texture from Bevy’s rendering pipeline and uses FFmpeg to
//! encode the video output.
//!
//! The output can be configured through:
//! - Custom file extensions via [`FileType`]
//! - Multiple video codecs via [`Codec`]
//! - Explicit pixel format control via [`PixelFormat`]
//!
//! This makes bevy_tape an ideal crate for screen recording with minimal FPS loss
//! in the viewport.
//!
//! # Example
//!
//! ## Dependencies
//! ```toml
//! bevy = "0.19"
//! bevy_tape = "0.1.0"
//! ```
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_tape::TapePlugin;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(TapePlugin)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(
//!     mut commands: Commands,
//!     mut meshes: ResMut<Assets<Mesh>>,
//!     mut materials: ResMut<Assets<StandardMaterial>>,
//! ) {
//!     commands.spawn((
//!         Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
//!         MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
//!     ));
//!
//!     // Cube
//!     commands.spawn((
//!         Mesh3d(meshes.add(Cuboid::default())),
//!         MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
//!         Transform::from_xyz(0.0, 10.0, 0.0),
//!     ));
//!
//!     // Light
//!     commands.spawn((
//!         PointLight {
//!             shadow_maps_enabled: true,
//!             ..default()
//!         },
//!         Transform::from_xyz(4.0, 8.0, 4.0),
//!     ));
//!
//!     // Camera
//!     commands.spawn((
//!         Camera3d::default(),
//!         // Add the RecordScreen component
//!         RecordScreen {
//!             output_name: String::from("custom_name"),
//!             fps: 300,
//!             file_type: FileType::MOV,
//!             pixel_format: PixelFormat::Rgba,
//!             codec: Codec::ProRes,
//!         },
//!         Transform::from_xyz(-2.0, 2.5, 10.0)
//!             .looking_at(Vec3::ZERO, Vec3::Y),
//!     ));
//! }
//! ```

use bevy::{
    prelude::*,
    render::{Render, RenderApp, extract_component::ExtractComponentPlugin},
};

pub mod record_component;
pub mod record_screen;
pub mod spawn_ffmpeg_command;

use record_screen::record::record;

use crate::record_component::{
    codec::Codec, file_type::FileType, px_format::PixelFormat, record_component::RecordScreen,
};

/// The [TapePlugin] adds the systems needed to the render world.

pub struct TapePlugin;

impl Plugin for TapePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<RecordScreen>::default());
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(Render, record);
        }
    }
}
