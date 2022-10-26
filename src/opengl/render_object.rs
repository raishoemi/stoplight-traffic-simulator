use super::buffer::{Buffer, VertexArray};
use super::{Program, Shader};

pub struct RenderObject {
    program: Program,
    vao: VertexArray,
    triangles_count: usize
}

impl RenderObject {
    pub fn new(gl: &gl::Gl, vertices: Vec<f32>, color: nalgebra::Vector4<f32>) -> Result<Self, String> {
        let vert_shader = Shader::from_vert_source(&gl, "assets/triangle.vert").expect("Failed to load vertex shader");
        let frag_shader = Shader::from_frag_source(&gl, "assets/triangle.frag").expect("Failed to load fragment shader");
        let program = Program::from_shaders(&gl, &[vert_shader, frag_shader]).expect("Failed to create program from shaders");
        let triangles_count = vertices.len() / 3;
        program.set_used();
        program.set_color_uniform(color);
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
            triangles_count
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe { gl.DrawArrays(gl::TRIANGLES, 0, self.triangles_count as gl::types::GLsizei) }
    }
}
