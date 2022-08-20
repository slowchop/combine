use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

/// A rectangle on the XY plane centered at the bottom middle.
#[derive(Debug, Copy, Clone)]
pub struct BottomQuad {
    pub size: Vec2,
}

impl BottomQuad {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

impl From<BottomQuad> for Mesh {
    fn from(quad: BottomQuad) -> Self {
        let extent_x = quad.size.x / 2.0;

        let (u_left, u_right) = { (0.0, 1.0) };
        let vertices = [
            ([-extent_x, 0.0, 0.0], [0.0, 0.0, 1.0], [u_left, 1.0]),
            (
                [-extent_x, quad.size.y, 0.0],
                [0.0, 0.0, 1.0],
                [u_left, 0.0],
            ),
            (
                [extent_x, quad.size.y, 0.0],
                [0.0, 0.0, 1.0],
                [u_right, 0.0],
            ),
            ([extent_x, 0.0, 0.0], [0.0, 0.0, 1.0], [u_right, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        for (position, normal, uv) in &vertices {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
