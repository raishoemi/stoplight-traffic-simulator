mod buffer;
mod color_buffer;
mod program;
mod render_object;
mod shader;
mod utils;
mod viewport;

pub use color_buffer::ColorBuffer;
pub use program::Program;
pub use render_object::RenderObject;
pub use shader::Shader;
pub use viewport::Viewport;

pub trait Renderable {
    fn render(&self);
}
