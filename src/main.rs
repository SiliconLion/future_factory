extern crate glfw;

use std::ffi::CString;
use std::mem::size_of;
use std::ffi::c_void;

use glfw::{Action, Context, Key};
use gl::*;


pub mod shader; 
pub mod geometry; 

use geometry::*;

#[allow(non_snake_case)]

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
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
    window.make_current();

    // the supplied function must be of the type:
    // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
    // `window` is a glfw::Window
    gl::load_with(|s| window.get_proc_address(s) as *const _);




    let float_size = std::mem::size_of::<f32>();

    unsafe{



    let vertices = vec![
        ThreePoint{ data: [-0.5, -0.5, 0.0,] },
        ThreePoint{ data: [ 0.5, -0.5, 0.0,] },
        ThreePoint{ data: [ 0.0,  0.5, 0.0,] },
    ];


    println!("size of verticeis: {}", std::mem::size_of_val(&vertices));




    let indices = vec![
        0, 1, 2,   // first triangle
    ];



    let geom = Geometry::from_verts_and_indices(
        gl::STATIC_DRAW,
        &vertices[..],
        &indices[..]
    ); 

    print_errors(104);

    let vertex_source = shader::ShaderSource::from_file(
            "/home/david/Desktop/programing/open-gl/glloadtest/src/shader_src/vertex.vert",
            gl::VERTEX_SHADER
        );
    let frag_source = shader::ShaderSource::from_file(
        "/home/david/Desktop/programing/open-gl/glloadtest/src/shader_src/fragment.frag",
        gl::FRAGMENT_SHADER
    );

    let program = shader::Shader::new( &vec![vertex_source, frag_source]);
    print_errors(124);

    program.bind();
    print_errors(127);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        //sets clear color
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        //clears the color buffer (as opposed to depth buffer or stencil buffer etc)
        gl::Clear(gl::COLOR_BUFFER_BIT);

 
        geom.draw();

        print_errors(147);

        // geom.draw();

        print_errors(151);

        // check and call events and swap the buffers
        window.swap_buffers();
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

