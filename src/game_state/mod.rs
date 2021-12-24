mod crumb;
mod draw;
mod face;
mod handle_event;
mod stick;
mod tooth;
mod update;

use geng::Camera2d;

use crate::assets::Assets;
use crate::config::Config;

use self::crumb::*;
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
        fn teeth_locations_to_teeth(
            locations: &[Vec2<f32>],
            textures: &[Rc<ugli::Texture>],
            config: &Config,
        ) -> Vec<Tooth> {
            locations
                .iter()
                .enumerate()
                .map(|(index, &position)| Tooth {
                    texture: textures[index].clone(),
                    position: position * config.face_radius * 2.0
                        - vec2(config.face_radius, config.face_radius),
                    state: ToothState::Healthy,
                })
                .collect()
        }

        let state = Self {
            geng: geng.clone(),
            assets: assets.clone(),
            framebuffer_size: vec2(1.0, 1.0),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 30.0,
            },
            face: Face {
                teeth: Teeth {
                    top: teeth_locations_to_teeth(
                        &assets.config.top_teeth_locations,
                        &assets.top_teeth,
                        &assets.config,
                    ),
                    bottom: teeth_locations_to_teeth(
                        &assets.config.bottom_teeth_locations,
                        &assets.bottom_teeth,
                        &assets.config,
                    ),
                },
                crumbs: vec![Crumb {
                    tooth_position: ToothPosition::Top(0),
                    local_position: vec2(0.0, 0.0),
                    target: CrumbTarget::Local(vec2(0.0, 0.0)),
                }],
            },
            stick: Stick {
                position: vec2(
                    0.0,
                    -assets.config.face_radius + assets.config.stick_size.y / 2.0,
                ),
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
