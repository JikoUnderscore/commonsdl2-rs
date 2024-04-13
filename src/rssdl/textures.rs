use sdl2_sys as sdl;

pub struct Texture {
    pub raw: *mut sdl::SDL_Texture,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { sdl::SDL_DestroyTexture(self.raw) };
    }
}

impl Texture {
    pub const fn from_raw(raw: *mut sdl::SDL_Texture) -> Self {
        Self { raw }
    }

    pub fn set_color_mod(&mut self, red: u8, green: u8, blue: u8) {
        let ret = unsafe { sdl::SDL_SetTextureColorMod(self.raw, red, green, blue) };

        if ret != 0 {
            unsafe {
                panic!("Error setting color mod: {:?}", std::ffi::CStr::from_ptr(sdl::SDL_GetError()));
            }
        }
    }
}
