mod draw;
mod face;
mod handle_event;
mod stick;
mod tooth;
mod update;

use geng::Camera2d;

use crate::assets::Assets;

use self::face::*;
use self::stick::*;
use self::tooth::*;

use super::*;

pub struct GameState {
    geng: Geng,
    assets: Rc<Assets>,
    framebuffer_size: Vec2<f32>,
    camera: Camera2d,

    transition: Option<Transition>,
    face: Face,
    stick: Stick,
}

impl GameState {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let state = Self {
            geng: geng.clone(),
            assets: assets.clone(),
            framebuffer_size: vec2(1.0, 1.0),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
            face: {
                let teeth_locations: Vec<_> = serde_json::from_str(&assets.teeth_config)
                    .expect("Failed to deserialize teeth config");

                assert_eq!(
                    assets.teeth.len(),
                    teeth_locations.len(),
                    "There must be as many textures as there are locations in the config file!"
                );
                Face {
                    teeth: teeth_locations
                        .into_iter()
                        .enumerate()
                        .map(|(index, position)| Tooth {
                            texture: assets.teeth[index].clone(),
                            position,
                            state: ToothState::Healthy,
                        })
                        .collect(),
                }
            },
            stick: Stick {
                position: vec2(0.0, -constants::FACE_SIZE + constants::STICK_SIZE.y / 2.0),
                state: StickState::Moving,
            },
            transition: None,
        };
        state
    }
}

impl geng::State for GameState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.draw_impl(framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.update_impl(delta_time);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.handle_event_impl(event);
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        self.transition.take().map(|transition| match transition {
            Transition::Reload => geng::Transition::Switch(Box::new(loading_screen(&self.geng))),
        })
    }
}

enum Transition {
    Reload,
}

fn mouse_world_pos(geng: &Geng, camera: &Camera2d, framebuffer_size: Vec2<f32>) -> Vec2<f32> {
    camera.screen_to_world(
        framebuffer_size,
        geng.window().mouse_pos().map(|x| x as f32),
    )
}
