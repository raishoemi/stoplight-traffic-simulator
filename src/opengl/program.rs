use std::ffi::CString;

use super::{utils::create_whitespace_cstring_with_length, Shader};

const COLOR_UNIFORM_NAME: &str = "colorUniform";

pub struct Program {
    id: gl::types::GLuint,
    color_uniform: gl::types::GLint,
    gl: gl::Gl,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id);
            }
        }
        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_length(len as usize);
            unsafe {
                gl.GetProgramInfoLog(
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
                gl.DetachShader(program_id, shader.id);
            }
        }
        let uniform_name =
            CString::new(COLOR_UNIFORM_NAME).expect("Failed to create cstring for unifrom name");
        let uniform_location = unsafe {
            gl.GetUniformLocation(program_id, uniform_name.as_ptr())
        };
        Ok(Program {
            gl: gl.clone(),
            id: program_id,
            color_uniform: uniform_location,
        })
    }

    pub fn set_color_uniform(&self, value: nalgebra::Vector4<f32>) {
        println!("{:?}", self.color_uniform);
        unsafe {
            self.gl
                .Uniform4f(self.color_uniform, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}
