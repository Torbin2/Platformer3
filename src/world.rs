use crate::gl;

use crate::camera;
use crate::matrix;
use crate::render;

pub struct Object<'a> {
    pub position: matrix::Vector,
    pub rotation: matrix::Vector,
    pub scale: matrix::Vector,

    pub render_object: Option<&'a render::RenderObject>,
    pub shader: Option<&'a render::Shader>,
}

impl Object<'_> {
    pub fn model_to_world_matrix(&self) -> matrix::Matrix {
        matrix::Matrix::gen_scaling(self.scale.x, self.scale.y, self.scale.z)
            * matrix::Matrix::gen_rotation(1.0, 0.0, 0.0, self.rotation.x)
            * matrix::Matrix::gen_rotation(0.0, 1.0, 0.0, self.rotation.y)
            * matrix::Matrix::gen_rotation(0.0, 0.0, 1.0, self.rotation.z)
            * matrix::Matrix::gen_translation(self.position.x, self.position.y, self.position.z)
        // matrix::Matrix::gen_translation(self.position.x, self.position.y, self.position.z)
    }
}

pub struct Level<'a> {
    pub objects: Vec<Object<'a>>,
}

impl Level<'_> {
    pub fn load<File: std::io::Read + std::io::Seek>(file: File) -> Result<Self, std::io::Error> {
        todo!("Level::load()")
    }

    pub fn save<File: std::io::Write + std::io::Seek>(&self) -> Result<(), std::io::Error> {
        todo!("Level::save()")
    }

    pub fn render(&self, cam: &camera::Camera) -> () {
        let world_to_clip = cam.gen_world_to_clip();

        let mut last_shader = None;
        let mut transform_loc = -1;
        for object in self.objects.iter() {
            if let Some(render_object) = &object.render_object {
                let shader = object.shader.expect("Object in Level has a model but no shader");
                let m = object.model_to_world_matrix() * world_to_clip.clone();  // TODO: cloning the
                                                                                 // matrix should not be
                                                                                 // necessary

                if last_shader.is_none_or(|s| s != shader.shader_program) {
                    unsafe {
                        gl::UseProgram(shader.shader_program);

                        transform_loc = gl::GetUniformLocation(shader.shader_program, c"transform".as_ptr());
                        if transform_loc == -1 {
                            println!("Could not get \"transform\" uniform location");
                            return;
                        }
                    }

                    last_shader = Some(shader.shader_program);
                }

                assert_ne!(transform_loc, -1);
                unsafe {
                    gl::UniformMatrix4fv(transform_loc, 1, gl::TRUE, m.0.as_ptr());
                }

                render_object.render();
            }
        }
    }

    pub fn raycast(&self, cam: &camera::Camera, direction: matrix::Vector) {
        todo!("level.raycast()")
    }
}

