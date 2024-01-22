use sdl2_sys as sdl;

pub type Event = sdl::SDL_Event;
pub type EventType = sdl::SDL_EventType;
pub type Scancode = sdl::SDL_Scancode;
#[cfg_attr(not(debug_assertions), inline(always))]
pub fn type_(e: Event) -> EventType {
    const SDL_FIRSTEVENTU32: u32 = sdl::SDL_EventType::SDL_FIRSTEVENT as u32;
    const SDL_QUITU32: u32 = sdl::SDL_EventType::SDL_QUIT as u32;
    const SDL_APP_TERMINATINGU32: u32 = sdl::SDL_EventType::SDL_APP_TERMINATING as u32;
    const SDL_APP_LOWMEMORYU32: u32 = sdl::SDL_EventType::SDL_APP_LOWMEMORY as u32;
    const SDL_APP_WILLENTERBACKGROUNDU32: u32 = sdl::SDL_EventType::SDL_APP_WILLENTERBACKGROUND as u32;
    const SDL_APP_DIDENTERBACKGROUNDU32: u32 = sdl::SDL_EventType::SDL_APP_DIDENTERBACKGROUND as u32;
    const SDL_APP_WILLENTERFOREGROUNDU32: u32 = sdl::SDL_EventType::SDL_APP_WILLENTERFOREGROUND as u32;
    const SDL_APP_DIDENTERFOREGROUNDU32: u32 = sdl::SDL_EventType::SDL_APP_DIDENTERFOREGROUND as u32;
    const SDL_LOCALECHANGEDU32: u32 = sdl::SDL_EventType::SDL_LOCALECHANGED as u32;
    const SDL_DISPLAYEVENTU32: u32 = sdl::SDL_EventType::SDL_DISPLAYEVENT as u32;
    const SDL_WINDOWEVENTU32: u32 = sdl::SDL_EventType::SDL_WINDOWEVENT as u32;
    const SDL_SYSWMEVENTU32: u32 = sdl::SDL_EventType::SDL_SYSWMEVENT as u32;
    const SDL_KEYDOWNU32: u32 = sdl::SDL_EventType::SDL_KEYDOWN as u32;
    const SDL_KEYUPU32: u32 = sdl::SDL_EventType::SDL_KEYUP as u32;
    const SDL_TEXTEDITINGU32: u32 = sdl::SDL_EventType::SDL_TEXTEDITING as u32;
    const SDL_TEXTINPUTU32: u32 = sdl::SDL_EventType::SDL_TEXTINPUT as u32;
    const SDL_KEYMAPCHANGEDU32: u32 = sdl::SDL_EventType::SDL_KEYMAPCHANGED as u32;
    const SDL_MOUSEMOTIONU32: u32 = sdl::SDL_EventType::SDL_MOUSEMOTION as u32;
    const SDL_MOUSEBUTTONDOWNU32: u32 = sdl::SDL_EventType::SDL_MOUSEBUTTONDOWN as u32;
    const SDL_MOUSEBUTTONUPU32: u32 = sdl::SDL_EventType::SDL_MOUSEBUTTONUP as u32;
    const SDL_MOUSEWHEELU32: u32 = sdl::SDL_EventType::SDL_MOUSEWHEEL as u32;
    const SDL_JOYAXISMOTIONU32: u32 = sdl::SDL_EventType::SDL_JOYAXISMOTION as u32;
    const SDL_JOYBALLMOTIONU32: u32 = sdl::SDL_EventType::SDL_JOYBALLMOTION as u32;
    const SDL_JOYHATMOTIONU32: u32 = sdl::SDL_EventType::SDL_JOYHATMOTION as u32;
    const SDL_JOYBUTTONDOWNU32: u32 = sdl::SDL_EventType::SDL_JOYBUTTONDOWN as u32;
    const SDL_JOYBUTTONUPU32: u32 = sdl::SDL_EventType::SDL_JOYBUTTONUP as u32;
    const SDL_JOYDEVICEADDEDU32: u32 = sdl::SDL_EventType::SDL_JOYDEVICEADDED as u32;
    const SDL_JOYDEVICEREMOVEDU32: u32 = sdl::SDL_EventType::SDL_JOYDEVICEREMOVED as u32;
    const SDL_CONTROLLERAXISMOTIONU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERAXISMOTION as u32;
    const SDL_CONTROLLERBUTTONDOWNU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERBUTTONDOWN as u32;
    const SDL_CONTROLLERBUTTONUPU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERBUTTONUP as u32;
    const SDL_CONTROLLERDEVICEADDEDU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERDEVICEADDED as u32;
    const SDL_CONTROLLERDEVICEREMOVEDU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERDEVICEREMOVED as u32;
    const SDL_CONTROLLERDEVICEREMAPPEDU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED as u32;
    const SDL_CONTROLLERTOUCHPADDOWNU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERTOUCHPADDOWN as u32;
    const SDL_CONTROLLERTOUCHPADMOTIONU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERTOUCHPADMOTION as u32;
    const SDL_CONTROLLERTOUCHPADUPU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERTOUCHPADUP as u32;
    const SDL_CONTROLLERSENSORUPDATEU32: u32 = sdl::SDL_EventType::SDL_CONTROLLERSENSORUPDATE as u32;
    const SDL_FINGERDOWNU32: u32 = sdl::SDL_EventType::SDL_FINGERDOWN as u32;
    const SDL_FINGERUPU32: u32 = sdl::SDL_EventType::SDL_FINGERUP as u32;
    const SDL_FINGERMOTIONU32: u32 = sdl::SDL_EventType::SDL_FINGERMOTION as u32;
    const SDL_DOLLARGESTUREU32: u32 = sdl::SDL_EventType::SDL_DOLLARGESTURE as u32;
    const SDL_DOLLARRECORDU32: u32 = sdl::SDL_EventType::SDL_DOLLARRECORD as u32;
    const SDL_MULTIGESTUREU32: u32 = sdl::SDL_EventType::SDL_MULTIGESTURE as u32;
    const SDL_CLIPBOARDUPDATEU32: u32 = sdl::SDL_EventType::SDL_CLIPBOARDUPDATE as u32;
    const SDL_DROPFILEU32: u32 = sdl::SDL_EventType::SDL_DROPFILE as u32;
    const SDL_DROPTEXTU32: u32 = sdl::SDL_EventType::SDL_DROPTEXT as u32;
    const SDL_DROPBEGINU32: u32 = sdl::SDL_EventType::SDL_DROPBEGIN as u32;
    const SDL_DROPCOMPLETEU32: u32 = sdl::SDL_EventType::SDL_DROPCOMPLETE as u32;
    const SDL_AUDIODEVICEADDEDU32: u32 = sdl::SDL_EventType::SDL_AUDIODEVICEADDED as u32;
    const SDL_AUDIODEVICEREMOVEDU32: u32 = sdl::SDL_EventType::SDL_AUDIODEVICEREMOVED as u32;
    const SDL_SENSORUPDATEU32: u32 = sdl::SDL_EventType::SDL_SENSORUPDATE as u32;
    const SDL_RENDER_TARGETS_RESETU32: u32 = sdl::SDL_EventType::SDL_RENDER_TARGETS_RESET as u32;
    const SDL_RENDER_DEVICE_RESETU32: u32 = sdl::SDL_EventType::SDL_RENDER_DEVICE_RESET as u32;
    const SDL_POLLSENTINELU32: u32 = sdl::SDL_EventType::SDL_POLLSENTINEL as u32;
    const SDL_USEREVENTU32: u32 = sdl::SDL_EventType::SDL_USEREVENT as u32;
    const SDL_LASTEVENTU32: u32 = sdl::SDL_EventType::SDL_LASTEVENT as u32;

    match unsafe { e.type_ } {
        SDL_FIRSTEVENTU32 => sdl::SDL_EventType::SDL_FIRSTEVENT,
        SDL_QUITU32 => sdl::SDL_EventType::SDL_QUIT,
        SDL_APP_TERMINATINGU32 => sdl::SDL_EventType::SDL_APP_TERMINATING,
        SDL_APP_LOWMEMORYU32 => sdl::SDL_EventType::SDL_APP_LOWMEMORY,
        SDL_APP_WILLENTERBACKGROUNDU32 => sdl::SDL_EventType::SDL_APP_WILLENTERBACKGROUND,
        SDL_APP_DIDENTERBACKGROUNDU32 => sdl::SDL_EventType::SDL_APP_DIDENTERBACKGROUND,
        SDL_APP_WILLENTERFOREGROUNDU32 => sdl::SDL_EventType::SDL_APP_WILLENTERFOREGROUND,
        SDL_APP_DIDENTERFOREGROUNDU32 => sdl::SDL_EventType::SDL_APP_DIDENTERFOREGROUND,
        SDL_LOCALECHANGEDU32 => sdl::SDL_EventType::SDL_LOCALECHANGED,
        SDL_DISPLAYEVENTU32 => sdl::SDL_EventType::SDL_DISPLAYEVENT,
        SDL_WINDOWEVENTU32 => sdl::SDL_EventType::SDL_WINDOWEVENT,
        SDL_SYSWMEVENTU32 => sdl::SDL_EventType::SDL_SYSWMEVENT,
        SDL_KEYDOWNU32 => sdl::SDL_EventType::SDL_KEYDOWN,
        SDL_KEYUPU32 => sdl::SDL_EventType::SDL_KEYUP,
        SDL_TEXTEDITINGU32 => sdl::SDL_EventType::SDL_TEXTEDITING,
        SDL_TEXTINPUTU32 => sdl::SDL_EventType::SDL_TEXTINPUT,
        SDL_KEYMAPCHANGEDU32 => sdl::SDL_EventType::SDL_KEYMAPCHANGED,
        SDL_MOUSEMOTIONU32 => sdl::SDL_EventType::SDL_MOUSEMOTION,
        SDL_MOUSEBUTTONDOWNU32 => sdl::SDL_EventType::SDL_MOUSEBUTTONDOWN,
        SDL_MOUSEBUTTONUPU32 => sdl::SDL_EventType::SDL_MOUSEBUTTONUP,
        SDL_MOUSEWHEELU32 => sdl::SDL_EventType::SDL_MOUSEWHEEL,
        SDL_JOYAXISMOTIONU32 => sdl::SDL_EventType::SDL_JOYAXISMOTION,
        SDL_JOYBALLMOTIONU32 => sdl::SDL_EventType::SDL_JOYBALLMOTION,
        SDL_JOYHATMOTIONU32 => sdl::SDL_EventType::SDL_JOYHATMOTION,
        SDL_JOYBUTTONDOWNU32 => sdl::SDL_EventType::SDL_JOYBUTTONDOWN,
        SDL_JOYBUTTONUPU32 => sdl::SDL_EventType::SDL_JOYBUTTONUP,
        SDL_JOYDEVICEADDEDU32 => sdl::SDL_EventType::SDL_JOYDEVICEADDED,
        SDL_JOYDEVICEREMOVEDU32 => sdl::SDL_EventType::SDL_JOYDEVICEREMOVED,
        SDL_CONTROLLERAXISMOTIONU32 => sdl::SDL_EventType::SDL_CONTROLLERAXISMOTION,
        SDL_CONTROLLERBUTTONDOWNU32 => sdl::SDL_EventType::SDL_CONTROLLERBUTTONDOWN,
        SDL_CONTROLLERBUTTONUPU32 => sdl::SDL_EventType::SDL_CONTROLLERBUTTONUP,
        SDL_CONTROLLERDEVICEADDEDU32 => sdl::SDL_EventType::SDL_CONTROLLERDEVICEADDED,
        SDL_CONTROLLERDEVICEREMOVEDU32 => sdl::SDL_EventType::SDL_CONTROLLERDEVICEREMOVED,
        SDL_CONTROLLERDEVICEREMAPPEDU32 => sdl::SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED,
        SDL_CONTROLLERTOUCHPADDOWNU32 => sdl::SDL_EventType::SDL_CONTROLLERTOUCHPADDOWN,
        SDL_CONTROLLERTOUCHPADMOTIONU32 => sdl::SDL_EventType::SDL_CONTROLLERTOUCHPADMOTION,
        SDL_CONTROLLERTOUCHPADUPU32 => sdl::SDL_EventType::SDL_CONTROLLERTOUCHPADUP,
        SDL_CONTROLLERSENSORUPDATEU32 => sdl::SDL_EventType::SDL_CONTROLLERSENSORUPDATE,
        SDL_FINGERDOWNU32 => sdl::SDL_EventType::SDL_FINGERDOWN,
        SDL_FINGERUPU32 => sdl::SDL_EventType::SDL_FINGERUP,
        SDL_FINGERMOTIONU32 => sdl::SDL_EventType::SDL_FINGERMOTION,
        SDL_DOLLARGESTUREU32 => sdl::SDL_EventType::SDL_DOLLARGESTURE,
        SDL_DOLLARRECORDU32 => sdl::SDL_EventType::SDL_DOLLARRECORD,
        SDL_MULTIGESTUREU32 => sdl::SDL_EventType::SDL_MULTIGESTURE,
        SDL_CLIPBOARDUPDATEU32 => sdl::SDL_EventType::SDL_CLIPBOARDUPDATE,
        SDL_DROPFILEU32 => sdl::SDL_EventType::SDL_DROPFILE,
        SDL_DROPTEXTU32 => sdl::SDL_EventType::SDL_DROPTEXT,
        SDL_DROPBEGINU32 => sdl::SDL_EventType::SDL_DROPBEGIN,
        SDL_DROPCOMPLETEU32 => sdl::SDL_EventType::SDL_DROPCOMPLETE,
        SDL_AUDIODEVICEADDEDU32 => sdl::SDL_EventType::SDL_AUDIODEVICEADDED,
        SDL_AUDIODEVICEREMOVEDU32 => sdl::SDL_EventType::SDL_AUDIODEVICEREMOVED,
        SDL_SENSORUPDATEU32 => sdl::SDL_EventType::SDL_SENSORUPDATE,
        SDL_RENDER_TARGETS_RESETU32 => sdl::SDL_EventType::SDL_RENDER_TARGETS_RESET,
        SDL_RENDER_DEVICE_RESETU32 => sdl::SDL_EventType::SDL_RENDER_DEVICE_RESET,
        SDL_POLLSENTINELU32 => sdl::SDL_EventType::SDL_POLLSENTINEL,
        SDL_USEREVENTU32 => sdl::SDL_EventType::SDL_USEREVENT,
        SDL_LASTEVENTU32 => sdl::SDL_EventType::SDL_LASTEVENT,
        _ => unreachable!(),
    }
}
#[inline]
pub fn poll_event() -> Option<Event> {
    let mut event = std::mem::MaybeUninit::uninit();
    unsafe {
        if sdl::SDL_PollEvent(event.as_mut_ptr()) != 0 {
            return Some(event.assume_init());
        }
    }

    return None;
}

pub struct KeyboardState<'a> {
    keyboard_state: &'a [u8],
}

impl<'a> KeyboardState<'a> {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn new() -> Self {
        let mut count = 0;
        unsafe {
            let ptr = sdl::SDL_GetKeyboardState(&mut count);
            return KeyboardState { keyboard_state: std::slice::from_raw_parts(ptr, count as usize) };
        }
    }

    pub fn is_scancode_pressed(&self, scancode: sdl::SDL_Scancode) -> bool {
        return self.keyboard_state[scancode as usize] != 0;
    }


}


#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseButton {
    Unknown = 0,
    Left = sdl::SDL_BUTTON_LEFT as u8,
    Middle = sdl::SDL_BUTTON_MIDDLE as u8,
    Right = sdl::SDL_BUTTON_RIGHT as u8,
    X1 = sdl::SDL_BUTTON_X1 as u8,
    X2 = sdl::SDL_BUTTON_X2 as u8,
}

impl MouseButton {
    #[inline]
    pub fn from_ll(button: u8) -> MouseButton {
        match button as u32 {
            sdl::SDL_BUTTON_LEFT => MouseButton::Left,
            sdl::SDL_BUTTON_MIDDLE => MouseButton::Middle,
            sdl::SDL_BUTTON_RIGHT => MouseButton::Right,
            sdl::SDL_BUTTON_X1 => MouseButton::X1,
            sdl::SDL_BUTTON_X2 => MouseButton::X2,
            _ => MouseButton::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MouseState {
    pub mouse_state: u32,
    pub x: i32,
    pub y: i32,
}

impl MouseState {
    pub fn new() -> MouseState {
        let mut x = 0;
        let mut y = 0;
        let mouse_state: u32 = unsafe { sdl::SDL_GetMouseState(&mut x, &mut y) };
        MouseState {
            mouse_state,
            x: x as i32,
            y: y as i32,
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn from_sdl_state(state: u32) -> MouseState {
        MouseState {
            mouse_state: state,
            x: 0,
            y: 0,
        }
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn to_sdl_state(&self) -> u32 {
        self.mouse_state
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    fn button_mask(&self, button: u32) -> u32 {
        1 << (button - 1)
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn left(&self) -> bool {
        (self.mouse_state & self.button_mask(sdl::SDL_BUTTON_LEFT)) != 0
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn middle(&self) -> bool {
        (self.mouse_state & self.button_mask(sdl::SDL_BUTTON_MIDDLE)) != 0
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn right(&self) -> bool {
        (self.mouse_state & self.button_mask(sdl::SDL_BUTTON_RIGHT)) != 0
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn x1(&self) -> bool {
        (self.mouse_state & self.button_mask(sdl::SDL_BUTTON_X1)) != 0
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn x2(&self) -> bool {
        (self.mouse_state & self.button_mask(sdl::SDL_BUTTON_X2)) != 0
    }
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        let mask = 1 << ((mouse_button as u32) - 1);
        self.mouse_state & mask != 0
    }
}