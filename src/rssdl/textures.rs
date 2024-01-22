use sdl2_sys as sdl;


pub struct Texture{
    pub raw: *mut sdl::SDL_Texture
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { sdl::SDL_DestroyTexture(self.raw)};
    }
}


impl Texture {
    pub const fn from_raw( raw: *mut sdl::SDL_Texture) -> Self {
        Self{ raw }
    }
}