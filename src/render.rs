use crate::gl;

pub struct Shader {
    shader_program: gl::GLuint,
}

impl Shader {
    pub fn new(
        vertex_shader_src: &str,
        geometry_shader_src: Option<&str>,
        fragment_shader_src: Option<&str>,
    ) -> Result<(Self, String), String> {
        let mut info_log = String::new();

        let shader_program = unsafe { gl::CreateProgram() };
        if shader_program == 0 {
            return Err("Failed to create shader program".to_owned());
        }

        let vertex_shader   = Self::compile_shader_with_info_log(&mut info_log, shader_program, Some(vertex_shader_src), gl::VERTEX_SHADER)  ?.unwrap();
        let geometry_shader = Self::compile_shader_with_info_log(&mut info_log, shader_program, geometry_shader_src,     gl::GEOMETRY_SHADER)?;
        let fragment_shader = Self::compile_shader_with_info_log(&mut info_log, shader_program, fragment_shader_src,     gl::FRAGMENT_SHADER)?;

        unsafe { gl::LinkProgram(shader_program) };

        {
            let mut status = 0 as gl::GLint;
            unsafe { gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut status) };

            if status == gl::FALSE as i32 {
                let mut info_log_length = 0 as gl::GLint;
                unsafe { gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut info_log_length) };

                let mut program_info_log = Vec::<u8>::new();
                program_info_log.resize(info_log_length as usize, 0);
                unsafe {
                    let mut l = 0 as gl::GLint;
                    gl::GetProgramInfoLog(shader_program, info_log_length, &mut l, program_info_log.as_mut_ptr() as *mut core::ffi::c_char);
                    assert_ne!(info_log_length, 0);
                    assert_eq!(info_log_length, l + 1);
                }
                let r: String = std::ffi::CStr::from_bytes_until_nul(&program_info_log)
                    .expect("Opengl returned an invalid string as shader info log")
                    .to_str()
                    .expect("Opengl returned an invalid string as shader info log")
                    .to_string();

                info_log.push_str("Linking shader program failed\n");
                info_log.push_str(&r);
                info_log.push_str("\n");
                return Err(info_log);
            }
        }

        unsafe { gl::DeleteShader(vertex_shader) };
        if let Some(geometry_shader) = geometry_shader {
            unsafe { gl::DeleteShader(geometry_shader) };
        }
        if let Some(fragment_shader) = fragment_shader {
            unsafe { gl::DeleteShader(fragment_shader) };
        }

        return Ok((
                Self { shader_program },
                info_log
        ));
    }

    fn compile_shader_with_info_log(info_log: &mut String, shader_program: gl::GLuint, shader_src: Option<&str>, shader_type: gl::GLenum) -> Result<Option<gl::GLuint>, String> {
        let shader_type_str = match shader_type {
            gl::VERTEX_SHADER => "vertex shader",
            gl::GEOMETRY_SHADER => "geometry shader",
            gl::FRAGMENT_SHADER => "fragment shader",

            _ => unimplemented!("Unimplemented shader type"),
        };

        match shader_src {
            None => {
                return Ok(None);
            },
            Some(shader_src) => {
                let shader = unsafe { gl::CreateShader(shader_type) };
                if shader == 0 {
                    info_log.push_str(&format!("Error creating {}", shader_type_str));
                    return Err(info_log.to_string());
                }
                let s = Self::add_to_info_log(info_log, "geometry shader", Self::compile_shader(shader, shader_src));
                if let Err(e) = s {
                    return Err(e);
                }

                unsafe {gl::AttachShader(shader_program, shader) };

                return Ok(Some(shader));
            }
        };
    }

    fn compile_shader(shader: gl::GLuint, shader_src: &str) -> Result<String, String> {
        let src_p = shader_src.as_ptr();
        let mut info_log_length = 0 as gl::GLint;  // Length including null terminator
        let mut succes = 0 as gl::GLint;
        unsafe {
            gl::ShaderSource(shader, 1, &(src_p as *const core::ffi::c_char), &(shader_src.len() as gl::GLint));
            gl::CompileShader(shader);

            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut succes);
        }

        let mut info_log = Vec::<u8>::new();
        info_log.resize(info_log_length as usize, 0);
        unsafe {
            let mut l = 0 as gl::GLint;
            gl::GetShaderInfoLog(shader, info_log_length, &mut l, info_log.as_mut_ptr() as *mut core::ffi::c_char);
            if l != 0 {
                assert_eq!(info_log_length, l + 1);
            }
        }
        let r: String = if info_log_length > 0 {
            std::ffi::CStr::from_bytes_until_nul(&info_log)
                .expect("Opengl returned an invalid string as shader info log")
                .to_str()
                .expect("Opengl returned an invalid string as shader info log")
                .to_string()
        } else {
            "".to_string()
        };

        if succes == gl::TRUE as i32 {
            Ok(r)
        } else {
            Err(r)
        }
    }

    fn add_to_info_log(info_log: &mut String, shader_title: &str, result: Result<String, String>) -> Result<(), String> {
        let is_ok = result.is_ok();
        let l = match result {
            Ok(l) => l,
            Err(l) => l,
        };
        if l.len() > 0 {
            info_log.push_str("Compiling shader ");
            info_log.push_str(&format!("{:?}", shader_title));
            if is_ok {
                info_log.push_str(" succeeded\n");
            } else {
                info_log.push_str(" failed\n");
            }
            info_log.push_str(&l);
            info_log.push_str("\n");
        }

        if is_ok {
            Ok(())
        } else {
            Err(info_log.to_string())
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.shader_program) };
    }
}

