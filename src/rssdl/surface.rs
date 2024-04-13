use anyhow;
use sdl2_sys as sdl;
use std::path::Path;

use crate::rwops::RWops;

pub struct Surface {
    pub raw: *mut sdl::SDL_Surface,
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            sdl::SDL_FreeSurface(self.raw);
        }
    }
}

impl Surface {
    pub fn load_bmp_rw(rwops: &mut RWops) -> anyhow::Result<Surface> {
        let raw = unsafe { sdl::SDL_LoadBMP_RW(rwops.raw, 0) };

        if raw.is_null() {
            return unsafe { Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!())) };
        } else {
            return Ok(Surface { raw });
        }
    }

    pub fn load_bmp<P: AsRef<Path>>(path: P) -> anyhow::Result<Surface> {
        let mut file = RWops::from_file(path, "rb\0")?;
        return  Surface::load_bmp_rw(&mut file);
    }
}
