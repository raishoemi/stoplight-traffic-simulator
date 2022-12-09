use nalgebra::Vector3;

use super::buffer::{Buffer, VertexArray, BufferTarget};
use super::{Program, Renderable, Shader};

const COLOR_UNIFORM_NAME: &str = "colorUniform\0";
const MODEL_UNIFORM_NAME: &str = "modelUniform\0";
const VIEW_UNIFORM_NAME: &str = "viewUniform\0";
const PROJECTION_UNIFORM_NAME: &str = "projectionUniform\0";

pub struct RenderObject {
    program: Program,
    vao: VertexArray,
    triangles_count: usize,
    color_uniform: gl::types::GLint,
    model_uniform: gl::types::GLint,
    view_uniform: gl::types::GLint,
    _projection_uniform: gl::types::GLint,
}

impl RenderObject {
    pub fn from_vertices(
        vertices: Vec<f32>,
        color: nalgebra::Vector4<f32>,
    ) -> Result<Self, String> {
        let vert_shader =
            Shader::from_vert_source("assets/triangle.vert").expect("Failed to load vertex shader");
        let frag_shader = Shader::from_frag_source("assets/triangle.frag")
            .expect("Failed to load fragment shader");
        let program = Program::from_shaders(&[vert_shader, frag_shader])
            .expect("Failed to create program from shaders");
        let triangles_count = vertices.len() / 3;
        program.set_used();
        let color_uniform = program.get_uniform_location(
            COLOR_UNIFORM_NAME.to_string().as_ptr() as *const gl::types::GLchar
        );
        let model_uniform = program.get_uniform_location(
            MODEL_UNIFORM_NAME.to_string().as_ptr() as *const gl::types::GLchar
        );
        let view_uniform = program.get_uniform_location(
            VIEW_UNIFORM_NAME.to_string().as_ptr() as *const gl::types::GLchar
        );
        let projection_uniform = program.get_uniform_location(
            PROJECTION_UNIFORM_NAME.to_string().as_ptr() as *const gl::types::GLchar,
        );
        program.set_4f_uniform_value(color_uniform, color);
        let translated_model =
            RenderObject::get_translation_matrix(nalgebra::Vector3::new(0.0, 0.0, 3.0));
        program.set_4fv_uniform_value(model_uniform, translated_model);
        program.set_4fv_uniform_value(
            view_uniform,
            nalgebra::Matrix4::from_diagonal(&nalgebra::Vector4::new(1.0, 1.0, -2.0, 1.0)),
        );
        program.set_4fv_uniform_value(
            projection_uniform,
            *nalgebra::Perspective3::new(16.0 / 9.0, 3.14 / 4.0, 0.1, 15.0).as_matrix(),
        );
        let vbo = Buffer::new(BufferTarget::ArrayBuffer);
        vbo.bind();
        vbo.set_buffer_data(&vertices, Some(triangles_count));
        vbo.unbind();
        let vao = VertexArray::new();
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
            model_uniform,
            view_uniform,
            _projection_uniform: projection_uniform,
        })
    }

    pub fn set_color(&self, color: nalgebra::Vector4<f32>) {
        self.program.set_4f_uniform_value(self.color_uniform, color);
    }

    pub fn set_model_matrix(&self, offset: Vector3<f32>) {
        self.program.set_4fv_uniform_value(
            self.model_uniform,
            RenderObject::get_translation_matrix(offset),
        );
    }

    pub fn set_view_matrix(&self, view_vector: nalgebra::Vector4<f32>) {
        self.program.set_4fv_uniform_value(
            self.view_uniform,
            nalgebra::Matrix4::from_diagonal(&view_vector),
        );
    }

    fn get_translation_matrix(offset: Vector3<f32>) -> nalgebra::Matrix4<f32> {
        // See https://solarianprogrammer.com/2013/05/22/opengl-101-matrices-projection-view-model/
        let mut identity = nalgebra::Matrix4::identity();
        identity.m14 = offset.x;
        identity.m24 = offset.y;
        identity.m34 = offset.z;
        identity
    }
}

impl Renderable for RenderObject {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, self.triangles_count as gl::types::GLsizei) }
    }
}
