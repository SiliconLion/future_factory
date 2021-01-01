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
    // print_errors(49);

    // //create/bind the vertex array object.
    // //VAO's store the calls to glVertexAttribPointer and glEnableVertexAttribArray so
    // //you dont have to rebind all the attributes each time 
    let mut VAO: u32 = 0;
// 
    gl::GenVertexArrays(1, &mut VAO);
    // print_errors(56);
    gl::BindVertexArray(VAO);
    // print_errors(57);


    let vertices = vec![
        ThreePoint{ data: [-0.5, -0.5, 0.0,] },
        ThreePoint{ data: [ 0.5, -0.5, 0.0,] },
        ThreePoint{ data: [ 0.0,  0.5, 0.0,] },
    ];


    println!("size of verticeis: {}", std::mem::size_of_val(&vertices));




    let indices = vec![
        0, 1, 2,   // first triangle
    ];

    // let geom : Geometry<ThreePoint> = Geometry::from_verts_and_indices(
    //     gl::STATIC_DRAW,
    //     &ThreePoint::from_float_buffer(&vertices[..])[..],
    //     &indices[..]
    // );

    print_errors(70);
    
    let mut VBO: u32 = 0;

    gl::GenBuffers(1, &mut VBO);
    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER, 9 * float_size as isize, vertices.as_ptr() as _, gl::STATIC_DRAW );

    // print_errors(77);
        
    let mut EBO: u32 = 0;
    gl::GenBuffers(1, &mut EBO);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * float_size ) as isize, indices.as_ptr() as _, gl::STATIC_DRAW); 


    // print_errors(82);


    //     //setting up the vertex layout
    
    // //0 tells it that its the first vertex attribute in the vertex.
    // //3 peices of data per this attribute.
    // //that data is floats.
    // //we dont care about normalization. 
    // //>"The fifth argument is known as the stride and tells us the space between consecutive 
    // //>vertex attributes. Since the next set of position data is located exactly 
    // //>n times the size of a float, we specify that value as the stride. 
    // //>Note that since we know that the array is tightly packed (there is no 
    // //>space between the next vertex attribute value) we could've also specified 
    // //>the stride as 0 to let OpenGL determine the stride (this only works when values 
    // //>are tightly packed). 
    // //where the position data beins in the buffer
    // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * float_size) as i32, 0 as *const c_void);
    // //tell big global state how the 0th vertex attribute is layed out
    // gl::EnableVertexAttribArray(0); 

    
    ThreePoint::set_vertex_attributes();
    ThreePoint::enable_vertex_attributes();

    let geom = Geometry {
        VAO, VBO, EBO, vertices, indices, usage: gl::STATIC_DRAW
    };

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

 
        // gl::BindVertexArray(VAO);
        // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        // gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, 0 as *const c_void);

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

