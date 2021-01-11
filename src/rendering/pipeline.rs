use gl::*;

pub unsafe fn clear_stencil() {
    gl::StencilMask(0xFF); //enable writing to the mask (including gl::Clear)
    gl::Clear(gl::STENCIL_BUFFER_BIT);
    gl::StencilMask(0x00); //disable writing to the mask just as cleanup
}

//only allows writing to stencil buffer, not color or depth
pub unsafe fn start_stencil_writing() {
    gl::Enable(gl::STENCIL_TEST);
    gl::StencilOp(gl::KEEP, gl::REPLACE, gl::REPLACE); //write to the mask, ignoring depth
    gl::StencilFunc(gl::ALWAYS, 1, 0xFF); //anything that passes, write 1 (i think)
    gl::StencilMask(0xFF); //this (weirdly) allows the mask to be written to
    
    gl::ColorMask(gl::FALSE,gl::FALSE,gl::FALSE,gl::FALSE); //dont write to color
    gl::DepthMask(gl::FALSE); //dont write to depth
}

pub unsafe fn stop_stencil_writing() {
    gl::StencilMask(0x00); //disable writing to the mask
    gl::ColorMask(gl::TRUE,gl::TRUE,gl::TRUE,gl::TRUE); //re-enable writing to color
    gl::DepthMask(gl::TRUE); //re-enable writing to depth
}

pub unsafe fn draw_where_stencil() {
    //If the mask value at that fragment is gl::Equal to 1 after its && with 0xFF,
    //draw the fragment
    gl::StencilFunc(gl::EQUAL, 1, 0xFF); 
}

pub unsafe fn disable_stencil() {
    gl::Disable(gl::STENCIL_TEST);
}