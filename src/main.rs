mod gl;
mod sdl3;
mod render;

fn gl_get_string(name: core::ffi::c_uint) -> &'static str {
    unsafe {
        core::ffi::CStr::from_ptr(gl::GetString(name))
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
}", None, Some("
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}")
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

    let indices = vec![0, 1, 2];
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0
    ];
    let vertex_attributes = vec![
        render::VertexAttribute{
            type_: render::VertexAttributeType::F32,
            component_amount: 3,
            normalized: false,
            stride: 0,
            pointer: 0
        }
    ];
    let mut triangle = render::RenderObject::new(
        render::Indices::U8(&indices),
        unsafe { std::slice::from_raw_parts(
            vertices.as_slice().as_ptr() as *const u8,
            vertices.len() * std::mem::size_of::<f32>()
        ) },
        &vertex_attributes
    )?;

    'main_loop: loop {
        while let Some(event) = sdl3::poll_event() {
            match event {
                sdl3::events::Event::SDL_EVENT_QUIT { .. } => { break 'main_loop; },
                sdl3::events::Event::SDL_EVENT_WINDOW_RESIZED { data1: width, data2: height, .. } => {
                    unsafe { gl::Viewport(0, 0, width, height) };
                },
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        unsafe {
            gl::UseProgram(shader_program.shader_program);
        }
        triangle.render();

        window.swap()?;
    }

    return Ok(());
}

