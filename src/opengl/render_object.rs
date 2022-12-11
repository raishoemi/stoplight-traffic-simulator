use nalgebra::{Matrix4, Perspective3, Point3, Vector3, Vector4};

use super::buffer::{Buffer, BufferTarget, VertexArray};
use super::{Program, Renderable, Shader};

const COLOR_UNIFORM_NAME: &str = "colorUniform\0";
const MODEL_UNIFORM_NAME: &str = "modelUniform\0";
const VIEW_UNIFORM_NAME: &str = "viewUniform\0";
const PROJECTION_UNIFORM_NAME: &str = "projectionUniform\0";

pub struct RenderObject {
    pub position: Vector3<f32>,
    pub camera_position: Vector3<f32>,
    program: Program,
    vao: VertexArray,
    triangles_count: usize,
    color_uniform: gl::types::GLint,
    model_uniform: gl::types::GLint,
    view_uniform: gl::types::GLint,
    _projection_uniform: gl::types::GLint,
}

impl RenderObject {
    pub fn from_vertices(vertices: Vec<f32>, color: Vector4<f32>) -> Result<Self, String> {
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
        let position = Vector3::new(0.0, 0.0, 5.0);
        let translated_model = RenderObject::create_model_matrix(&position);
        program.set_4fv_uniform_value(model_uniform, translated_model);

        let camera_position = Vector3::new(0.0, 0.0, -2.0);
        program.set_4fv_uniform_value(
            view_uniform,
            RenderObject::create_view_matrix(&camera_position),
        );
        program.set_4fv_uniform_value(
            projection_uniform,
            *Perspective3::new(16.0 / 9.0, 3.14 / 4.0, 0.1, 15.0).as_matrix(),
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
            position,
            camera_position,
            program,
            vao,
            triangles_count,
            color_uniform,
            model_uniform,
            view_uniform,
            _projection_uniform: projection_uniform,
        })
    }

    pub fn set_color(&self, color: Vector4<f32>) {
        self.program.set_4f_uniform_value(self.color_uniform, color);
    }

    pub fn move_by(&mut self, offset: Vector3<f32>) {
        self.position += offset;
        self.program.set_4fv_uniform_value(
            self.model_uniform,
            RenderObject::create_model_matrix(&self.position),
        );
    }

    pub fn move_camera(&mut self, vector_offset: Vector3<f32>) {
        self.camera_position += vector_offset;
        self.program.set_4fv_uniform_value(
            self.view_uniform,
            RenderObject::create_view_matrix(&self.camera_position),
        );
    }

    fn create_model_matrix(position: &Vector3<f32>) -> Matrix4<f32> {
        // See https://solarianprogrammer.com/2013/05/22/opengl-101-matrices-projection-view-model/
        let mut identity = Matrix4::identity();
        identity.m14 = position.x;
        identity.m24 = position.y;
        identity.m34 = position.z;
        identity
    }

    fn create_view_matrix(position: &Vector3<f32>) -> Matrix4<f32> {
        let camera_position = Point3::new(position.x, position.y, position.z);
        let target = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let view_matrix = Matrix4::look_at_rh(&camera_position, &target, &up);
        view_matrix
    }
}

impl Renderable for RenderObject {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, self.triangles_count as gl::types::GLsizei) }
    }
}
