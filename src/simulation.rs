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

    pub fn update(&self) {}

    pub fn render(&self) {
        self.rectangle.render();
    }
}
