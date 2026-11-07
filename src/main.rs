mod gl;
mod sdl3;

mod camera;
mod matrix;
mod primitives;
mod render;
mod world;

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
        primitives::OBJECT_VERTEX_SHADER, None, Some("
#version 330 core
out vec4 FragColor;

in vec3 local_coord;

void main() {
    // FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    FragColor = vec4(local_coord, 1.0f);
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

    let vertex_attributes = vec![
        render::VertexAttribute{
            type_: render::VertexAttributeType::F32,
            component_amount: 3,
            normalized: false,
            stride: 0,
            pointer: 0
        }
    ];
    let cube = render::RenderObject::new(
        render::Indices::U8(primitives::CUBE_INDICES),
        render::vertices_to_u8_array(primitives::CUBE_VERTICES),
        &vertex_attributes
    )?;

    let triangle = render::RenderObject::new(
        render::Indices::U8(primitives::TRIANGLE_INDICES),
        render::vertices_to_u8_array(primitives::TRIANGLE_VERTICES),
        &vertex_attributes
    )?;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut cam = camera::Camera {
        position: matrix::Vector { x: 0.0, y: 0.0, z: 0.0 },
        direction: matrix::Vector { x: 0.0, y: 0.0, z: 0.0 },

        aspect_ratio: 1.0
    };

    let mut level = world::Level::new()?;
    level.objects.push(
        world::Object {
            position: matrix::Vector { x: 3.0, y: 0.0, z: 0.0 },
            rotation: matrix::Vector { x: 0.0, y: 0.0, z: 0.05 * 3.141 },
            scale: matrix::Vector { x: 1.0, y: 1.0, z: 1.0 },

            render_object: Some(&cube),
            shader: Some(&shader_program)
        }
    );

    let mut frame_start = std::time::Instant::now();

    let mut width = 1;
    let mut height = 1;

    let mut menu_enabled = false;
    window.set_relative_mouse_mode(true)?;

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
                sdl3::events::Event::SDL_EVENT_KEY_DOWN { scancode, repeat, .. } => {
                    match scancode.0 {
                        sdl3::keys::SCANCODE_ESCAPE => {
                            if !repeat {
                                if !menu_enabled {
                                    menu_enabled = true;

                                    window.warp_mouse(width as f32 / 2.0, height as f32 / 2.0);

                                    window.set_relative_mouse_mode(false)?;
                                } else {
                                    menu_enabled = false;

                                    window.set_relative_mouse_mode(true)?;
                                    sdl3::get_relative_mouse_state();
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        cam.aspect_ratio = width as f32 / height as f32;

        let mut keys = [0; sdl3::keys::SCANCODE_COUNT as usize];
        sdl3::get_keyboard_state(&mut keys);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if !menu_enabled {
            if keys[sdl3::keys::SCANCODE_S as usize] != 0 {
                cam.move_in_view_space(0.0, 0.0, 0.01 * dt);
            }
            if keys[sdl3::keys::SCANCODE_W as usize] != 0 {
                cam.move_in_view_space(0.0, 0.0, -0.01 * dt);
            }

            if keys[sdl3::keys::SCANCODE_A as usize] != 0 {
                cam.move_in_view_space(0.01 * dt, 0.0, 0.0);
            }
            if keys[sdl3::keys::SCANCODE_D as usize] != 0 {
                cam.move_in_view_space(-0.01 * dt, 0.0, 0.0);
            }

            if keys[sdl3::keys::SCANCODE_SPACE as usize] != 0 {
                cam.move_in_view_space(0.0, 0.01 * dt, 0.0);
            }
            if keys[sdl3::keys::SCANCODE_LCTRL as usize] != 0 {
                cam.move_in_view_space(0.0, -0.01 * dt, 0.0);
            }

            let (mouse_dx, mouse_dy, _mouse_flags) = sdl3::get_relative_mouse_state();
            {
                let mult = 1000.0;
                cam.direction.x -= mouse_dy as f32 / mult;
                cam.direction.y -= mouse_dx as f32 / mult;

                cam.direction.x = cam.direction.x.min(std::f32::consts::PI / 2.0).max(-std::f32::consts::PI / 2.0);
            }
        }

        if menu_enabled {
            level.render(&cam);

            let (mouse_x, mouse_y, _mouse_flags) = sdl3::get_mouse_state();
            level.render_centers(&cam, (mouse_x / width as f32, mouse_y / height as f32));
        } else {
            level.render(&cam);
        }

        {
            unsafe {
                gl::UseProgram(shader_program.shader_program);
            }

            let m = matrix::Matrix::gen_rotation(0.0, 0.0, 1.0, rect_rotation) * cam.gen_world_to_clip();
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
        }

        window.swap()?;
    }

    return Ok(());
}

