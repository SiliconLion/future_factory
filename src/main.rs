#![allow(non_snake_case)]

extern crate glfw;

use std::ffi::CString;
use std::mem::size_of;
use std::ffi::c_void;

use glfw::{Action, Context, Key};
use gl::*;

use rand::prelude::*;

use cgmath::prelude::*;
use cgmath::{Matrix4, Rad};

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

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        glfw::WindowEvent::FramebufferSize(width, height)  => {
            unsafe {
                gl::Viewport(0,0, width, height);
            }
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

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    // the supplied function must be of the type:
    // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
    // `window` is a glfw::Window
    gl::load_with(|s| window.get_proc_address(s) as *const _);



    unsafe{

    let mut rows = 10;
    let mut cols = 10;
    let mut tiles: Vec<Vec<Tile>> = Vec::with_capacity(rows);
    let mut counter = 0;
    for r in 0..rows {
        tiles.push( Vec::with_capacity(cols));
        for c in 0..cols {
            tiles[r].push(Tile::new(r as i32, c as i32));
            if counter % 4 == 0 {
                tiles[r][c].set_type(TileType::Factory(Factory::Red));
            } else if counter %3 == 0 {
                tiles[r][c].set_type(TileType::Factory(Factory::Blue));
            } else if counter % 2 == 0 {
                tiles[r][c].set_type(TileType::Factory(Factory::Green));
            } else {
                tiles[r][c].set_type(TileType::Empty);
            }
            counter += 1;
        }
    }

    let mut tile = Tile::new(0, 0);
    tile.set_type(TileType::Factory(Factory::Red));
    
    let textures = TileTextures {
        red_factory: Texture::new_from_file("src/textures/red_factory.png"),
        blue_factory: Texture::new_from_file("src/textures/blue_factory.png"),
        green_factory: Texture::new_from_file("src/textures/green_factory.png"),
        empty_texture: Texture::new_blank()
    };
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
    let transform_loc = gl::GetUniformLocation(tile_program.id, CString::new("transform").unwrap().as_ptr());
    let transform = Matrix4::from_scale(0.1);


    // gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        let mouse_pos = get_normalized_cursor_pos(&window);



        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        clear_stencil();

        tile_program.bind();
        print_errors(129);
        gl::Uniform1i(sampler_loc, 0); //tile.draw_skin binds the texture to 0
        gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());
        print_errors(132);
        for r in 0..rows {
            for c in 0..cols {
                let tile = &tiles[r][c];
                tile.draw_skin(&textures);
            }
        }

       

        // red_factory.bind(0);
        // tile.geometry.draw(gl::TRIANGLE_STRIP);
        //  tile.draw_skin(&textures);

        print_errors(233);


        // check and call events and swap the buffers
        window.swap_buffers();


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


unsafe fn print_errors(line: u32) {
    let mut err = gl::GetError();
    if err != gl::NO_ERROR {println!("the errors at line {} are:", line);}
    while err != gl::NO_ERROR 
    {
        println!("error: {:#X}", err);
        err = gl::GetError();
    }
}

