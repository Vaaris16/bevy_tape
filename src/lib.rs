use bevy::{
    prelude::*,
    render::{Render, RenderApp, extract_component::ExtractComponentPlugin},
};

pub mod errors;
pub mod record_component;
pub mod record_screen;
pub mod spawn_ffmpeg_command;

use record_screen::record::record;

use crate::record_component::{
    codec::Codec, file_type::FileType, px_format::PixelFormat, record_component::RecordScreen,
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
