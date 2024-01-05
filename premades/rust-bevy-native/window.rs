use bevy::{
    core::FrameCount,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{PrimaryWindow, WindowMode},
};

pub struct WindowStuffPlugin;

impl WindowStuffPlugin {
    pub fn make_window_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
        if frames.0 == 3 {
            window.single_mut().visible = true;
        }
    }

    pub fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
        let window = window.single();
        commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        });
    }
}

impl Plugin for WindowStuffPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::Rgba {
            red: 0.231,
            green: 0.231,
            blue: 0.231,
            alpha: 0.0,
        }))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Colours".into(),
                        mode: WindowMode::BorderlessFullscreen,
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::PRIMARY),
                        ..default()
                    }),
                }),
        )
        .add_systems(Startup, Self::spawn_camera)
        .add_systems(Update, Self::make_window_visible);
    }
}
