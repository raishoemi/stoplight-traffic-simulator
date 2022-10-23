use nalgebra as na;
use opengl::{Program, RenderObject, Shader};

extern crate gl;
extern crate sdl2;

mod geometry;
pub mod opengl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Double Pendulum", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let color_buffer = opengl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used(&gl);
    let vert_shader = Shader::from_vert_source(&gl, "assets/triangle.vert").unwrap();
    let frag_shader = Shader::from_frag_source(&gl, "assets/triangle.frag").unwrap();
    let program = Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();
    let vertices = geometry::cube(1.0, (1.0, 0.0, 0.0));
    let triangle = RenderObject::new(program, &gl, vertices).unwrap();

    let mut viewport = opengl::Viewport::for_window(900, 700);
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                _ => {}
            }
        }
        color_buffer.clear(&gl);
        triangle.render(&gl);
        window.gl_swap_window();
    }
}
