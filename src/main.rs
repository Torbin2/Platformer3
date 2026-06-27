mod gl;
mod sdl3;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut window = sdl3::Window::create()?;
    let mut gl_context = window.gl_create_context()?;
    println!("Hello, world!");

    return Ok(());
}
