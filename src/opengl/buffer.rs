use gl;

pub struct Buffer {
    vbo: gl::types::GLuint,
    gl: gl::Gl,
}

impl Buffer {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }
        Buffer {
            gl: gl.clone(),
            vbo,
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, 0) }
    }

    pub fn set_buffer_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct VertexArray {
    gl: gl::Gl,
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe { gl.GenVertexArrays(1, &mut vao) }
        VertexArray {
            gl: gl.clone(),
            vao,
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.vao) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) }
    }

    pub fn enable_vertex_attribs(&self) {
        unsafe {
            self.gl.EnableVertexAttribArray(0);
            self.gl.VertexAttribPointer(
                0 as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<f32>() * 3) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &mut self.vao) }
    }
}
