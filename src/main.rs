#![allow(non_snake_case)]

extern crate glfw;

use std::ffi::CString;
use std::mem::size_of;
use std::ffi::c_void;

use glfw::{Action, Context, Key};
use gl::*;

use rand::prelude::*;

use cgmath::prelude::*;
use cgmath::Matrix4;

pub mod shader; 
pub mod geometry; 
pub mod utilities;

use geometry::*;
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
    
    let mut rng = rand::thread_rng();

    let width_count: usize = 10;
    let height_count: usize = 10;
    let mut vertices = Vec::with_capacity( (width_count * height_count) as usize );
    for i in 0..width_count +1{
        for j in 0..height_count +1{
            vertices.push(
                PointWithNorm {
                    //x and y range from -1 to 1. 
                    //x starts at -1 and increases, and y starts at 1 and decreases
                    location : [ 
                        (2.0 / width_count as f32 ) * i as f32 - 1.0,
                         1.0 - ((2.0 / height_count as f32 ) * j as f32) , 
                         0.0
                    ],
                    norm: [rng.gen(), rng.gen(), rng.gen()]
                }
            );
        }
    }

    let mut indices = Vec::with_capacity( (width_count * height_count) as usize );
    for row in 0..height_count {
        for col in 0..width_count {
            //first triangle in rect
            indices.push(coords_to_index(row,     col,      width_count +1));
            indices.push(coords_to_index(row + 1, col,      width_count+1));
            indices.push(coords_to_index(row,     col + 1 , width_count+1));

            //second tri in rect
            indices.push(coords_to_index(row,     col + 1,  width_count+1));
            indices.push(coords_to_index(row + 1, col,      width_count+1));
            indices.push(coords_to_index(row + 1, col + 1 , width_count+1));
        }
    }
    let indices: Vec<u32> = indices.iter().map( |&e| e as u32).collect();


    let geom = Geometry::from_verts_and_indices(
        gl::STATIC_DRAW,
        &vertices[..],
        &indices[..]
    ); 

    print_errors(104);



    let vertex_source = shader::ShaderSource::from_file(
            "src/shader_src/vertex.vert",
            gl::VERTEX_SHADER
        );
    let frag_source = shader::ShaderSource::from_file(
        "src/shader_src/fragment.frag",
        gl::FRAGMENT_SHADER
    );

    let program = shader::Shader::new( &vec![vertex_source, frag_source]);
    print_errors(124);

    program.bind();
    print_errors(127);

    
    let scale_loc = gl::GetUniformLocation(program.id, CString::new("scale").unwrap().as_ptr() );
    let mut counter = 1.0;

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        //sets clear color
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        //clears the color buffer (as opposed to depth buffer or stencil buffer etc)
        gl::Clear(gl::COLOR_BUFFER_BIT);

        let scale = Matrix4::from_scale(counter);
        gl::UniformMatrix4fv(scale_loc, 1, gl::FALSE, scale.as_ptr());

        print_errors(143);
        gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
        gl::LineWidth(1.0);
        print_errors(146);
        geom.draw(gl::TRIANGLES);

        print_errors(149);



        // check and call events and swap the buffers
        window.swap_buffers();

        counter -= 0.001;
    }

    }

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

