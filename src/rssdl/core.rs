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
#[derive(Clone, Copy)]
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

    pub fn center(&self) -> Point {
        let x = self.raw.x + (self.raw.w / 2);
        let y = self.raw.y + (self.raw.h / 2);
        Point::new(x, y)
    }

    pub fn left(&self) -> i32 {
        self.raw.x
    }

    pub fn right(&self) -> i32 {
        self.raw.x + self.raw.w
    }

    pub fn top(&self) -> i32 {
        self.raw.y
    }

    pub fn bottom(&self) -> i32 {
        self.raw.y + self.raw.h
    }

    pub fn top_left(&self) -> Point {
        Point::new(self.left(), self.top())
    }

    pub fn top_right(&self) -> Point {
        Point::new(self.right(), self.top())
    }

    pub fn bottom_left(&self) -> Point {
        Point::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> Point {
        Point::new(self.right(), self.bottom())
    }

    pub fn center_on<P>(&mut self, point: P)
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        self.raw.x = x - self.raw.w / 2;
        self.raw.y = y - self.raw.h / 2;
    }

    pub fn offset(&mut self, x: i32, y: i32) {
        match self.raw.x.checked_add(x) {
            Some(val) => self.raw.x = val,
            None => {
                if x >= 0 {
                    self.raw.x = max_int_value() as i32;
                } else {
                    self.raw.x = i32::min_value();
                }
            }
        }
        match self.raw.y.checked_add(y) {
            Some(val) => self.raw.y = val,
            None => {
                if y >= 0 {
                    self.raw.y = max_int_value() as i32;
                } else {
                    self.raw.y = i32::min_value();
                }
            }
        }
    }

    pub fn contains_point<P>(&self, point: P) -> bool
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        let inside_x = x >= self.left() && x < self.right();
        inside_x && (y >= self.top() && y < self.bottom())
    }

    pub fn is_contains_rect(&self, other: Rect) -> bool {
        other.left() >= self.left() && other.right() <= self.right() && other.top() >= self.top() && other.bottom() <= self.bottom()
    }

    pub const fn from_raw(raw: sdl::SDL_Rect) -> Rect {
        Rect::new(raw.x, raw.y, raw.w, raw.h)
    }

    pub fn from_enclose_points<R: Into<Option<Rect>>>(points: &[Point], clipping_rect: R) -> Option<Rect>
    where
        R: Into<Option<Rect>>,
    {
        let clipping_rect = clipping_rect.into();

        if points.is_empty() {
            return None;
        }

        let mut out = std::mem::MaybeUninit::uninit();

        let clip_ptr: *const sdl::SDL_Rect = match clipping_rect.as_ref() {
            Some(r) => &r.raw,
            None => std::ptr::null(),
        };

        let result =
            unsafe { sdl::SDL_EnclosePoints(Point::raw_slice(points), points.len() as i32, clip_ptr, out.as_mut_ptr()) != sdl::SDL_bool::SDL_FALSE };

        if result {
            let out = unsafe { out.assume_init() };

            // Return an error if the dimensions are too large.
            Some(Rect::from_raw(out))
        } else {
            None
        }
    }

    pub fn has_intersection(&self, other: Rect) -> bool {
        unsafe { sdl::SDL_HasIntersection(&self.raw, &other.raw) != sdl::SDL_bool::SDL_FALSE }
    }

    pub fn intersection(&self, other: Rect) -> Option<Rect> {
        let mut out = std::mem::MaybeUninit::uninit();

        let success = unsafe { sdl::SDL_IntersectRect(&self.raw, &other.raw, out.as_mut_ptr()) != sdl::SDL_bool::SDL_FALSE };

        if success {
            let out = unsafe { out.assume_init() };
            Some(Rect::from_raw(out))
        } else {
            None
        }
    }

    pub fn union(&self, other: Rect) -> Rect {
        let mut out = std::mem::MaybeUninit::uninit();

        unsafe {
            // If `self` and `other` are both empty, `out` remains uninitialized.
            // Because empty rectangles aren't allowed in Rect, we don't need to worry about this.
            sdl::SDL_UnionRect(&self.raw, &other.raw, out.as_mut_ptr())
        };

        let out = unsafe { out.assume_init() };

        Rect::from_raw(out)
    }

    pub fn intersect_line(&self, start: Point, end: Point) -> Option<(Point, Point)> {
        let (mut start_x, mut start_y) = (start.raw.x, start.raw.y);
        let (mut end_x, mut end_y) = (end.raw.x, end.raw.y);

        let intersected =
            unsafe { sdl::SDL_IntersectRectAndLine(&self.raw, &mut start_x, &mut start_y, &mut end_x, &mut end_y) != sdl::SDL_bool::SDL_FALSE };

        if intersected {
            Some((Point::new(start_x, start_y), Point::new(end_x, end_y)))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub raw: sdl::SDL_Point,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Point {
        Point::new(x, y)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.raw.x == other.raw.x && self.raw.y == other.raw.y
    }
}
impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.raw.x, self.raw.y)
    }
}
impl Eq for Point {}

impl std::fmt::Debug for Point {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return write!(fmt, "Point {{ x: {}, y: {} }}", self.raw.x, self.raw.y);
    }
}

/// The maximal integer value that can be used for rectangles.
///
/// This value is smaller than strictly needed, but is useful in ensuring that
/// rect sizes will never have to be truncated when clamping.
pub fn max_int_value() -> u32 {
    i32::max_value() as u32 / 2
}

/// The minimal integer value that can be used for rectangle positions
/// and points.
///
/// This value is needed, because otherwise the width of a rectangle created
/// from a point would be able to exceed the maximum width.
pub fn min_int_value() -> i32 {
    i32::min_value() / 2
}

fn clamped_mul(a: i32, b: i32) -> i32 {
    match a.checked_mul(b) {
        Some(val) => val,
        None => {
            if (a < 0) ^ (b < 0) {
                min_int_value()
            } else {
                max_int_value() as i32
            }
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { raw: sdl::SDL_Point { x, y } }
    }

    pub fn from_raw(raw: sdl::SDL_Point) -> Point {
        Point::new(raw.x, raw.y)
    }

    pub fn raw_slice(slice: &[Point]) -> *const sdl::SDL_Point {
        slice.as_ptr() as *const sdl::SDL_Point
    }

    /// Returns a new point by shifting this point's coordinates by the given
    /// x and y values.
    pub fn offset(self, x: i32, y: i32) -> Point {
        let x = match self.raw.x.checked_add(x) {
            Some(val) => val,
            None => {
                if x < 0 {
                    min_int_value()
                } else {
                    max_int_value() as i32
                }
            }
        };
        let y = match self.raw.y.checked_add(y) {
            Some(val) => val,
            None => {
                if y < 0 {
                    min_int_value()
                } else {
                    max_int_value() as i32
                }
            }
        };
        return Point::new(x, y);
    }

    /// Returns a new point by multiplying this point's coordinates by the
    /// given scale factor.
    pub fn scale_safe(self, f: i32) -> Point {
        Point::new(clamped_mul(self.raw.x, f), clamped_mul(self.raw.y, f))
    }
    pub fn scale_(self, f: i32) -> Point {
        Point::new(self.raw.x * f, self.raw.y * f)
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

#[derive(Debug, Clone, Copy)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl Default for IVec2 {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
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
