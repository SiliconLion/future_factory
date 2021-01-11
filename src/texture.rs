use image::{open, imageops::flip_vertical_in};
use gl::*;
use std::ffi::c_void;

//all textures will use alpha channel, even if they are really RGB not RGBA.
pub struct Texture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    //by default, texture is created with wraping set to clamp to border, and 
    //filtering set to linear.
    pub unsafe fn new<S: Into<String>>(path: S) -> Texture {
        let image = open(path.into()).unwrap().into_rgba8();
        let mut flipped_image = image.clone(); //kinda nasty but had to move fast and it worked
        flip_vertical_in(&image, &mut flipped_image);
        
        let mut texture = Texture {id: 0, width: flipped_image.width(), height: flipped_image.height()};
        texture.set_wrapping(gl::CLAMP_TO_BORDER, gl::CLAMP_TO_BORDER);
        texture.set_filtering(gl::LINEAR, gl::LINEAR);

        texture.bind(0);

        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGBA as i32, 
            texture.width as i32, 
            texture.height as i32, 
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            flipped_image.into_raw().as_ptr() as *const c_void //the .as_ptr cast is suspect.
        ); 

        gl::GenerateMipmap(gl::TEXTURE_2D);

        return texture;
    }

    //slot is which texture slot to bind to. 
    pub unsafe fn bind(&self, slot: u32) {
        //set which slot is active
        gl::ActiveTexture(gl::TEXTURE0 + slot);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub unsafe fn unbind(slot: u32) {
        gl::ActiveTexture(gl::TEXTURE0 + slot);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    //sets the vertical and horizontal wrapping settings
    //technically doesnt need to be mut, but really its a mutating operation its just not 
    //changing data.
    pub unsafe fn set_wrapping(&mut self, s_wrap: gl::types::GLenum, t_wrap: gl::types::GLenum) {
        self.bind(0);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, s_wrap as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, t_wrap as gl::types::GLint);
        Texture::unbind(0);
    }

    //sets the way sampling is interpolated 
    pub unsafe fn set_filtering(&mut self, min_filter: gl::types::GLenum, max_filter: gl::types::GLenum) {
        self.bind(0);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, max_filter as gl::types::GLint);
        Texture::unbind(0);
    } 
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &(self.id) ); }
    }
}