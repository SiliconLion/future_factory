use crate::rendering::geometry::{Geometry, TexPoint};
use crate::rendering::texture::{Texture};
use crate::rendering::shader::Shader;
use std::ffi::CString;

pub enum TileType {
    Factory(Factory),
    Empty,
    Pipe(Pipe)
}

pub enum Factory {
    Red,
    Green,
    Blue
}

pub enum Pipe {
    Red,
    Green, 
    Blue
}


pub struct Tile {
    pub kind: TileType,
    pub row: i32,
    pub col: i32,
    pub geometry: Geometry<TexPoint>,
    // pub plain_texture: Texture,
    // pub stencil_texture: Texture
}

impl Tile {
    pub fn new(row: i32, col: i32) -> Tile {
        let x = col as f32;
        let y = row as f32;
        let z = 0.0;
        let width = 1.0; let height = 1.0;
        let vertices = [
            TexPoint { location: [x, y, z], tex_coord: [0.0, 1.0]}, // top left
            TexPoint { location: [x, y - height, z], tex_coord: [0.0, 0.0]}, //bottom left
            TexPoint { location: [x + width, y, z], tex_coord: [1.0, 1.0]}, // top right
            TexPoint { location: [x + width, y - height, z], tex_coord: [1.0, 0.0]} //bottom right
        ];

        let indices = [0, 1, 2, 3]; //triangle strip indices.

        let geometry = unsafe { Geometry::from_verts_and_indices(gl::STATIC_DRAW, &vertices[..], &indices[..]) };
        return Tile {
            kind: TileType::Empty,
            row, col,
            geometry
        }
    }
    
    pub fn set_type(&mut self, kind: TileType) {
        self.kind = kind;
    }

    //Will need textures and shaders set up already.
    pub fn draw_skin(&self, textures: &TileTextures) {
        unsafe {
            let texture = match &self.kind {
                TileType::Factory(factory_color) => {
                    match factory_color {
                        Factory::Red => &textures.red_factory,
                        Factory::Blue => &textures.blue_factory,
                        Factory::Green => &textures.green_factory,
                    }
                },
                TileType::Pipe(_) => &textures.pipe_texture,
                _ => return
            };

            texture.bind(0);
            self.geometry.draw(gl::TRIANGLE_STRIP);
            Texture::unbind(0);
        }
    }

    pub fn draw_stencil(&self, stencils: &TileStencils) {
        unsafe {
            let texture = match &self.kind {
                TileType::Factory(_) => return,
                TileType::Empty => return,
                TileType::Pipe(_) => &stencils.pipe_stencil
            };

            texture.bind(0);
            self.geometry.draw(gl::TRIANGLE_STRIP);
            Texture::unbind(0);
        }
    }
}

pub struct TileTextures {
    pub red_factory: Texture,
    pub blue_factory: Texture,
    pub green_factory: Texture,
    pub pipe_texture: Texture,
    pub empty_texture: Texture
    //more feilds to come
}

pub struct TileStencils {
    pub pipe_stencil: Texture,
}