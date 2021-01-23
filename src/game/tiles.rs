use crate::rendering::geometry::{Geometry, TexPoint};
use crate::rendering::texture::{Texture};
use crate::rendering::shader::Shader;
use std::ffi::CString;
use std::rc::Rc;

pub enum TileType {
    Factory(Color),
    Empty,
    Pipe(Orientation)
}

// pub enum Factory {

// }

// pub struct Pipe {
//     orientation: Orientaion
// }

pub enum Orientation {
    Vertical,
    Horizontal,
}

pub enum Color {
    Red,
    Green, 
    Blue
}

pub struct Tile {
    pub kind: TileType,
    pub row: i32,
    pub col: i32,
    pub geometry: Geometry<TexPoint>,
    pub texture: Rc<Texture>,
    pub stencil: Option< Rc<Texture> >
}

impl Tile {
    pub fn new(row: i32, col: i32, kind: TileType, texture: Rc<Texture>, stencil: Option< Rc<Texture> >) -> Tile {
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
            kind,
            row, col,
            geometry,
            texture,
            stencil
        }
    }

    pub fn new_empty(board: &Board, row: i32, col: i32) -> Tile {
        return Tile::new(row, col, TileType::Empty, board.textures.empty_texture.clone(), None);
    }

    pub fn new_factory(board: &Board, row: i32, col: i32, color: Color) -> Tile {
        let texture = match color {
            Color::Red => board.textures.red_factory.clone(),
            Color::Blue => board.textures.blue_factory.clone(),
            Color::Green => board.textures.green_factory.clone(),
        };

        return Tile::new(row, col, TileType::Factory(color), texture, None);
    }

    pub fn new_pipe(board: &Board, row: i32, col: i32, orientation: Orientation) -> Tile {
        return Tile::new(
            row, col, 
            TileType::Pipe(orientation), 
            board.textures.pipe_texture.clone(), 
            Some(board.stencils.pipe_stencil.clone())
        );
    }

    //Will need textures and shaders set up already.
    pub fn draw_texture(&self) {
        unsafe {
            self.texture.bind(0);
            self.geometry.draw(gl::TRIANGLE_STRIP);
            Texture::unbind(0);
        }
    }

    pub fn draw_stencil(&self) {
        if let Some(stencil) = &self.stencil {
            unsafe {
                stencil.bind(0);
                self.geometry.draw(gl::TRIANGLE_STRIP);
                Texture::unbind(0);
            }
        }
    }

}

pub struct TileTextures {
    pub red_factory: Rc<Texture>,
    pub blue_factory: Rc<Texture>,
    pub green_factory: Rc<Texture>,
    pub pipe_texture: Rc<Texture>,
    pub empty_texture: Rc<Texture>
    //more feilds to come?
}

pub struct TileStencils {
    pub pipe_stencil: Rc<Texture>,
}

pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub textures: TileTextures,
    pub stencils: TileStencils,
    pub rows: u32, 
    pub cols: u32
}

impl Board {
    pub fn new(rows: u32, cols: u32, textures: TileTextures, stencils: TileStencils) -> Board{

        let tiles: Vec<Vec<Tile>> = Vec::new();
        let mut board = Board {
            tiles, textures, stencils, rows, cols
        };

        //the way I do this is very crude, but its a one time thing, and i dont 
        //want to make the rest of the api gross.
        let mut tiles_copy = Vec::with_capacity(rows as  usize); 
        for r in 0..rows {
            tiles_copy.push(Vec::with_capacity(cols as usize));
            for c in 0..cols {
                tiles_copy[r as usize].push(Tile::new_empty(&board, r as i32, c as i32));
            }
        }
        board.tiles = tiles_copy;

        return board;
    }

    pub fn set_tile(&mut self, row: i32, col: i32, kind: TileType) {
        let tile = match kind {
            TileType::Factory(color) => Tile::new_factory(self, row, col, color),
            TileType::Pipe(orientation) => Tile::new_pipe(self, row, col, orientation),
            TileType::Empty => Tile::new_empty(self, row, col)
        };

        self.tiles[row as usize][col as usize] = tile;
    }
}