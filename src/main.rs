use nalgebra as na;
use opengl::{ColorBuffer, Viewport};
use sdl2::pixels::Color;

extern crate gl;
extern crate sdl2;

mod geometry;
pub mod opengl;
mod simulation;

// TODO: This should be an `engine` library crate, not a binary crate.
fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("Whatever!", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    //
    let ttf_context = sdl2::ttf::init().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut font = ttf_context
        .load_font("C:\\Windows\\Fonts\\arial.ttf", 128)
        .unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let surface = font
        .render("Hello rust!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .unwrap();
    let _texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    // unsafe {
    //     _texture.gl_bind_texture();
    // }

    let _gl_context = canvas.window().gl_create_context().unwrap(); // Must be assigned to variable, else it will be dropped immediately
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used();

    let mut game = simulation::Simulation::init();
    // TODO: should implement framewith with delta time

    let mut viewport = Viewport::for_window(900, 700);
    let mut event_pump = sdl.event_pump().unwrap();
    'render: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'render,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used();
                }
                _ => {}
            }
        }
        game.update(&event_pump);

        color_buffer.clear();
        game.render();
        canvas.window().gl_swap_window();
    }
}
