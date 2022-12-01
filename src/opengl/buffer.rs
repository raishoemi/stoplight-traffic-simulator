pub struct Buffer {
    vbo: gl::types::GLuint,
}

impl Buffer {
    pub fn new() -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        Buffer {
            vbo,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) }
    }

    pub fn set_buffer_data<T>(&self, data: &[T], triangles_count: Option<usize>) {
        let data_len = match triangles_count {
            None => data.len(),
            Some(x) => (x * 3) as usize,
        };
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data_len * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao) }
        VertexArray {
            vao,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    pub fn enable_vertex_attribs(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
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
        unsafe { gl::DeleteVertexArrays(1, &mut self.vao) }
    }
}
