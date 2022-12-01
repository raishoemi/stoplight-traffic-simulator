use crate::opengl::{ColorBuffer, Renderable, Viewport};
use nalgebra as na;

extern crate gl;
extern crate sdl2;

pub struct Renderer {
    pub render_objects: Vec<Box<dyn Renderable>>,
    pub window: sdl2::video::Window,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub sdl: sdl2::Sdl,
}

impl Renderer {
    pub fn init() -> Renderer {
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
        window.gl_create_context().unwrap();
        Renderer {
            render_objects: Vec::new(),
            sdl,
            window,
            video_subsystem,
        }
    }

    pub fn render(&self) {
        let gl = gl::Gl::load_with(|s| {
            self.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
        color_buffer.set_used(&gl);

        let mut viewport = Viewport::for_window(900, 700);
        let mut event_pump = self.sdl.event_pump().unwrap();
        'render: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'render,
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
            for render_object in self.render_objects.iter() {
                render_object.render(&gl);
            }
            self.window.gl_swap_window();
        }
    }
}
