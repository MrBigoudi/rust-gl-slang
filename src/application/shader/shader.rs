use std::{
    ffi::CString,
    fs::File,
    io::Read,
    ptr::{self},
};

use gl::types::{GLchar, GLint, GLsizei};

pub enum ShaderType {
    VertexShader,
    FragmentShader,
    GeometryShader,
    TesselationControlShader,
    TesselationEvaluationShader,
    ComputeShader,
}

pub(super) struct Shader {
    pub path: String,
    pub id: u32,
}

pub const SHADER_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/shaders/");

impl Shader {
    pub fn new(path: &String, shader_type: ShaderType) -> Self {
        let id = Self::create_shader(shader_type);
        let code = Self::load_shader_code(path);
        Self::compile_shader(path, &code, id);
        Shader {
            path: path.clone(),
            id: id,
        }
    }

    fn create_shader(shader_type: ShaderType) -> u32 {
        match shader_type {
            ShaderType::VertexShader => unsafe { gl::CreateShader(gl::VERTEX_SHADER) },
            ShaderType::FragmentShader => unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) },
            ShaderType::GeometryShader => unsafe { gl::CreateShader(gl::GEOMETRY_SHADER) },
            ShaderType::TesselationControlShader => unsafe {
                gl::CreateShader(gl::TESS_CONTROL_SHADER)
            },
            ShaderType::TesselationEvaluationShader => unsafe {
                gl::CreateShader(gl::TESS_EVALUATION_SHADER)
            },
            ShaderType::ComputeShader => unsafe { gl::CreateShader(gl::COMPUTE_SHADER) },
        }
    }

    fn load_shader_code(path: &String) -> CString {
        let mut shader_file =
            File::open(path).unwrap_or_else(|err| panic!("Failed to open {}: {:?}\n", path, err));
        let mut code = String::new();
        shader_file
            .read_to_string(&mut code)
            .unwrap_or_else(|err| panic!("Failed to read shader {}: {:?}\n", path, err));

        CString::new(code.as_bytes()).unwrap()
    }

    fn compile_shader(path: &String, code: &CString, id: u32) {
        unsafe {
            gl::ShaderSource(id, 1, &code.as_ptr(), ptr::null());
            gl::CompileShader(id);
            Self::check_compile_error(path, id);
        }
    }

    unsafe fn check_compile_error(path: &String, id: u32) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = vec![0u8; 1024];
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                id,
                info_log.len() as GLsizei,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            let log_str = std::str::from_utf8(&info_log)
                .unwrap()
                .trim_matches(char::from(0));
            println!("Failed to compile shader {}:\n\t{}\n", path, log_str);
        }
    }
}
