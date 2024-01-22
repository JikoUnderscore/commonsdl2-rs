use std::fmt::Debug;

use sdl2_sys as sdl;


pub struct FpsCap {
    frame_delay: u32,
    pub dt: f32,
    last_time: u64,
}
static mut PERFORMANCE_FREQUENCY: f32 = 0.0;

impl FpsCap {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn new(fps_cap: u32) -> Self {
        unsafe {
            PERFORMANCE_FREQUENCY = sdl::SDL_GetPerformanceFrequency() as f32;
        }

        Self { frame_delay: 1000 / fps_cap, dt: 0.0, last_time: unsafe { sdl::SDL_GetPerformanceCounter() } }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn start(&mut self) {
        let now2 = unsafe { sdl::SDL_GetPerformanceCounter() };
        let elapsed = now2 - self.last_time;
        self.dt = (elapsed as f32) / (unsafe { PERFORMANCE_FREQUENCY });
        // println!("FPS: {} | dt {}", 1.0 / self.dt, self.dt);
        self.last_time = now2;
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn end(&self) {
        let now1 = unsafe { sdl::SDL_GetPerformanceCounter() };
        let elapsed = now1 - self.last_time;
        let cap_frame_end = (((elapsed as f32) * 1000.0) / (unsafe { PERFORMANCE_FREQUENCY })) as u32;

        if cap_frame_end < self.frame_delay {
            let ms = self.frame_delay - cap_frame_end;
            if ms > 0 {
                unsafe { sdl::SDL_Delay(ms) };
            }
        }
    }
}

pub struct Rect {
    pub raw: sdl::SDL_Rect,
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return write!(fmt, "Rect {{ x: {}, y: {}, w: {}, h: {} }}", self.raw.x, self.raw.y, self.raw.w, self.raw.h);
    }
}

impl Rect {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { raw: sdl::SDL_Rect { x, y, w, h } }
    }

    pub fn is_contaning_point(&self, point: IVec2) -> bool {
        point.x >= self.raw.x && point.x <= self.raw.x + self.raw.w && point.y >= self.raw.y && point.y <= self.raw.y + self.raw.h
    }

    pub fn is_colliding(&self, rect: &Rect) -> bool {
        self.raw.x < rect.raw.x  + rect.raw.w && // Collision on Left of a and right of b
        self.raw.x + self.raw.w > rect.raw.x  && // Collision on Right of a and left of b
        self.raw.y < rect.raw.y  + rect.raw.h && // Collision on Bottom of a and Top of b
        self.raw.y + self.raw.h > rect.raw.y // Collision on Top of a and Bottom of b
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn as_raw(&self) -> sdl::SDL_Color {
        sdl::SDL_Color { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color { r, g, b, a: 0xff }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color { r, g, b, a }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
impl IVec2 {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Camera {
    zoom: f32,
    dimension: Vec2,
    position: Vec2,
}

impl Camera {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub const fn new() -> Self {
        Self { zoom: 1.0, dimension: Vec2::new(0.0, 0.0), position: Vec2::new(0.0, 0.0) }
    }
}
