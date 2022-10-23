use super::buffer::{Buffer, VertexArray};
use super::Program;

pub struct RenderObject {
    program: Program,
    _vbo: Buffer,
    vao: VertexArray,
    triangles_count: u32
}

impl RenderObject {
    pub fn new(program: Program, gl: &gl::Gl, vertices: Vec<f32>) -> Result<Self, String> {
        let vbo = Buffer::new(&gl);
        vbo.bind();
        vbo.set_buffer_data(&vertices);
        vbo.unbind();
        let vao = VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        vao.enable_vertex_attribs();
        vbo.unbind();
        vao.unbind();
        Ok(RenderObject {
            program,
            _vbo: vbo,
            vao,
            triangles_count: ((vertices.len() / 6) as u32)
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe { gl.DrawArrays(gl::TRIANGLES, 0, self.triangles_count as gl::types::GLsizei) }
    }
}
