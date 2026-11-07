use crate::gl;

use crate::camera;
use crate::matrix;
use crate::primitives;
use crate::render;
use crate::render::RenderObject;

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

    triangle: render::RenderObject,
    triangle_shader: render::Shader
}

const CIRCLE_VERTEX_SHADER: &str = "
#version 330 core
out vec4 FragColor;

in vec3 local_coord;

uniform vec3 color;

void main() {
    if (length(local_coord) > 0.5) {
        discard;
    }
    FragColor = vec4(color, 1.0);
}
";

impl Level<'_> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Level {
            objects: vec![],

            triangle: RenderObject::new(
                render::Indices::U8(primitives::TRIANGLE_INDICES),
                render::vertices_to_u8_array(primitives::TRIANGLE_VERTICES),
                primitives::TRIANGLE_VERTEX_ATTRIBUTES
            )?,
            triangle_shader: render::Shader::print_shader_info(render::Shader::new(
                primitives::OBJECT_VERTEX_SHADER, None, Some(CIRCLE_VERTEX_SHADER)
            ))?
        })
    }

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

                        transform_loc = shader.get_uniform_location(c"transform");
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

    pub fn render_centers(&self, cam: &camera::Camera, mut mouse_pos: (f32, f32)) -> Option<&Object<'_>> {
        let transform_loc = self.triangle_shader.get_uniform_location(c"transform");
        let color_loc = self.triangle_shader.get_uniform_location(c"color");  // TODO: Set color and
                                                                              // render
        unsafe {
            gl::UseProgram(self.triangle_shader.shader_program);
        }

        let world_to_clip = cam.gen_world_to_clip();

        let circle_scale: f32 = 0.04;
        mouse_pos.0 = mouse_pos.0 * 2.0 - 1.0;
        mouse_pos.1 = -(mouse_pos.1 * 2.0 - 1.0);

        let triangle_scale = circle_scale * 2.0;
        let render_distance = 25.0;

        let mut selection: Option<&Object> = None;

        for object in self.objects.iter() {
            if (cam.position - object.position).length() > render_distance {
                continue;
            }

            let m = object.model_to_world_matrix() * world_to_clip.clone();
            let (clip_space_pos, perspective_divisor) = m.apply_vector(object.position, 1.0);

            let clip_space_pos = clip_space_pos / perspective_divisor;

            let dist_sq =
                (clip_space_pos.x - mouse_pos.0).powi(2)
                + (clip_space_pos.y - mouse_pos.1).powi(2);
            let color = if selection.is_none() && dist_sq < circle_scale.powi(2) {
                selection = Some(&object);
                matrix::Vector { x: 1.0, y: 1.0, z: 1.0 }
            } else {
                matrix::Vector{ x: 1.0, y: 0.0, z: 0.0 }
            };
            unsafe {
                gl::Uniform3f(color_loc, color.x, color.y, color.z);
            }

            let m = matrix::Matrix::gen_scaling((1.0 / cam.aspect_ratio) * triangle_scale, triangle_scale, triangle_scale)
                * matrix::Matrix::gen_translation(clip_space_pos.x, clip_space_pos.y, -1.0);
            unsafe {
                gl::UniformMatrix4fv(transform_loc, 1, gl::TRUE, m.0.as_ptr());
            }

            self.triangle.render();
        }

        selection
    }

    pub fn raycast(&self, cam: &camera::Camera, direction: matrix::Vector) {
        todo!("level.raycast()")
    }
}

