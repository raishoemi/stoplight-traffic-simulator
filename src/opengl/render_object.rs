use super::buffer::{Buffer, VertexArray};
use super::Program;

pub struct RenderObject {
    program: Program,
    _vbo: Buffer,
    vao: VertexArray,
}

impl RenderObject {
    pub fn new(program: Program, gl: &gl::Gl) -> Result<Self, String> {
        let vertices: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            1.0, 0.0, 0.0,
            0.5, -0.5, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.5, 0.0,
            0.0, 0.0, 1.0
        ];
        let vbo = Buffer::new(&gl);
        vbo.bind();
        vbo.set_buffer_data(&vertices);
        vbo.unbind();
        let vao = VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        unsafe {
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                0 as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<f32>() * 6) as gl::types::GLint,
                std::ptr::null()
            );
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(
                1 as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<f32>() * 6) as gl::types::GLint,
                (std::mem::size_of::<f32>() * 3) as *const gl::types::GLvoid,
            );
        }
        vbo.unbind();
        vao.unbind();
        Ok(RenderObject {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe { gl.DrawArrays(gl::TRIANGLES, 0, 3) }
    }
}
