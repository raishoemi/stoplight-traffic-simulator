use std::{
    ffi::{CString},
    fs,
};

use super::utils::create_whitespace_cstring_with_length;

pub struct Shader {
    pub id: gl::types::GLuint,
}

impl Shader {
    pub fn from_vert_source(path: &str) -> Result<Shader, String> {
        Shader::from_source(path, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(path: &str) -> Result<Shader, String> {
        Shader::from_source(path, gl::FRAGMENT_SHADER)
    }

    fn from_source(path: &str, kind: gl::types::GLenum) -> Result<Self, String> {
        let source = CString::new(fs::read_to_string(path).or(Err("Failed to read shader file"))?)
            .or(Err("Failed to convert String to CString"))?;
        let id: u32 = unsafe { gl::CreateShader(kind) };
        unsafe {
            // TODO: Try `ShaderSouce` with `length` instead of null to avoid using null-terminated C-strings
            // See: https://registry.khronos.org/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error: CString = create_whitespace_cstring_with_length(len as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }
            return Err(error.to_string_lossy().into_owned());
        }
        return Ok(Shader { id });
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
