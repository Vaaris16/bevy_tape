# bevy_tape

bevy_tape is a plugin built for screen recording in the [bevy](https://crates.io/crates/bevy) game engine.

bevy_tape captures the primary render texture from Bevy’s rendering pipeline and encodes it using FFmpeg.

Recording starts when you add the [TapePlugin](https://doc.rs/bevy_tape/struct.TapePlugin.html) and [RecordScreen](https://doc.rs/bevy_tape/record_component/record/struct.RecordScreen.html) component to your camera.

## Note
bevy_tape does not support HDR yet.

## Example

### Dependencies
```toml
bevy = "0.19"
bevy_tape = "0.1.1"
```
 ```rust
 use bevy::prelude::*;
 use bevy_tape::{
     TapePlugin,
     record_component::{
         codec::Codec, file_type::FileType, px_format::PixelFormat, record::RecordScreen,
     },
 };

 fn main() {
     App::new()
         .add_plugins(DefaultPlugins.set(WindowPlugin {
             primary_window: Some(Window {
                 resizable: false,
                 ..Default::default()
             }),
             ..Default::default()
         }))
         .add_plugins(TapePlugin)
         .add_systems(Startup, setup)
         .run();
 }

 fn setup(
     mut commands: Commands,
     mut meshes: ResMut<Assets<Mesh>>,
     mut materials: ResMut<Assets<StandardMaterial>>,
 ) {
     commands.spawn((
         Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
         MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
     ));

     // Cube
     commands.spawn((
         Mesh3d(meshes.add(Cuboid::default())),
         MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
         Transform::from_xyz(0.0, 0.5, 0.0),
     ));

     // Light
     commands.spawn((
         PointLight {
             shadow_maps_enabled: true,
             ..default()
         },
         Transform::from_xyz(4.0, 8.0, 4.0),
     ));

     // Camera
     commands.spawn((
         Camera3d::default(),
         // Add the RecordScreen component
         RecordScreen {
             // name of the output file without the file extension
             output_name: String::from("custom_filename"),

             // fps of the video
             fps: 60,

             // define the file type for the video
             file_type: FileType::MP4,

             // define pixel format for the video
             pixel_format: PixelFormat::Yuv420p,

             // defines the codec used for the video
             codec: Codec::H265,

             // sets the maximum video duration in seconds
             max_secs: Some(10),

             // delays the start of recording by the specified number of seconds.
             min_secs: 5,
         },
         Transform::from_xyz(-2.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
     ));
 }
 ```

