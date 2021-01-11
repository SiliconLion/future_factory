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

pub mod shader; 
pub mod geometry; 
pub mod utilities;
pub mod texture; 
pub mod primitives;

use geometry::*;
use utilities::*;
use texture::Texture;
use primitives::*;

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

    let width_count: usize = 400;
    let height_count: usize = 400;
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


    let background = Geometry::from_verts_and_indices(
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

    let background_program = shader::Shader::new( &vec![vertex_source, frag_source]);
    let transformation_loc = gl::GetUniformLocation(
        background_program.id, 
        CString::new("transformation").unwrap().as_ptr()
    );


    print_errors(127);

    let rect_verts = [
        ThreePoint {data:[-0.25, 0.25, 0.0]},
        ThreePoint {data:[-0.25, -0.25, 0.0]},
        ThreePoint {data:[0.25, 0.25, 0.0]},
        ThreePoint {data:[0.25, -0.25, 0.0]}
    ];
    //triangle strip indexing
    let rect_indices = [0,1,2,3];

    let rect = Geometry::from_verts_and_indices(
        gl::STATIC_DRAW,
        &rect_verts[..],
        &rect_indices[..]
    );

    let rect_vert_shader = shader::ShaderSource::from_file(
        "src/shader_src/simple.vert",
        gl::VERTEX_SHADER
    );

    let rect_frag_shader = shader::ShaderSource::from_file(
        "src/shader_src/simple.frag",
        gl::FRAGMENT_SHADER
    );

    let rect_program = shader::Shader::new( &vec![rect_vert_shader, rect_frag_shader]);
    let translation_loc = gl::GetUniformLocation(rect_program.id, CString::new("translation").unwrap().as_ptr());

    let gold_texture = Texture::new("src/textures/soft_gold.jpg");
    let tex_rect = TexturedRect::new(gold_texture, 0.3, 0.2, 0.3, 0.2, 0.0);
    let tex_rect_vert_shader = shader::ShaderSource::from_file(
        "src/shader_src/texture.vert",
        gl::VERTEX_SHADER
    );
    let tex_rect_frag_shader = shader::ShaderSource::from_file(
        "src/shader_src/texture.frag",
        gl::FRAGMENT_SHADER
    );
    let tex_rect_program = shader::Shader::new( &vec![tex_rect_vert_shader, tex_rect_frag_shader]);
    tex_rect_program.bind();
    print_errors(171);
    
    let mut counter: f32 = 0.0;
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        let mouse_pos = get_normalized_cursor_pos(&window);



        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::StencilMask(0xFF); //enable writing to the mask (including gl::Clear)
        gl::Clear(gl::COLOR_BUFFER_BIT|gl::STENCIL_BUFFER_BIT);
        gl::StencilMask(0x00); //disable writing to the mask just as cleanup


        gl::Enable(gl::STENCIL_TEST); //enable stencil test 

//Draw to the mask our stencil

        
        gl::StencilOp(gl::KEEP, gl::REPLACE, gl::REPLACE); //write to the mask, ignoring depth
        gl::StencilFunc(gl::ALWAYS, 1, 0xFF); //anything that passes, write 1 (i think)
        gl::StencilMask(0xFF); //this (weirdly) allows the mask to be written to
        
        gl::ColorMask(gl::FALSE,gl::FALSE,gl::FALSE,gl::FALSE); //dont write to color
        gl::DepthMask(gl::FALSE); //dont write to depth

        rect_program.bind();
        gl::Uniform2f(translation_loc, mouse_pos.0, mouse_pos.1);
        rect.draw(gl::TRIANGLE_STRIP);
        rect_program.unbind();
        
        gl::StencilMask(0x00); //disable writing to the mask
        gl::ColorMask(gl::TRUE,gl::TRUE,gl::TRUE,gl::TRUE); //re-enable writing to color
        gl::DepthMask(gl::TRUE); //re-enable writing to depth

//Draw what the stencil is applied to  (the background)

        let transformation = Matrix4::from_scale(4.0) * Matrix4::from_angle_z(Rad(counter));

        //If the mask value at that fragment is gl::Equal to 1 after its && with 0xFF,
        //draw the fragment
        gl::StencilFunc(gl::EQUAL, 1, 0xFF); 
        background_program.bind();
        gl::UniformMatrix4fv(transformation_loc, 1, gl::FALSE, transformation.as_ptr());
        background.draw(gl::TRIANGLES);
        background_program.unbind();
        gl::Disable(gl::STENCIL_TEST);

//draw the textured rect
        print_errors(224);
        tex_rect_program.bind();
        print_errors(226);
        let sampler_loc = gl::GetUniformLocation(tex_rect_program.id, CString::new("ourTexture").unwrap().as_ptr());
        print_errors(228);
        tex_rect.draw(sampler_loc);
        print_errors(230);
        tex_rect_program.unbind();

        print_errors(233);





        // check and call events and swap the buffers
        window.swap_buffers();

        counter += 0.01;
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

