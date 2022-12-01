use crate::{
    geometry,
    opengl::{RenderObject, Renderable},
};

pub struct Game {
    rectangle: RenderObject,
}

impl Game {
    pub fn init(gl: &gl::Gl) -> Game {
        let vertices = geometry::cube(1.0);
        let rectangle =
            RenderObject::new(&gl, vertices, nalgebra::Vector4::from([0.5, 0.5, 0.3, 0.3]))
                .unwrap();
        Game { rectangle }
    }

    pub fn update(&self, delta_time: f32) {

        // update game state
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.rectangle.render(&gl);
    }
}
