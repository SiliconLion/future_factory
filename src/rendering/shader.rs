extern crate glfw;
use std::mem::size_of;
use std::ffi::c_void;
use std::ffi::CString;

use std::ptr::null;
use std::ptr::null_mut;

use std::fs::read_to_string;



pub struct ShaderSource {
    pub source: CString,
    pub shader_type: gl::types::GLenum
}

impl ShaderSource {
    pub fn new(source: String, shader_type: gl::types::GLenum) -> ShaderSource {
        ShaderSource {
            source: CString::new(source).unwrap(),
            shader_type
        }
    }

    pub fn from_file(path: &str , shader_type: gl::types::GLenum ) -> ShaderSource {
        ShaderSource::new(
            read_to_string(path).expect("Cannot create ShaderSource: invalid file path."), 
            shader_type)
    }
}

pub struct Shader {
    pub id: u32
}

impl Shader {
    pub unsafe fn new(sources: &Vec<ShaderSource>) -> Shader {

        let shaders: Vec<u32> = 
            sources.iter().map( |shad_source| -> u32 {
                let shader_id = gl::CreateShader(shad_source.shader_type);
                gl::ShaderSource(
                    shader_id,
                    1,
                    &shad_source.source.as_ptr() as _,
                    null()
                );
                return shader_id
            })
            .collect();

        shaders.iter().for_each( | &id | {
            gl::CompileShader(id);

            //for checking error statuses
            //should be some form of bool but i cant be bothered 
            //set to 1 cuz assume its true
            let mut success: i32 = 1;
            let mut info_log = String::with_capacity(512);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(
                    id, 
                    512, 
                    null_mut::<_>(), 
                    info_log.as_mut_ptr() as *mut i8
                );
                print!("ERROR: Shader compilation failed. Shader type: {},\n{}\n",id, info_log);
            }
        });
        
        let shader = Shader { id: gl::CreateProgram() };

        shaders.iter().for_each(|&id| gl::AttachShader(shader.id, id) );
        gl::LinkProgram(shader.id);
        
        shaders.iter().for_each( |&id| gl::DeleteShader(id) );

        gl::ValidateProgram(shader.id);
        let mut status: i32 = 0;
        gl::GetProgramiv(shader.id, gl::VALIDATE_STATUS, &mut status);
        if status as u8 == gl::FALSE {
            println!("program not valid");
        }

        return shader;
    }

    pub unsafe fn bind(&self) {
        gl::UseProgram(self.id);
    }
    //same thing as bind_zero but can be semantically clearer whats going on.
    pub unsafe fn unbind(&self) {
        gl::UseProgram(0);
    }
    pub unsafe fn bind_zero() {
        gl::UseProgram(0);
    }
}


// gl::ShaderSource(
//     frag_shader, 
//     1, 
//     &frag_source.as_ptr() as _, 
//     null()
// );

// print_errors(144);

// //for checking error statuses
// //should be some form of bool but i cant be bothered 
// //set to 1 cuz assume its true
// let mut success: i32 = 1;
// let mut info_log = String::with_capacity(512);

// gl::CompileShader(vert_shader);
// gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut success);

// if success == 0 {
//     gl::GetShaderInfoLog(
//         vert_shader, 
//         512, 
//         null_mut::<_>(), 
//         info_log.as_mut_ptr() as *mut i8
//     );
//     print!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}\n", info_log);
// }

//     //a shader program is a bunch of different shaders linked together
//     let mut shaderProgram: u32 = gl::CreateProgram();
//     //we tell the shader program what shaders to use, then link them together
//     gl::AttachShader(shaderProgram, vert_shader);
//     gl::AttachShader(shaderProgram, frag_shader);
//     gl::LinkProgram(shaderProgram);
//     //tell the big global state, this is the program we will be using to render
//     gl::UseProgram(shaderProgram);

//     //we're not going to reuse these shaders for anything else or link
//     //them to other programs, so delete them. dont want to leak memory. 
//     gl::DeleteShader(vert_shader);
//     gl::DeleteShader(frag_shader);