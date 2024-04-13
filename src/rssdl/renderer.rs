use anyhow;
use sdl2_sys as sdl;

use crate::{
    core::{Color, IVec2, Rect, Vec2},
    surface::Surface,
    textures::Texture,
};

pub struct SDLContext;

impl Drop for SDLContext {
    fn drop(&mut self) {
        unsafe {
            sdl::SDL_Quit();
        }
    }
}

pub struct ImageContext;

impl Drop for ImageContext {
    fn drop(&mut self) {
        unsafe {
            sdl::image::IMG_Quit();
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum WindowPos {
    Undefined,
    Centered,
    Positioned(i32),
}

fn to_raw_windowpos(pos: WindowPos) -> i32 {
    match pos {
        WindowPos::Undefined => sdl::SDL_WINDOWPOS_UNDEFINED_MASK as i32,
        WindowPos::Centered => sdl::SDL_WINDOWPOS_CENTERED_MASK as i32,
        WindowPos::Positioned(x) => x as i32,
    }
}

pub struct Window {
    pub renderer: Renderer,
    raw_window: *mut sdl::SDL_Window,
}
impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            sdl::SDL_DestroyWindow(self.raw_window);
            sdl::SDL_DestroyRenderer(self.renderer.raw);
        }
    }
}
pub struct WindowBuilder {
    index: Option<u32>,
    renderer_flags: u32,
    title: &'static str,
    width: u32,
    height: u32,
    x: WindowPos,
    y: WindowPos,
    window_flags: u32,
    is_shaped: bool,
}

impl Window {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_logical_size(&self, logical_width: i32, logical_height: i32) -> anyhow::Result<()> {
        unsafe {
            if sdl::SDL_RenderSetLogicalSize(self.renderer.raw, logical_width, logical_height) != 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }
        }
        Ok(())
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn window_size(&self) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            sdl::SDL_GetWindowSize(self.raw_window, &mut w, &mut h);
        }
        return (w, h);
    }
}

impl WindowBuilder {
    pub fn build(&self) -> anyhow::Result<Window> {
        unsafe {
            let window = if self.is_shaped {
                sdl::SDL_CreateShapedWindow(
                    self.title.as_ptr() as *const _,
                    to_raw_windowpos(self.x) as u32,
                    to_raw_windowpos(self.y) as u32,
                    self.width,
                    self.height,
                    self.window_flags,
                )
            } else {
                sdl::SDL_CreateWindow(
                    self.title.as_ptr() as *const _,
                    to_raw_windowpos(self.x),
                    to_raw_windowpos(self.y),
                    self.width as i32,
                    self.height as i32,
                    self.window_flags,
                )
            };

            if window.is_null() {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }

            let index = match self.index {
                None => -1,
                Some(index) => index as i32,
            };

            let renderer = sdl::SDL_CreateRenderer(window, index, self.renderer_flags);
            if renderer.is_null() {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }

            return Ok(Window { renderer: Renderer { raw: renderer }, raw_window: window });
        }
    }

    pub fn set_shaped(&mut self) -> &mut WindowBuilder {
        self.is_shaped = true;
        self
    }

    pub fn set_window_flags(&mut self, flags: u32) -> &mut WindowBuilder {
        self.window_flags = flags;
        self
    }

    pub fn position(&mut self, x: i32, y: i32) -> &mut WindowBuilder {
        self.x = WindowPos::Positioned(x);
        self.y = WindowPos::Positioned(y);
        self
    }

    pub fn position_centered(&mut self) -> &mut WindowBuilder {
        self.x = WindowPos::Centered;
        self.y = WindowPos::Centered;
        self
    }

    pub fn fullscreen(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32;
        self
    }

    pub fn fullscreen_desktop(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
        self
    }

    // pub fn opengl(&mut self) -> &mut WindowBuilder {
    //     self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
    //     self
    // }

    // pub fn vulkan(&mut self) -> &mut WindowBuilder {
    //     self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_VULKAN as u32;
    //     self
    // }

    pub fn hidden(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32;
        self
    }

    pub fn borderless(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32;
        self
    }

    pub fn resizable(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;
        self
    }

    pub fn minimized(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32;
        self
    }

    pub fn maximized(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32;
        self
    }

    pub fn input_grabbed(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED as u32;
        self
    }

    /// Creates the window in high-DPI mode if supported (>= SDL 2.0.1)
    pub fn allow_highdpi(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32;
        self
    }

    /// Window should always above others (>= SDL 2.0.5)
    pub fn always_on_top(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sdl::SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP as u32;
        self
    }

    // pub fn metal_view(&mut self) -> &mut WindowBuilder {
    //     self.create_metal_view = true;
    //     self
    // }

    pub fn index(&mut self, index: u32) -> &mut WindowBuilder {
        self.index = Some(index);
        self
    }

    pub fn software(&mut self) -> &mut WindowBuilder {
        self.renderer_flags |= sdl::SDL_RendererFlags::SDL_RENDERER_SOFTWARE as u32;
        self
    }

    pub fn accelerated(&mut self) -> &mut WindowBuilder {
        self.renderer_flags |= sdl::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32;
        self
    }

    pub fn present_vsync(&mut self) -> &mut WindowBuilder {
        self.renderer_flags |= sdl::SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32;
        self
    }

    pub fn target_texture(&mut self) -> &mut WindowBuilder {
        self.renderer_flags |= sdl::SDL_RendererFlags::SDL_RENDERER_TARGETTEXTURE as u32;
        self
    }
}

impl SDLContext {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn init() -> anyhow::Result<SDLContext> {
        unsafe {
            if sdl::SDL_Init(sdl::SDL_INIT_VIDEO | sdl::SDL_INIT_EVENTS) != 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }

            assert_eq!(sdl::SDL_SetHint("SDL_HINT_RENDER_BATCHING".as_ptr() as _, "1".as_ptr() as _), sdl::SDL_bool::SDL_TRUE);
        }

        return Ok(SDLContext {});
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn window_builder(&self, title: &'static str, width: u32, height: u32) -> WindowBuilder {
        let title_last = title.len() - 1;
        assert_eq!(title.chars().nth(title_last).expect("to get the last char"), '\0', "Title must be zero terminated");

        return WindowBuilder {
            title,
            width,
            height,
            x: WindowPos::Undefined,
            y: WindowPos::Undefined,
            is_shaped: false,
            index: None,
            renderer_flags: 0,
            window_flags: 0,
        };
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn init_image() -> anyhow::Result<ImageContext> {
        let flags = sdl::image::IMG_InitFlags_IMG_INIT_PNG;
        unsafe {
            let init_flags = sdl::image::IMG_Init(flags as i32) as u32;
            if (init_flags & flags) != flags {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }
        }

        return Ok(ImageContext {});
    }
}

#[allow(non_snake_case)]
pub mod WindowFlags {
    use sdl2_sys as sdl;
    pub const SHOWN: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_SHOWN as u32;
    pub const MOUSE_GRABBED: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MOUSE_GRABBED as u32;
    pub const INPUT_FOCUS: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32;
    pub const MOUSE_FOCUS: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MOUSE_FOCUS as u32;
    pub const FOREIGN: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_FOREIGN as u32;
    pub const MOUSE_CAPTURE: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MOUSE_CAPTURE as u32;
    pub const SKIP_TASKBAR: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_SKIP_TASKBAR as u32;
    pub const UTILITY: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_UTILITY as u32;
    pub const TOOLTIP: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_TOOLTIP as u32;
    pub const POPUP_MENU: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_POPUP_MENU as u32;
    pub const KEYBOARD_GRABBED: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_KEYBOARD_GRABBED as u32;
}

pub struct Renderer {
    raw: *mut sdl::SDL_Renderer,
}

impl Renderer {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn load_texture_from_file(&self, filename: &str) -> anyhow::Result<Texture> {
        let title_last = filename.len() - 1;
        assert_eq!(filename.chars().nth(title_last).expect("to get the last char"), '\0', "File path must be zero terminated");
        unsafe {
            let raw = sdl::image::IMG_LoadTexture(self.raw, filename.as_ptr() as *const _);
            if raw.is_null() {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }

            return Ok(Texture::from_raw(raw));
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn load_texture_bytes(&self, buf: &[u8]) -> anyhow::Result<Texture> {
        //! Loads an SDL Texture from a buffer that the format must be something supported by SDL2_image (png, jpeg, ect, but NOT RGBA8888 bytes for instance)
        unsafe {
            let buf = sdl::SDL_RWFromMem(buf.as_ptr() as *mut _, buf.len() as i32);
            let raw = sdl::image::IMG_LoadTexture_RW(self.raw, buf, 1); // close(free) buff after load
            if (raw as *mut ()).is_null() {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            } else {
                return Ok(Texture::from_raw(raw));
            }
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn create_texture_from_surface(&self, surface: &Surface) -> anyhow::Result<Texture> {
        let result = unsafe { sdl::SDL_CreateTextureFromSurface(self.raw, surface.raw) };
        if result.is_null() {
            return unsafe { Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!())) };
        } else {
            return Ok(Texture { raw: result });
        }
    }

    // pub fn warp_mouse_in_window(&self, x: i32, y: i32) {
    //     unsafe { sdl::SDL_WarpMouseInWindow(self.window, x, y); }
    // }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_draw_color_checked(&self, color: impl Into<Color>) {
        let c = color.into();
        unsafe {
            if sdl::SDL_SetRenderDrawColor(self.raw, c.r, c.g, c.b, c.a) < 0 {
                panic!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!(),);
            }
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_draw_color(&self, color: impl Into<Color>) {
        let c = color.into();
        unsafe {
            sdl::SDL_SetRenderDrawColor(self.raw, c.r, c.g, c.b, c.a);
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn draw_rect_checked(&self, rect: &Rect) -> anyhow::Result<()> {
        unsafe {
            if sdl::SDL_RenderDrawRect(self.raw, &rect.raw) < 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }
        }
        return Ok(());
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn draw_rect(&self, rect: &Rect) {
        unsafe {
            sdl::SDL_RenderDrawRect(self.raw, &rect.raw);
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn fill_rect_checked(&self, rect: &Rect) -> anyhow::Result<()> {
        unsafe {
            if sdl::SDL_RenderFillRect(self.raw, &rect.raw) < 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!()));
            }
        }
        return Ok(());
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn fill_rect(&self, rect: &Rect) {
        unsafe {
            sdl::SDL_RenderFillRect(self.raw, &rect.raw);
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn clear(&self) {
        unsafe {
            sdl::SDL_RenderClear(self.raw);
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn present(&mut self) {
        unsafe {
            sdl::SDL_RenderPresent(self.raw);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect) {
        unsafe {
            sdl::SDL_RenderCopy(self.raw, texture.raw, &src_rect.raw, &dst_rect.raw);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_miss_src(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect) {
        unsafe {
            sdl::SDL_RenderCopy(self.raw, texture.raw, std::ptr::null(), &dst_rect.raw);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_miss_src_dst(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect) {
        unsafe {
            sdl::SDL_RenderCopy(self.raw, texture.raw, std::ptr::null(), std::ptr::null());
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_miss_dst(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect) {
        unsafe {
            sdl::SDL_RenderCopy(self.raw, texture.raw, &src_rect.raw, std::ptr::null());
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, center: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        let point2d = sdl::SDL_Point { x: center.x, y: center.y };
        unsafe {
            let flip = match (flip_horizontal, flip_vertical) {
                (false, false) => sdl::SDL_RendererFlip::SDL_FLIP_NONE,
                (true, false) => sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                (false, true) => sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL,
                (true, true) => std::mem::transmute::<u32, sdl::SDL_RendererFlip>(
                    std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL)
                        | std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL),
                ),
            };

            sdl::SDL_RenderCopyEx(self.raw, texture.raw, &src_rect.raw, &dst_rect.raw, angle, &point2d, flip);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex_miss_src(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, center: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        let point2d = sdl::SDL_Point { x: center.x, y: center.y };
        unsafe {
            let flip = match (flip_horizontal, flip_vertical) {
                (false, false) => sdl::SDL_RendererFlip::SDL_FLIP_NONE,
                (true, false) => sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                (false, true) => sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL,
                (true, true) => std::mem::transmute::<u32, sdl::SDL_RendererFlip>(
                    std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL)
                        | std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL),
                ),
            };

            sdl::SDL_RenderCopyEx(self.raw, texture.raw, std::ptr::null(), &dst_rect.raw, angle, &point2d, flip);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex_miss_src_dst(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, center: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        let point2d = sdl::SDL_Point { x: center.x, y: center.y };
        unsafe {
            let flip = match (flip_horizontal, flip_vertical) {
                (false, false) => sdl::SDL_RendererFlip::SDL_FLIP_NONE,
                (true, false) => sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                (false, true) => sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL,
                (true, true) => std::mem::transmute::<u32, sdl::SDL_RendererFlip>(
                    std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL)
                        | std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL),
                ),
            };

            sdl::SDL_RenderCopyEx(self.raw, texture.raw, std::ptr::null(), std::ptr::null(), angle, &point2d, flip);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex_miss_src_center(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, center: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        unsafe {
            let flip = match (flip_horizontal, flip_vertical) {
                (false, false) => sdl::SDL_RendererFlip::SDL_FLIP_NONE,
                (true, false) => sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                (false, true) => sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL,
                (true, true) => std::mem::transmute::<u32, sdl::SDL_RendererFlip>(
                    std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL)
                        | std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL),
                ),
            };

            sdl::SDL_RenderCopyEx(self.raw, texture.raw, std::ptr::null(), &dst_rect.raw, angle, std::ptr::null(), flip);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex_miss_center(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, center: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        unsafe {
            let flip = match (flip_horizontal, flip_vertical) {
                (false, false) => sdl::SDL_RendererFlip::SDL_FLIP_NONE,
                (true, false) => sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                (false, true) => sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL,
                (true, true) => std::mem::transmute::<u32, sdl::SDL_RendererFlip>(
                    std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL)
                        | std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL),
                ),
            };
            sdl::SDL_RenderCopyEx(self.raw, texture.raw, &src_rect.raw, &dst_rect.raw, angle, std::ptr::null(), flip);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex_miss_center_dst(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, center: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        unsafe {
            let flip = match (flip_horizontal, flip_vertical) {
                (false, false) => sdl::SDL_RendererFlip::SDL_FLIP_NONE,
                (true, false) => sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                (false, true) => sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL,
                (true, true) => std::mem::transmute::<u32, sdl::SDL_RendererFlip>(
                    std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL)
                        | std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL),
                ),
            };
            sdl::SDL_RenderCopyEx(self.raw, texture.raw, &src_rect.raw, std::ptr::null(), angle, std::ptr::null(), flip);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex_miss_center_dst_src(&mut self, texture: &Texture, src_rect: Rect, dst_rect: Rect, angle: f64, center: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        unsafe {
            let flip = match (flip_horizontal, flip_vertical) {
                (false, false) => sdl::SDL_RendererFlip::SDL_FLIP_NONE,
                (true, false) => sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                (false, true) => sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL,
                (true, true) => std::mem::transmute::<u32, sdl::SDL_RendererFlip>(
                    std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_HORIZONTAL)
                        | std::mem::transmute::<sdl::SDL_RendererFlip, u32>(sdl::SDL_RendererFlip::SDL_FLIP_VERTICAL),
                ),
            };
            sdl::SDL_RenderCopyEx(self.raw, texture.raw, std::ptr::null(), std::ptr::null(), angle, std::ptr::null(), flip);
        }
    }
}
