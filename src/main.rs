#![allow(non_snake_case)]

extern crate glfw;

use std::ffi::CString;
use std::mem::size_of;
use std::ffi::c_void;
use std::rc::Rc;


use glfw::{Action, Context, Key};
use gl::*;

use rand::prelude::*;

use cgmath::prelude::*;
use cgmath::{Matrix4, Rad, Vector3};

pub mod game;
use game::tiles::*;

pub mod rendering;
use rendering::*;

use geometry::*;
use shader::*;
use texture::Texture;
use primitives::*;
use pipeline::*;

pub mod utilities;
use utilities::*;

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, board: &mut Board, row: &mut usize, col: &mut usize) {
    match event {
        glfw::WindowEvent::Key(key, _, Action::Press, _) => {
            match key {
                Key::Escape => window.set_should_close(true),
                Key::Right => {*col += 1;},
                Key::Left => {*col -= 1},
                Key::Up => {*row -= 1;},
                Key::Down => {*row += 1;},
                Key::Space => {

                    let mut rng = rand::thread_rng();
                    let range = rng.gen_range(0..3);
                    match range {
                        0 => board.set_tile(*row as i32, *col as i32, TileType::Factory(Color::Red)),
                        1 => board.set_tile(*row as i32, *col as i32, TileType::Factory(Color::Blue)),
                        2 => board.set_tile(*row as i32, *col as i32, TileType::Factory(Color::Green)),
                        _ => {}
                    }
                },
                Key::F => {
                    board.set_tile(*row as i32, *col as i32, TileType::Pipe(Orientation::Horizontal))
                },
                _ => {}
            }
            
        },
        glfw::WindowEvent::FramebufferSize(width, height)  => {
            unsafe {
                gl::Viewport(0,0, width, height);
            }
        },
        glfw::WindowEvent::MouseButton(_, action, _) => {
            // if action == glfw::Action::Press {
            //     println!("here!");
            //     let mouse_pos = get_normalized_cursor_pos(&window);
            //     let (row, col) = get_tile_coords(mouse_pos.0, mouse_pos.1);

            //     let mut tile = &mut tiles[row][col];

            //     let mut rng = rand::thread_rng();
            //     let range = rng.gen_range(0..3);
            //     match range {
            //         0 => tile.set_type(TileType::Factory(Factory::Red)),
            //         1 => tile.set_type(TileType::Factory(Factory::Green)),
            //         2 => tile.set_type(TileType::Factory(Factory::Blue)),
            //         _ => {}
            //     }
            // }

        }
        _ => {}
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // window.set_key_polling(true);
    // window.set_framebuffer_size_polling(true);
    window.set_all_polling(true);
    window.make_current();

    // the supplied function must be of the type:
    // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
    // `window` is a glfw::Window
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let mut selected_r = 0;
    let mut selected_c = 0;

    unsafe{
    let textures = TileTextures {
        red_factory: Rc::new(Texture::new_from_file("src/textures/red_factory.png")),
        blue_factory: Rc::new(Texture::new_from_file("src/textures/blue_factory.png")),
        green_factory: Rc::new(Texture::new_from_file("src/textures/green_factory.png")),
        pipe_texture: Rc::new(Texture::new_from_file("src/textures/pipe.png")),
        empty_texture:Rc::new(Texture::new_blank())
    };

    let stencils = TileStencils {
        pipe_stencil: Rc::new(Texture::new_from_file("src/textures/pipe_stencil.png"))
    };

    let mut board = Board::new(20, 20, textures, stencils);

    let background = TexturedRect::new(
        Texture::new_from_file("src/textures/shiny_green.jpg"),
        4.0, 4.0,
        -2.0, 2.0, 0.0
    );


    print_errors(92);
    let tile_program = Shader::new( &vec![
        shader::ShaderSource::from_file(
           "src/shader_src/tile.vert", 
            gl::VERTEX_SHADER
        ),
        shader::ShaderSource::from_file(
            "src/shader_src/tile.frag",
            gl::FRAGMENT_SHADER
        )
    ]);

    print_errors(103);


    let sampler_loc = gl::GetUniformLocation(tile_program.id, CString::new("ourTexture").unwrap().as_ptr());
    let scale_loc = gl::GetUniformLocation(tile_program.id, CString::new("scale").unwrap().as_ptr());
    let translation_loc = gl::GetUniformLocation(tile_program.id, CString::new("translation").unwrap().as_ptr());
    let scale = Matrix4::from_scale(2.0 / board.rows as f32);
    
    let mut counter = 0.0;

    // gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut board, &mut selected_r, &mut selected_c);
        }
        let mouse_pos = get_normalized_cursor_pos(&window);


        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        clear_stencil();

        tile_program.bind();
        print_errors(129);


        gl::Uniform1i(sampler_loc, 0); //tile.draw_skin binds the texture to 0
        gl::UniformMatrix4fv(scale_loc, 1, gl::FALSE, scale.as_ptr());
        // gl::Uniform2f(translation_loc, -1.0 - 2.0 / cols as f32, -1.0 - 2.0 / rows as f32);
        // gl::Uniform2f(translation_loc, -1.0 * cols as f32 / 2.0, -1.0 * (rows + 1) as f32 / 2.0);
        // gl::Uniform2f(translation_loc, mouse_pos.0, mouse_pos.1);
        gl::Uniform2f(translation_loc, -10.0, -9.0);
        for r in 0..board.rows {
            for c in 0..board.cols {
                let tile = &board.tiles[r as usize][c as usize];
                tile.draw_texture();
            }
        }

        start_stencil_writing();
        for r in 0..board.rows {
            for c in 0..board.cols {
                let tile = &board.tiles[r as usize][c as usize];
                tile.draw_stencil();
            }
        }
        stop_stencil_writing();

        draw_where_stencil();
        background.draw(sampler_loc);
        disable_stencil();




        print_errors(233);


        // check and call events and swap the buffers
        window.swap_buffers();

        if counter >= 2.0 {
            counter = 0.0;
        } else {
            counter += 0.001;
        }

    }

    }

}

fn get_normalized_cursor_pos(window: &glfw::Window) -> (f32, f32) {
    let (sx, sy) = window.get_size();
    let screen_dims = (sx as f32, sy as f32);
    let (mx, my) = window.get_cursor_pos();
    let mouse_pos = (mx as f32, my as f32);

    return (
        (mouse_pos.0 / screen_dims.0) * 2.0 - 1.0,
        1.0 - (mouse_pos.1 / screen_dims.1) * 2.0 
    );
}

fn get_tile_coords(screen_x: f32, screen_y: f32) -> (usize, usize){
    let tile_width = 1.0 / 20.0;
    let tile_height = 1.0 / 20.0;
    let row = ( (screen_y + 1.0) / tile_height ).floor() as usize; 
    let col = ( (screen_x + 1.0) / tile_width ).floor() as usize; 

    return (row, col);
}


unsafe fn print_errors(line: u32) {
    let mut err = gl::GetError();
    if err != gl::NO_ERROR {println!("the errors at line {} are:", line);}
    while err != gl::NO_ERROR 
    {
        println!("error: {:#X}", err);
        err = gl::GetError();
    }
}

