use sdl2_sys as sdl;

pub fn set(name: &str, value: &str) -> bool {
    let name_last = name.len() - 1;
    assert_eq!(name.chars().nth(name_last).expect("to get the last char"), '\0', "name must be zero terminated");
    let value_last = value.len() - 1;
    assert_eq!(value.chars().nth(value_last).expect("to get the last char"), '\0', "value must be zero terminated");

    unsafe {
        sdl::SDL_SetHint(
            name.as_ptr() as *const _,
            value.as_ptr() as *const _,
        ) == sdl::SDL_bool::SDL_TRUE
    }
}