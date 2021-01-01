use gl::*;
use std::convert::TryInto;
use std::mem::size_of;
use std::ffi::c_void;

//the stuct that impliments this trait should also "#[repr(packed)]". 
pub unsafe trait Vertex: Sized + Clone {
    unsafe fn set_vertex_attributes();
    unsafe fn enable_vertex_attributes();
    fn stride() -> usize;
    //creates a single vertex from bytes
    unsafe fn from_bytes(bytes: &[u8]) -> Self;
    unsafe fn from_byte_buffer(buffer: &[u8]) -> Vec<Self> {
        if buffer.len() % Self::stride() != 0 {
            println!("Error! cannot evenly create verticies out of buffer");
        }

        let vertex_count = buffer.len() / Self::stride();
        let mut vertices = Vec::with_capacity(vertex_count);

        for i in 0..vertex_count {
            vertices.push(Self::from_bytes( &buffer[i*Self::stride()..(i+1)*Self::stride()] ));
        }

        return vertices;
    }
}


#[derive(Clone, Copy)]
#[repr(packed)]
pub struct TwoPoint {
    data: [f32; 2]
}

unsafe impl Vertex for TwoPoint {
    fn stride() -> usize {
        return 2 * std::mem::size_of::<f32>();
    }
    unsafe fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != Self::stride() {
            panic!("TwoDPoint did not wholly Divide bytes");
        }

        let bytes: [u8; 8] = bytes.try_into().unwrap();
        let data = unsafe { 
            std::mem::transmute::<[u8; 8], [f32; 2]>(bytes)
        };

        Self{data}
    }

    unsafe fn set_vertex_attributes() {
        gl::VertexAttribPointer(
            0, 
            2, 
            gl::FLOAT, 
            gl::FALSE, 
            (2 * size_of::<f32>()) as i32, 
            0 as *const c_void
        );
    }
    unsafe fn enable_vertex_attributes() {
        gl::EnableVertexAttribArray(0); 
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct ThreePoint {
    pub data: [f32; 3]
}

unsafe impl Vertex for ThreePoint {
    fn stride() -> usize {
        return 3 * std::mem::size_of::<f32>();
    }
    unsafe fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != Self::stride() {
            panic!("ThreePoint did not wholly Divide bytes");
        }

        let bytes: [u8; 12] = bytes.try_into().unwrap();
        let data = unsafe { 
            std::mem::transmute::<[u8; 12], [f32; 3]>(bytes)
        };

        Self{data}
    }

    unsafe fn set_vertex_attributes() {
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (3 * size_of::<f32>()) as i32, 
            0 as *const c_void
        );
    }
    unsafe fn enable_vertex_attributes() {
        gl::EnableVertexAttribArray(0); 
    }
}

impl ThreePoint {
    pub fn from_float_buffer(buffer: &[f32]) -> Vec<ThreePoint> {
        if buffer.len() % 3 != 0 {
            panic!();
        }

        let mut points = Vec::with_capacity(buffer.len() / 3);
        for i in 0..(buffer.len() / 3) {
            points.push( ThreePoint {
                data: [buffer[i], buffer[i+1], buffer[i+2]]
            });
        }

        return points;
    }
}

pub struct Geometry<V: Vertex> {
    pub VAO: u32,
    pub VBO: u32,
    pub EBO: u32,
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
    //what mode the vertex buffer associated with VBO is. 
    //eg, GL_STATIC_DRAW, GL_DYNAMIC_DRAW
    pub usage: types::GLenum
}

impl<V: Vertex> Geometry<V> {
    pub unsafe fn new(usage: types::GLenum) -> Geometry<V> {
        let vertices = Vec::new();
        let indices = Vec::new();

        let mut VAO: u32 = 0;
        gl::GenVertexArrays(1, &mut VAO);

        let mut VBO: u32 = 0;
        gl::GenBuffers(1, &mut VBO);
        // gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

        let mut EBO = 0;
        gl::GenBuffers(1, &mut EBO);


        //set up the VAO calls 
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        //this will set up the layout of the vertex.
        V::set_vertex_attributes();
        V::enable_vertex_attributes();

        //unbind
        gl::BindVertexArray(0);

        Geometry {
            VAO,
            VBO,
            EBO,
            vertices,
            indices,
            usage
        }
    }

    pub unsafe fn from_verts_and_indices(usage: types::GLenum, vertices: &[V], indices: &[u32]) -> Geometry<V> {
        let mut geom = Geometry::new(usage);
        geom.vertices = vertices.clone().into();
        geom.indices = indices.clone().into();

        geom.update_verts();
        geom.update_indicies();

        return geom;
    }


    // pub fn replace_verticies(&mut self, buffer: &[V]) {
    //     unimplemented!();
    // }

    // pub fn add_vertex(&mut self, vert: V) {
    //     unimplemented!();
    // }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.VAO);
    }

    //doesnt really need to take &self, but calling self.unbind() is way nicer than
    //Geometry<V>::unbind();. Instead I provide that as Geometry<V>::bind_empty();
    //Same thing.
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    //binds the VAO to 0. 
    pub fn bind_empty() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    //call when you modify the vertices and want to send the changes to the gpu. 
    unsafe fn update_verts(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
        gl::BufferData(gl::ARRAY_BUFFER, 
            (self.vertices.len() * V::stride()) as isize, 
            self.vertices.as_ptr() as _, 
            self.usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    unsafe fn update_indicies(&self) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
            (self.indices.len() * size_of::<u32>()) as isize, 
            self.indices.as_ptr() as _, 
            self.usage
        );
        gl::BindBuffer(ELEMENT_ARRAY_BUFFER, 0);
    }

    //simple drawing. not going to be the only way to draw
    pub unsafe fn draw(&self) {
        self.bind();
        gl::DrawElements(
            gl::TRIANGLES, 
            self.indices.len() as i32, 
            gl::UNSIGNED_INT, 
            0 as *const c_void
        );
        self.unbind();
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
