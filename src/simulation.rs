use nalgebra::Vector3;
use sdl2::keyboard::Scancode;

use crate::{
    geometry,
    opengl::{RenderObject, Renderable},
};

pub struct Simulation {
    rectangle: RenderObject,
}

impl Simulation {
    pub fn init() -> Simulation {
        let vertices = geometry::cube_array_buffer(1.0);
        let rectangle =
            RenderObject::from_vertices(vertices, nalgebra::Vector4::from([0.5, 0.5, 0.3, 0.3]))
                .unwrap();
        Simulation { rectangle }
    }

    pub fn update(&mut self, event_pump: &sdl2::EventPump) {
        // match event_pump keyboard state
        event_pump
            .keyboard_state()
            .pressed_scancodes()
            .for_each(|scancode| match scancode {
                Scancode::D => self.rectangle.move_by(Vector3::new(0.01, 0.0, 0.0)),
                Scancode::A => self.rectangle.move_by(Vector3::new(-0.01, 0.0, 0.0)),
                Scancode::W => self.rectangle.move_by(Vector3::new(0.0, 0.01, 0.0)),
                Scancode::S => self.rectangle.move_by(Vector3::new(0.0, -0.01, 0.0)),

                Scancode::Y => self.rectangle.move_camera(Vector3::new(0.01, 0.0, 0.0)),
                Scancode::U => self.rectangle.move_camera(Vector3::new(-0.01, 0.0, 0.0)),
                Scancode::H => self.rectangle.move_camera(Vector3::new(0.0, 0.01, 0.0)),
                Scancode::J => self.rectangle.move_camera(Vector3::new(0.0, -0.01, 0.0)),
                Scancode::N => self.rectangle.move_camera(Vector3::new(0.0, 0.0, 0.01)),
                Scancode::M => self.rectangle.move_camera(Vector3::new(0.0, 0.0, -0.01)),
                _ => (),
            });
    }

    pub fn render(&self) {
        self.rectangle.render();
    }
}
