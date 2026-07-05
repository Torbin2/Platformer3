mod gl;
mod sdl3;
mod render;

fn gl_get_string(name: core::ffi::c_uint) -> &'static str {
    unsafe {
        core::ffi::CStr::from_ptr(gl::GetString(name) as *const i8)
            .to_str()
            .expect("GL returned invalid string")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    sdl3::init(sdl3::INIT_VIDEO)?;

    sdl3::gl_set_attribute(sdl3::GL_CONTEXT_MAJOR_VERSION, 3)?;
    sdl3::gl_set_attribute(sdl3::GL_CONTEXT_MINOR_VERSION, 3)?;
    sdl3::gl_set_attribute(sdl3::GL_CONTEXT_PROFILE_MASK, sdl3::GL_CONTEXT_PROFILE_CORE)?;

    let mut window = sdl3::Window::create(
        c"Platformer 3",
        800, 800,
        sdl3::WINDOW_RESIZABLE | sdl3::WINDOW_MAXIMIZED | sdl3::WINDOW_OPENGL)?;
    let mut _gl_context = window.gl_create_context()?;

    println!("OpenGL Vendor:   {}", gl_get_string(gl::VENDOR));
    println!("OpenGL Renderer: {}", gl_get_string(gl::RENDERER));
    println!("OpenGL Version:  {}", gl_get_string(gl::VERSION));
    println!("GLSL Version:    {}", gl_get_string(gl::SHADING_LANGUAGE_VERSION));

    // Enable vsync
    sdl3::gl_set_swap_interval(1)?;

    {
        let (width, height) = window.get_size_in_pixels();
        unsafe { gl::Viewport(0, 0, width, height); }
    }

    let shader_program = match render::Shader::new(
        "
#version 330 core
layout (location = 0) in vec3 aPos;

void main () {
	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}", None, None
    ) {
        Ok((shader_program, info_log)) => {
            if info_log.len() > 0 {
                print!("Shader program compiled succesfully:\n{}", info_log);
            }
            shader_program
        }
        Err(info_log) => {
            print!("Shader program compilation failed:\n{}", info_log);
            return Err(info_log.into());
        }
    };

    'main_loop: loop {
        while let Some(event) = sdl3::poll_event() {
            match event {
                sdl3::events::Event::SDL_EVENT_QUIT(_) => { break 'main_loop; },
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        window.swap()?;
    }

    return Ok(());
}

