use super::buffer::{Buffer, VertexArray};
use super::{Program, Shader};

const COLOR_UNIFORM_NAME: &str = "colorUniform\0";

pub struct RenderObject {
    program: Program,
    vao: VertexArray,
    triangles_count: usize,
    color_uniform: gl::types::GLint,
}

impl RenderObject {
    pub fn new(
        gl: &gl::Gl,
        vertices: Vec<f32>,
        color: nalgebra::Vector4<f32>,
    ) -> Result<Self, String> {
        let vert_shader = Shader::from_vert_source(&gl, "assets/triangle.vert")
            .expect("Failed to load vertex shader");
        let frag_shader = Shader::from_frag_source(&gl, "assets/triangle.frag")
            .expect("Failed to load fragment shader");
        let program = Program::from_shaders(&gl, &[vert_shader, frag_shader])
            .expect("Failed to create program from shaders");
        let triangles_count = vertices.len() / 3;
        program.set_used();
        let color_uniform = program.get_uniform_location(
            COLOR_UNIFORM_NAME.to_string().as_ptr() as *const gl::types::GLchar
        );
        program.set_v4_uniform_value(color_uniform, color);
        let vbo = Buffer::new(&gl);
        vbo.bind();
        vbo.set_buffer_data(&vertices, Some(triangles_count));
        vbo.unbind();
        let vao = VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        vao.enable_vertex_attribs();
        vbo.unbind();
        vao.unbind();
        Ok(RenderObject {
            program,
            vao,
            triangles_count,
            color_uniform,
        })
    }

    pub fn set_color(&self, color: nalgebra::Vector4<f32>) {
        self.program.set_v4_uniform_value(self.color_uniform, color);
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe { gl.DrawArrays(gl::TRIANGLES, 0, self.triangles_count as gl::types::GLsizei) }
    }
}
