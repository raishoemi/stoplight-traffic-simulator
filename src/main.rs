use opengl::RenderObject;
use renderer::Renderer;

extern crate gl;
extern crate sdl2;

mod geometry;
pub mod opengl;
mod renderer;

fn main() {
    let mut renderer: Renderer = Renderer::init();

    // _gl_context and gl loading must be created here so we can pass the same reference to
    // the render objects and the renderer. Ideally, this would be done in the renderer
    // but due to rust borrowing rules, it's very difficult to do so.
    let _gl_context = renderer.window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        renderer.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let vertices = geometry::cube(1.0);
    let triangle =
        RenderObject::new(&gl, vertices, nalgebra::Vector4::from([0.5, 0.5, 0.3, 0.3])).unwrap();
    renderer.render_objects.push(Box::new(triangle));

    renderer.render(&gl);
}
