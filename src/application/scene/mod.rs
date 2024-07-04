use std::{ffi::c_void, mem};

use gl::types::{GLfloat, GLsizei, GLsizeiptr};

use super::shader::Program;

pub struct Scene {
    vao: u32,
}

impl Scene {
    pub fn new(program: &Program) -> Self {
        let vao = Self::init(program);
        Scene { vao: vao }
    }

    pub fn render(&mut self, program: &Program) {
        unsafe {
            program.use_program();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn init(program: &Program) -> u32 {
        let (mut vbo, mut vao) = (0, 0);
        unsafe {
            program.use_program();

            let vertices: [f32; 18] = [
                // positions     // colors
                0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
                -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
                0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
            ];
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }
        vao
    }
}
