use gl::types::*;

use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::time;

use std::error::Error;

pub struct Program {
    program: GLuint,
}

impl Program {
    pub fn new(vert_source: &str, frag_source: &str) -> Self {
        let vert_shader;
        unsafe {
            vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            // Attempt to compile the shader
            let c_str = CString::new(vert_source.as_bytes()).unwrap();
            gl::ShaderSource(vert_shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(vert_shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(vert_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    vert_shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                    );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
                    );
            }
        }
        let frag_shader;
        unsafe {
            frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            // Attempt to compile the shader
            let c_str = CString::new(frag_source.as_bytes()).unwrap();
            gl::ShaderSource(frag_shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(frag_shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(frag_shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(frag_shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    frag_shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                    );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
                    );
            }
        }
        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vert_shader);
            gl::AttachShader(program, frag_shader);
            gl::LinkProgram(program);
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                    );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
                    );
            }
        }
        Program { program }
    }
    pub fn enable(&mut self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}
