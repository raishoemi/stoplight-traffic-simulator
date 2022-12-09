use nalgebra::Vector3;

use crate::{
    geometry,
    opengl::{RenderObject, Renderable},
};

pub struct Simulation {
    rectangle: RenderObject,
}

impl Simulation {
    pub fn init() -> Simulation {
        let vertices = geometry::cube(1.0);
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
                sdl2::keyboard::Scancode::D => self.rectangle.set_position(Vector3::new(
                    self.rectangle.position.x + 0.01,
                    self.rectangle.position.y,
                    self.rectangle.position.z,
                )),
                sdl2::keyboard::Scancode::A => self.rectangle.set_position(Vector3::new(
                    self.rectangle.position.x - 0.01,
                    self.rectangle.position.y,
                    self.rectangle.position.z,
                )),
                sdl2::keyboard::Scancode::W => self.rectangle.set_position(Vector3::new(
                    self.rectangle.position.x,
                    self.rectangle.position.y + 0.01,
                    self.rectangle.position.z,
                )),
                sdl2::keyboard::Scancode::S => self.rectangle.set_position(Vector3::new(
                    self.rectangle.position.x,
                    self.rectangle.position.y - 0.01,
                    self.rectangle.position.z,
                )),
                _ => (),
            });
    }

    pub fn render(&self) {
        self.rectangle.render();
    }
}
