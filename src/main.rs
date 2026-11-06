mod gl;
mod sdl3;
mod render;

mod matrix;
mod camera;

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

uniform mat4 transform;

void main () {
	gl_Position = transform * vec4(aPos, 1.0);
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

    let indices = vec![0, 1, 2,  1, 3, 2];
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, -10.0,
         0.5, -0.5, -10.0,
         -0.5,  0.5, -10.0,
         0.5, 0.5, -10.0
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

    // unsafe {
    //     gl::Enable(gl::DEPTH_TEST);
    // }

    let mut cam = camera::Camera {
        position: matrix::Vector { x: 0.0, y: 0.0, z: 0.0 },
        direction: matrix::Vector { x: 0.0, y: 0.0, z: 0.0 }
    };

    let mut frame_start = std::time::Instant::now();

    let mut width = 1;
    let mut height = 1;
    let mut rect_rotation = 0.0;
    'main_loop: loop {
        let dt = frame_start.elapsed().as_nanos() as f32 / 1.0e6;
        let _fps = 1000.0 / dt;

        frame_start = std::time::Instant::now();
        while let Some(event) = sdl3::poll_event() {
            match event {
                sdl3::events::Event::SDL_EVENT_QUIT { .. } => { break 'main_loop; },
                sdl3::events::Event::SDL_EVENT_WINDOW_RESIZED { data1: w, data2: h, .. } => {
                    unsafe { gl::Viewport(0, 0, w, h) };
                    width = w;
                    height = h;
                },
                _ => {}
            }
        }

        let mut keys = [0; sdl3::keys::SCANCODE_COUNT as usize];
        sdl3::get_keyboard_state(&mut keys);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        unsafe {
            gl::UseProgram(shader_program.shader_program);
        }

        // let m = matrix::Matrix::gen_perspective(3.141 / 4.0, width as f32 / height as f32, 0.1, 100.0);
        if keys[sdl3::keys::SCANCODE_S as usize] != 0 {
            cam.position.z += 0.01 * dt;
        }
        if keys[sdl3::keys::SCANCODE_W as usize] != 0 {
            cam.position.z -= 0.01 * dt;
        }
        let m = matrix::Matrix::gen_rotation(0.0, 0.0, 1.0, rect_rotation) * cam.gen_world_to_clip(width as f32 / height as f32);
        rect_rotation += 0.001 * dt;
        unsafe {
            let transform_loc = gl::GetUniformLocation(shader_program.shader_program, c"transform".as_ptr());
            if transform_loc == -1 {
                println!("Could not get \"transform\" uniform location");
            } else {
                gl::UniformMatrix4fv(transform_loc, 1, gl::TRUE, m.0.as_ptr());
            }
        }

        triangle.render();

        window.swap()?;
    }

    return Ok(());
}

