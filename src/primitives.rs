use crate::render;

pub const OBJECT_VERTEX_SHADER: &str = "
#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 transform;

out vec3 local_coord;

void main () {
	gl_Position = transform * vec4(aPos, 1.0);
        local_coord = aPos;
}";

pub const CUBE_INDICES: &[u8] = &[
    // Back
    0, 1, 2,
    1, 3, 2,

    // Front
    4, 5, 6,
    5, 7, 6,

    // Top
    0, 1, 4,
    4, 5, 1,

    // Right face
    1, 5, 3,
    5, 7, 3,

    // Left face
    0, 4, 2,
    4, 6, 2,

    // Bottom
    2, 3, 6,
    6, 7, 3,
];

pub const CUBE_VERTICES: &[f32] = &[
    -1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
     1.0,  1.0, -1.0,

    -1.0, -1.0,  1.0,
     1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,
];

pub const TRIANGLE_INDICES: &[u8] = &[
    0, 1, 2
];
pub const TRIANGLE_VERTICES: &[f32] = &[
    0.0, 1.0, 0.0,
    // ((1.0 / 3.0) * std::f32::consts::TAU).sin(), ((1.0 / 3.0) * std::f32::consts::TAU).cos(), 0.0,
    0.8660254037844387, -0.5, 0.0,
    // ((2.0 / 3.0) * std::f32::consts::TAU).sin(), ((2.0 / 3.0) * std::f32::consts::TAU).cos(), 0.0,
    -0.8660254037844384, -0.5, 0.0
];
pub const TRIANGLE_VERTEX_ATTRIBUTES: &[render::VertexAttribute] = &[
    render::VertexAttribute {
        type_: render::VertexAttributeType::F32,
        component_amount: 3,
        normalized: false,
        stride: 0,
        pointer: 0
    }
];

