use std::process::Child;

use bevy::{
    prelude::*,
    render::{
        Render, RenderApp,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
    },
};
mod record_screen;
mod spawn_ffmpeg_command;

use record_screen::record::record;

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
    pub fps: u32,
}

#[derive(Resource)]
pub struct FFmpegChild {
    pub child: Child,
}

impl Default for RecordScreen {
    fn default() -> Self {
        Self {
            output_name: String::from("output"),
            fps: 60,
        }
    }
}
