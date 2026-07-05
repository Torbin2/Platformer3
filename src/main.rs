mod gl;
mod sdl3;

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
    println!("Hello, world!");

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

    'main_loop: loop {
        // TODO: Event loop

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        window.swap()?;
    }

    return Ok(());
}
