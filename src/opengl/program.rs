use super::{utils::create_whitespace_cstring_with_length, Shader};

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id);
            }
        }
        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_length(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }
            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id);
            }
        }
        Ok(Program { id: program_id })
    }

    pub fn get_uniform_location(&self, name: *const gl::types::GLchar) -> gl::types::GLint {
        return unsafe { gl::GetUniformLocation(self.id, name) };
    }

    pub fn set_4f_uniform_value(
        &self,
        uniform_location: gl::types::GLint,
        value: nalgebra::Vector4<f32>,
    ) {
        unsafe {
            gl::Uniform4f(uniform_location, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_4fv_uniform_value(
        &self,
        uniform_location: gl::types::GLint,
        value: nalgebra::Matrix4<f32>,
    ) {
        unsafe {
            gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, value.as_ptr());
        }
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
