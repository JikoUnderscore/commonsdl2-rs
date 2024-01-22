use anyhow;
use sdl2_sys as sdl;

use crate::{
    core::{Color, IVec2, Rect, Vec2},
    textures::Texture,
};




pub struct SDLContext {
    pub renderer: Renderer,
    raw_window: *mut sdl::SDL_Window,
}

impl Drop for SDLContext {
    fn drop(&mut self) {
        unsafe {
            sdl::SDL_DestroyWindow(self.raw_window);
            sdl::SDL_DestroyRenderer(self.renderer.raw);
            sdl::SDL_Quit();
        }
    }
}

impl SDLContext {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn init_image() -> anyhow::Result<()> {
        let flags = sdl::image::IMG_InitFlags_IMG_INIT_PNG;
        unsafe {
            let init_flags = sdl::image::IMG_Init(flags as i32) as u32;
            if (init_flags & flags) != flags {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }
        }

        return Ok(());
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn init_window_and_renderer<const WIDTH: u32, const HEIGHT: u32, const WINDOWFLAGS: u32, const LOGICAL_WIDTH: u32, const LOGICAL_HEIGHT: u32, const RENDERERFLAGS: u32>(title: &str) -> anyhow::Result<Self> {
        let title_last = title.len() - 1;
        assert_eq!(title.chars().nth(title_last).expect("to get the last char"), '\0', "Title must be zero terminated");
        unsafe {
            assert_eq!(sdl::SDL_SetHint("SDL_HINT_RENDER_BATCHING".as_ptr() as _, "1".as_ptr() as _), sdl::SDL_bool::SDL_TRUE);

            if sdl::SDL_Init(sdl::SDL_INIT_VIDEO | sdl::SDL_INIT_EVENTS) != 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }

            let window = sdl::SDL_CreateWindow(
                title.as_ptr() as *const _,
                sdl::SDL_WINDOWPOS_CENTERED_MASK as i32,
                sdl::SDL_WINDOWPOS_CENTERED_MASK as i32,
                WIDTH as i32,
                HEIGHT as i32,
                WINDOWFLAGS,
            );
            if window.is_null() {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }

            let renderer = sdl::SDL_CreateRenderer(
                window,
                -1,
                RENDERERFLAGS,
            );
            if renderer.is_null() {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }

            if sdl::SDL_RenderSetLogicalSize(renderer, LOGICAL_WIDTH as i32, LOGICAL_HEIGHT as i32) != 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }

            println!("SDL_GetHint {:?}", sdl::SDL_GetHint("SDL_HINT_RENDER_BATCHING".as_ptr() as *const i8));

            return Ok(SDLContext { renderer: Renderer { raw: renderer }, raw_window: window });
        }
    }

    pub fn window_size(&self) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            sdl::SDL_GetWindowSize(self.raw_window, &mut w, &mut h);
        }
        return (w , h );
    }
}

#[allow(non_snake_case)]
pub mod WindowFlags {
    use sdl2_sys as sdl;
    pub const FULLSCREEN: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN as u32;
    pub const OPENGL: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
    pub const SHOWN: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_SHOWN as u32;
    pub const HIDDEN: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32;
    pub const BORDERLESS: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32;
    pub const RESIZABLE: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;
    pub const MINIMIZED: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MINIMIZED as u32;
    pub const MAXIMIZED: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MAXIMIZED as u32;
    pub const MOUSE_GRABBED: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MOUSE_GRABBED as u32;
    pub const INPUT_FOCUS: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32;
    pub const MOUSE_FOCUS: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MOUSE_FOCUS as u32;
    pub const FULLSCREEN_DESKTOP: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32;
    pub const FOREIGN: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_FOREIGN as u32;
    pub const ALLOW_HIGHDPI: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32;
    pub const MOUSE_CAPTURE: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_MOUSE_CAPTURE as u32;
    pub const ALWAYS_ON_TOP: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP as u32;
    pub const SKIP_TASKBAR: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_SKIP_TASKBAR as u32;
    pub const UTILITY: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_UTILITY as u32;
    pub const TOOLTIP: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_TOOLTIP as u32;
    pub const POPUP_MENU: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_POPUP_MENU as u32;
    pub const KEYBOARD_GRABBED: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_KEYBOARD_GRABBED as u32;
    pub const VULKAN: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_VULKAN as u32;
    pub const METAL: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_METAL as u32;
    pub const INPUT_GRABBED: u32 = sdl::SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED as u32;
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
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }

            return Ok(Texture::from_raw(raw));
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
                panic!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!(), );
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
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
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
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
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
    pub fn wscopy(&mut self, texture: &Texture, sprite_src: Rect, dst: Vec2, size: (i32, i32)) {
        let dstrect = Rect::new(dst.x as i32, dst.y as i32, size.0, size.1);
        unsafe {
            sdl::SDL_RenderCopy(self.raw, texture.raw, &sprite_src.raw, &dstrect.raw);
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn wscopy_checked(&mut self, texture: &Texture, sprite_src: Rect, dst: Vec2, size: (i32, i32)) -> anyhow::Result<()> {
        let dstrect = Rect::new(dst.x as i32, dst.y as i32, size.0, size.1);
        unsafe {
            if sdl::SDL_RenderCopy(self.raw, texture.raw, &sprite_src.raw, &dstrect.raw) < 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }
        }

        return Ok(());
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn wscopy_ex(
        &mut self,
        texture: &Texture,
        sprite_src: Rect,
        dst: Vec2,
        size: (i32, i32),
        angle: f64,
        point: IVec2,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) {
        let dstrect = Rect::new(dst.x as i32, dst.y as i32, size.0, size.1);
        let point2d = sdl::SDL_Point { x: point.x, y: point.y };
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

            sdl::SDL_RenderCopyEx(self.raw, texture.raw, &sprite_src.raw, &dstrect.raw, angle, &point2d, flip);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy(&mut self, texture: &Texture, sprite_src: Rect, dst: Vec2) {
        let dstrect = Rect::new(dst.x as i32, dst.y as i32, sprite_src.raw.w, sprite_src.raw.h);
        unsafe {
            sdl::SDL_RenderCopy(self.raw, texture.raw, &sprite_src.raw, &dstrect.raw);
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_checked(&mut self, texture: &Texture, sprite_src: Rect, dst: Vec2) -> anyhow::Result<()> {
        let dstrect = Rect::new(dst.x as i32, dst.y as i32, sprite_src.raw.w, sprite_src.raw.h);
        unsafe {
            if sdl::SDL_RenderCopy(self.raw, texture.raw, &sprite_src.raw, &dstrect.raw) < 0 {
                return Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(),line!(), column!()));
            }
        }

        return Ok(());
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn copy_ex(&mut self, texture: &Texture, sprite_src: Rect, dst: Vec2, angle: f64, point: IVec2, flip_horizontal: bool, flip_vertical: bool) {
        let dstrect = Rect::new(dst.x as i32, dst.y as i32, sprite_src.raw.w, sprite_src.raw.h);
        let point2d = sdl::SDL_Point { x: point.x, y: point.y };
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

            sdl::SDL_RenderCopyEx(self.raw, texture.raw, &sprite_src.raw, &dstrect.raw, angle, &point2d, flip);
        }
    }
}

pub mod RendererFlags {
    use sdl2_sys as sdl;

    pub const SOFTWARE: u32 = sdl::SDL_RendererFlags::SDL_RENDERER_SOFTWARE as u32;
    pub const ACCELERATED: u32 = sdl::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32;
    pub const PRESENTVSYNC: u32 = sdl::SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC as u32;
    pub const TARGETTEXTURE: u32 = sdl::SDL_RendererFlags::SDL_RENDERER_TARGETTEXTURE as u32;
}
