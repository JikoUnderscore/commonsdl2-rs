use std::{ffi::c_void, marker::PhantomData, mem};

use anyhow;
use sdl2_sys as sdl;

pub struct TimerSubsystem;

impl Drop for TimerSubsystem {
    fn drop(&mut self) {
        unsafe {
            sdl::SDL_QuitSubSystem(sdl::SDL_INIT_TIMER);
        }
    }
}

impl TimerSubsystem {
    pub fn new() -> anyhow::Result<TimerSubsystem> {
        let res = unsafe { sdl::SDL_InitSubSystem(sdl::SDL_INIT_TIMER) };

        if res != 0 {
            return unsafe { Err(anyhow::anyhow!("{:?} {}({},{})", std::ffi::CStr::from_ptr(sdl::SDL_GetError()), file!(), line!(), column!())) };
        }

        return Ok(TimerSubsystem {});
    }

    /// Constructs a new timer using the boxed closure `callback`.
    ///
    /// The timer is started immediately, it will be cancelled either:
    ///
    /// * when the timer is dropped
    /// * or when the callback returns a non-positive continuation interval
    ///
    /// The callback is run in a thread that is created and managed internally
    /// by SDL2 from C. The callback *must* not panic!
    pub fn add_timer<'b, 'c>(&'b self, delay: u32, callback: TimerCallback<'c>) -> Timer<'b, 'c> {
        unsafe {
            let callback = Box::new(callback);
            let timer_id = sdl::SDL_AddTimer(delay, Some(c_timer_callback), mem::transmute_copy(&callback));

            Timer { callback: Some(callback), raw: timer_id, _marker: PhantomData }
        }
    }

    /// Gets the number of milliseconds elapsed since the timer subsystem was initialized.
    ///
    /// It's recommended that you use another library for timekeeping, such as `time`.
    ///
    /// This function is not recommended in upstream SDL2 as of 2.0.18 and internally
    /// calls the 64-bit variant and masks the result.
    #[doc(alias = "SDL_GetTicks")]
    pub fn ticks(&self) -> u32 {
        // This is thread-safe as long as the ticks subsystem is inited, and
        // tying this to `TimerSubsystem` ensures the timer subsystem can
        // safely make calls into the ticks subsystem without invoking a
        // thread-unsafe `SDL_TicksInit()`.
        //
        // This binding is offered for completeness but is debatably a relic.
        unsafe { sdl::SDL_GetTicks() }
    }

    /// Gets the number of milliseconds elapsed since the timer subsystem was initialized.
    ///
    /// It's recommended that you use another library for timekeeping, such as `time`.
    #[doc(alias = "SDL_GetTicks64")]
    pub fn ticks64(&self) -> u64 {
        // This is thread-safe as long as the ticks subsystem is inited, and
        // tying this to `TimerSubsystem` ensures the timer subsystem can
        // safely make calls into the ticks subsystem without invoking a
        // thread-unsafe `SDL_TicksInit()`.
        //
        // This binding is offered for completeness but is debatably a relic.
        unsafe { sdl::SDL_GetTicks64() }
    }

    /// Sleeps the current thread for the specified amount of milliseconds.
    ///
    /// It's recommended that you use `std::thread::sleep()` instead.
    #[doc(alias = "SDL_Delay")]
    pub fn delay(&self, ms: u32) {
        // This is thread-safe as long as the ticks subsystem is inited, and
        // tying this to `TimerSubsystem` ensures the timer subsystem can
        // safely make calls into the ticks subsystem without invoking a
        // thread-unsafe `SDL_TicksInit()`.
        //
        // This binding is offered for completeness but is debatably a relic.
        unsafe { sdl::SDL_Delay(ms) }
    }

    #[doc(alias = "SDL_GetPerformanceCounter")]
    pub fn performance_counter(&self) -> u64 {
        unsafe { sdl::SDL_GetPerformanceCounter() }
    }

    #[doc(alias = "SDL_GetPerformanceFrequency")]
    pub fn performance_frequency(&self) -> u64 {
        unsafe { sdl::SDL_GetPerformanceFrequency() }
    }
}

pub type TimerCallback<'a> = Box<dyn FnMut() -> u32 + 'a + Send>;

pub struct Timer<'b, 'a> {
    callback: Option<Box<TimerCallback<'a>>>,
    raw: sdl::SDL_TimerID,
    _marker: PhantomData<&'b ()>,
}

impl<'b, 'a> Timer<'b, 'a> {
    /// Returns the closure as a trait-object and cancels the timer
    /// by consuming it...
    pub fn into_inner(mut self) -> TimerCallback<'a> {
        *self.callback.take().unwrap()
    }
}

impl<'b, 'a> Drop for Timer<'b, 'a> {
    #[inline]
    #[doc(alias = "SDL_RemoveTimer")]
    fn drop(&mut self) {
        // SDL_RemoveTimer returns SDL_FALSE if the timer wasn't found (impossible),
        // or the timer has been cancelled via the callback (possible).
        // The timer being cancelled isn't an issue, so we ignore the result.
        unsafe { sdl::SDL_RemoveTimer(self.raw) };
    }
}

extern "C" fn c_timer_callback(_interval: u32, param: *mut c_void) -> u32 {
    // FIXME: This is UB if the callback panics! (But will realistically
    // crash on stack underflow.)
    //
    // I tried using `std::panic::catch_unwind()` here and it compiled but
    // would not catch. Maybe wait for `c_unwind` to stabilize? Then the behavior
    // will automatically abort the process when panicking over an `extern "C"`
    // function.
    let f = param as *mut TimerCallback<'_>;
    unsafe { (*f)() }
}
