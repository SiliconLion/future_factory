use crate::geometry::*;
use crate::texture::*;

pub struct TexturedRect {
    pub geom: Geometry<TexPoint>,
    pub texture: Texture,
    pub width: f32,
    pub height: f32,
    //x and y are from top left corner.
    pub x: f32, 
    pub y: f32,
    //all verticies share z coordinate
    pub z: f32
}

impl TexturedRect {
    pub unsafe fn new(texture: Texture, width: f32, height: f32, x: f32, y: f32, z: f32) -> TexturedRect {
        let vertices = [
            TexPoint { location: [x, y, z], tex_coord: [0.0, 1.0]}, // top left
            TexPoint { location: [x, y - height, z], tex_coord: [0.0, 0.0]}, //bottom left
            TexPoint { location: [x + width, y, z], tex_coord: [1.0, 1.0]}, // top right
            TexPoint { location: [x + width, y - height, z], tex_coord: [1.0, 0.0]} //bottom right
        ];

        let indices = [0, 1, 2, 3]; //triangle strip indices.

        return TexturedRect {
            geom:  Geometry::from_verts_and_indices(gl::STATIC_DRAW, &vertices[..], &indices[..]) ,
            texture,
            width, height, 
            x, y, z
        };
    }

    //sampler_loc is the location of the uniform sampler_2D in the shader. 
    //will bind the texture to slot 0 for drawing. 
    pub unsafe  fn draw(&self, sampler_loc: gl::types::GLint) {
        self.texture.bind(0);
        //sets the uniform located at sampler_loc to be the first texture slot
        gl::Uniform1i(sampler_loc, 0);
        self.geom.draw(gl::TRIANGLE_STRIP);
        Texture::unbind(0);
    }
}