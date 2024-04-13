use std::path::Path;

use anyhow;
use sdl2_sys as sdl;

pub struct RWops {
    pub raw: *mut sdl::SDL_RWops,
}

impl RWops {
    pub unsafe fn from_raw(raw: *mut sdl::SDL_RWops) -> RWops {
        RWops { raw }
    }

    pub fn from_file<P: AsRef<Path>>(path: P, mode: &str) -> anyhow::Result<RWops> {
        let pathstr = path.as_ref().to_str().unwrap();
        assert_eq!(pathstr.chars().nth(pathstr.len() - 1).expect("to get the last char"), '\0', "Path must be zero terminated");

        assert_eq!(mode.chars().nth(mode.len() - 1).expect("to get the last char"), '\0', "Mode must be zero terminated");

        let raw = unsafe { sdl::SDL_RWFromFile(pathstr.as_ptr() as *const _, mode.as_ptr() as *const _) };

        if raw.is_null() {
            return unsafe { Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!())) };
        } else {
            return Ok(RWops { raw });
        }
    }

    pub fn from_bytes(buf: &[u8]) -> anyhow::Result<RWops> {
        let raw = unsafe { sdl::SDL_RWFromConstMem(buf.as_ptr() as *const _, buf.len() as _) };

        if raw.is_null() {
            return unsafe { Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!())) };
        } else {
            return Ok(RWops { raw });
        }
    }

    pub fn from_read<T>(r: &mut T, buffer: &mut Vec<u8>) -> anyhow::Result<RWops>
    where
        T: std::io::Read + Sized,
    {
        match r.read_to_end(buffer) {
            Ok(_size) => RWops::from_bytes(buffer),
            Err(ioerror) => Err(anyhow::anyhow!("{:?} {}({},{})", ioerror, file!(), line!(), column!())),
        }
    }

    pub fn from_bytes_mut(buf: &mut [u8]) -> anyhow::Result<RWops> {
        let raw = unsafe { sdl::SDL_RWFromMem(buf.as_ptr() as *mut _, buf.len() as _) };

        if raw.is_null() {
            return unsafe { Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!())) };
        } else {
            Ok(RWops { raw })
        }
    }

    pub fn len(&self) -> Option<usize> {
        let result = unsafe { ((*self.raw).size.unwrap())(self.raw) };

        match result {
            -1 => None,
            v => Some(v as usize),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.len() {
            Some(s) => s == 0,
            None => true,
        }
    }
}

impl Drop for RWops {
    fn drop(&mut self) {
        let ret = unsafe { ((*self.raw).close.unwrap())(self.raw) };
        if ret != 0 {
            panic!("{:?}", unsafe { std::ffi::CStr::from_ptr(sdl::SDL_GetError()) });
        }
    }
}

impl std::io::Read for RWops {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let out_len = buf.len();
        // FIXME: it's better to use as_mut_ptr().
        // number of objects read, or 0 at error or end of file.
        let ret = unsafe { ((*self.raw).read.unwrap())(self.raw, buf.as_ptr() as *mut _, 1, out_len as sdl::size_t) };
        Ok(ret as usize)
    }
}

impl std::io::Write for RWops {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let in_len = buf.len();
        let ret = unsafe { ((*self.raw).write.unwrap())(self.raw, buf.as_ptr() as *const _, 1, in_len as sdl::size_t) };
        Ok(ret as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::io::Seek for RWops {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        // whence code is different from SeekStyle
        let (whence, offset) = match pos {
            std::io::SeekFrom::Start(pos) => (sdl::RW_SEEK_SET, pos as i64),
            std::io::SeekFrom::End(pos) => (sdl::RW_SEEK_END, pos),
            std::io::SeekFrom::Current(pos) => (sdl::RW_SEEK_CUR, pos),
        };
        let ret = unsafe { ((*self.raw).seek.unwrap())(self.raw, offset, std::mem::transmute(whence)) };
        if ret == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret as u64)
        }
    }
}
