mod shader;

use gl::types::{GLchar, GLint, GLsizei};
use shader::Shader;

pub use shader::SHADER_DIR;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn graphics(vertex_shader: &String, fragment_shader: &String) -> Self {
        let vertex = Shader::new(vertex_shader, shader::ShaderType::VertexShader);
        let fragment = Shader::new(fragment_shader, shader::ShaderType::FragmentShader);

        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex.id);
            gl::AttachShader(id, fragment.id);
            gl::LinkProgram(id);
            Self::check_linking_error(id);
            gl::DeleteShader(vertex.id);
            gl::DeleteShader(fragment.id);
        };
        Program { id: id }
    }

    pub fn graphics_with_geom(
        vertex_shader: &String,
        fragment_shader: &String,
        geometry_shader: &String,
    ) -> Self {
        let vertex = Shader::new(vertex_shader, shader::ShaderType::VertexShader);
        let fragment = Shader::new(fragment_shader, shader::ShaderType::FragmentShader);
        let geometry = Shader::new(geometry_shader, shader::ShaderType::GeometryShader);

        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex.id);
            gl::AttachShader(id, fragment.id);
            gl::AttachShader(id, geometry.id);
            gl::LinkProgram(id);
            Self::check_linking_error(id);
            gl::DeleteShader(vertex.id);
            gl::DeleteShader(fragment.id);
            gl::DeleteShader(geometry.id);
        };

        Program { id: id }
    }

    pub fn graphics_with_tess(
        vertex_shader: &String,
        fragment_shader: &String,
        tess_evaluation_shader: &String,
        tess_control_shader: &String,
    ) -> Self {
        let vertex = Shader::new(vertex_shader, shader::ShaderType::VertexShader);
        let fragment = Shader::new(fragment_shader, shader::ShaderType::FragmentShader);
        let tesselation_evaluation = Shader::new(
            tess_evaluation_shader,
            shader::ShaderType::TesselationEvaluationShader,
        );
        let tesselation_control = Shader::new(
            tess_control_shader,
            shader::ShaderType::TesselationControlShader,
        );

        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex.id);
            gl::AttachShader(id, fragment.id);
            gl::AttachShader(id, tesselation_evaluation.id);
            gl::AttachShader(id, tesselation_control.id);
            gl::LinkProgram(id);
            Self::check_linking_error(id);
            gl::DeleteShader(vertex.id);
            gl::DeleteShader(fragment.id);
            gl::DeleteShader(tesselation_evaluation.id);
            gl::DeleteShader(tesselation_control.id);
        };

        Program { id: id }
    }

    pub fn graphics_with_tess_and_geom(
        vertex_shader: &String,
        fragment_shader: &String,
        tess_evaluation_shader: &String,
        tess_control_shader: &String,
        geometry_shader: &String,
    ) -> Self {
        let vertex = Shader::new(vertex_shader, shader::ShaderType::VertexShader);
        let fragment = Shader::new(fragment_shader, shader::ShaderType::FragmentShader);
        let tesselation_evaluation = Shader::new(
            tess_evaluation_shader,
            shader::ShaderType::TesselationEvaluationShader,
        );
        let tesselation_control = Shader::new(
            tess_control_shader,
            shader::ShaderType::TesselationControlShader,
        );
        let geometry = Shader::new(geometry_shader, shader::ShaderType::GeometryShader);

        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex.id);
            gl::AttachShader(id, fragment.id);
            gl::AttachShader(id, tesselation_evaluation.id);
            gl::AttachShader(id, tesselation_control.id);
            gl::AttachShader(id, geometry.id);
            gl::LinkProgram(id);
            Self::check_linking_error(id);
            gl::DeleteShader(vertex.id);
            gl::DeleteShader(fragment.id);
            gl::DeleteShader(tesselation_evaluation.id);
            gl::DeleteShader(tesselation_control.id);
            gl::DeleteShader(geometry.id);
        };

        Program { id: id }
    }

    unsafe fn check_linking_error(id: u32) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = vec![0u8; 1024];
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                id,
                info_log.len() as GLsizei,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            let log_str = std::str::from_utf8(&info_log)
                .unwrap()
                .trim_matches(char::from(0));
            println!("Failed to link the program:\n\t{}\n", log_str);
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }
}
