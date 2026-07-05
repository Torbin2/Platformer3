#![allow(unused)]

use crate::gl;

mod _sdl3 {
    pub type Bool = core::ffi::c_int;

    #[link(name="SDL3")]
    extern "C" {
        pub fn SDL_Init(flags: u32) -> Bool;

        pub fn SDL_CreateWindow(title: *const core::ffi::c_char, width: core::ffi::c_int, height: core::ffi::c_int, flags: u64) -> *mut core::ffi::c_void;
        pub fn SDL_DestroyWindow(window: *mut core::ffi::c_void) -> ();
        pub fn SDL_GetWindowSizeInPixels(window: *mut core::ffi::c_void, width: *mut core::ffi::c_int, height: *mut core::ffi::c_int) -> Bool;

        pub fn SDL_GL_SetAttribute(attr: core::ffi::c_uint, value: core::ffi::c_int) -> Bool;
        pub fn SDL_GL_CreateContext(window: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        pub fn SDL_GL_DestroyContext(context: *mut core::ffi::c_void) -> Bool;
        pub fn SDL_GL_SetSwapInterval(interval: core::ffi::c_int) -> Bool;
        pub fn SDL_GL_GetProcAddress(proc_name: *const core::ffi::c_char) -> *const core::ffi::c_void;
        pub fn SDL_GL_SwapWindow(window: *mut core::ffi::c_void) -> Bool;

        pub fn SDL_GetError() -> *const core::ffi::c_char;
    }
}

fn get_error() -> String {
    unsafe { std::ffi::CStr::from_ptr(_sdl3::SDL_GetError()) }
        .to_str()
        .expect("SDL returned invalid utf-8 from SDL_GetError()")
        .to_owned()
}

fn sdl_bool_to_result(res: _sdl3::Bool) -> Result<(), String> {
    if res != 0 {
        Ok(())
    } else {
        Err(get_error())
    }
}

pub const INIT_AUDIO   : u32 = 0x00000010; /**< `SDL_INIT_AUDIO` implies `SDL_INIT_EVENTS` */
pub const INIT_VIDEO   : u32 = 0x00000020; /**< `SDL_INIT_VIDEO` implies `SDL_INIT_EVENTS`, should be initialized on the main thread */
pub const INIT_JOYSTICK: u32 = 0x00000200; /**< `SDL_INIT_JOYSTICK` implies `SDL_INIT_EVENTS` */
pub const INIT_HAPTIC  : u32 = 0x00001000;
pub const INIT_GAMEPAD : u32 = 0x00002000; /**< `SDL_INIT_GAMEPAD` implies `SDL_INIT_JOYSTICK` */
pub const INIT_EVENTS  : u32 = 0x00004000;
pub const INIT_SENSOR  : u32 = 0x00008000; /**< `SDL_INIT_SENSOR` implies `SDL_INIT_EVENTS` */
pub const INIT_CAMERA  : u32 = 0x00010000; /**< `SDL_INIT_CAMERA` implies `SDL_INIT_EVENTS` */

pub fn init(flags: u32) -> Result<(), String> {
    sdl_bool_to_result(unsafe { _sdl3::SDL_Init(flags) } )
}

pub const GL_CONTEXT_MAJOR_VERSION: u32 = 17;
pub const GL_CONTEXT_MINOR_VERSION: u32 = 18;
pub const GL_CONTEXT_PROFILE_MASK : u32 = 20;

pub const GL_CONTEXT_PROFILE_CORE         : i32 = 1;
pub const GL_CONTEXT_PROFILE_COMPATIBILITY: i32 = 2;

pub fn gl_set_attribute(attr: u32, value: i32) -> Result<(), String> {
    sdl_bool_to_result(unsafe { _sdl3::SDL_GL_SetAttribute(attr, value) } )
}

// pub fn SDL_GL_SetSwapInterval(interval: core::ffi::c_int) -> Bool;
pub fn gl_set_swap_interval(interval: i32) -> Result<(), String> {
    sdl_bool_to_result(unsafe {
        _sdl3::SDL_GL_SetSwapInterval(interval as core::ffi::c_int)
    })
}

pub const WINDOW_FULLSCREEN           : u64 = 0x0000000000000001;    /**< window is in fullscreen mode */
pub const WINDOW_OPENGL               : u64 = 0x0000000000000002;    /**< window usable with OpenGL context */
pub const WINDOW_OCCLUDED             : u64 = 0x0000000000000004;    /**< window is occluded */
pub const WINDOW_HIDDEN               : u64 = 0x0000000000000008;    /**< window is neither mapped onto the desktop nor shown in the taskbar/dock/window list; SDL_ShowWindow() is required for it to become visible */
pub const WINDOW_BORDERLESS           : u64 = 0x0000000000000010;    /**< no window decoration */
pub const WINDOW_RESIZABLE            : u64 = 0x0000000000000020;    /**< window can be resized */
pub const WINDOW_MINIMIZED            : u64 = 0x0000000000000040;    /**< window is minimized */
pub const WINDOW_MAXIMIZED            : u64 = 0x0000000000000080;    /**< window is maximized */
pub const WINDOW_MOUSE_GRABBED        : u64 = 0x0000000000000100;    /**< window has grabbed mouse input */
pub const WINDOW_INPUT_FOCUS          : u64 = 0x0000000000000200;    /**< window has input focus */
pub const WINDOW_MOUSE_FOCUS          : u64 = 0x0000000000000400;    /**< window has mouse focus */
pub const WINDOW_EXTERNAL             : u64 = 0x0000000000000800;    /**< window not created by SDL */
pub const WINDOW_MODAL                : u64 = 0x0000000000001000;    /**< window is modal */
pub const WINDOW_HIGH_PIXEL_DENSITY   : u64 = 0x0000000000002000;    /**< window uses high pixel density back buffer if possible */
pub const WINDOW_MOUSE_CAPTURE        : u64 = 0x0000000000004000;    /**< window has mouse captured (unrelated to MOUSE_GRABBED) */
pub const WINDOW_MOUSE_RELATIVE_MODE  : u64 = 0x0000000000008000;    /**< window has relative mode enabled */
pub const WINDOW_ALWAYS_ON_TOP        : u64 = 0x0000000000010000;    /**< window should always be above others */
pub const WINDOW_UTILITY              : u64 = 0x0000000000020000;    /**< window should be treated as a utility window, not showing in the task bar and window list */
pub const WINDOW_TOOLTIP              : u64 = 0x0000000000040000;    /**< window should be treated as a tooltip and does not get mouse or keyboard focus, requires a parent window */
pub const WINDOW_POPUP_MENU           : u64 = 0x0000000000080000;    /**< window should be treated as a popup menu, requires a parent window */
pub const WINDOW_KEYBOARD_GRABBED     : u64 = 0x0000000000100000;    /**< window has grabbed keyboard input */
pub const WINDOW_FILL_DOCUMENT        : u64 = 0x0000000000200000;    /**< window is in fill-document mode (Emscripten only), since SDL 3.4.0 */
pub const WINDOW_VULKAN               : u64 = 0x0000000010000000;    /**< window usable for Vulkan surface */
pub const WINDOW_METAL                : u64 = 0x0000000020000000;    /**< window usable for Metal view */
pub const WINDOW_TRANSPARENT          : u64 = 0x0000000040000000;    /**< window with transparent buffer */
pub const WINDOW_NOT_FOCUSABLE        : u64 = 0x0000000080000000;    /**< window should not be focusable */

pub struct Window {
    window: *mut core::ffi::c_void,
}
impl Drop for Window {
    fn drop(&mut self) -> () {
        unsafe {
            _sdl3::SDL_DestroyWindow(self.window);
        }
    }
}

pub struct GLContext<'window> {
    context: *mut core::ffi::c_void,

    phantom: std::marker::PhantomData<&'window core::ffi::c_void>,
}

impl Drop for GLContext<'_> {
    fn drop(&mut self) -> () {
        unsafe {
            _sdl3::SDL_GL_DestroyContext(self.context);
        }
    }
}

impl<'a> Window {
    pub fn create(title: &core::ffi::CStr, width: i32, height: i32, flags: u64) -> Result<Self, String> {
        let window = unsafe { _sdl3::SDL_CreateWindow(title.as_ptr(), width, height, flags) };
        if window.is_null() {
            Err(get_error())
        } else {
            Ok(Window {
                window
            })
        }
    }

    pub fn gl_create_context(&mut self) -> Result<GLContext<'a>, String> {
        let out = unsafe { _sdl3::SDL_GL_CreateContext(self.window) };
        if out.is_null() {
            return Err(get_error());
        }

        gl::load(|s| unsafe {
            _sdl3::SDL_GL_GetProcAddress(
                std::ffi::CString::new(s)
                .expect("GL function name is not valid for a C string")
                .as_ptr()
            )
        });

        Ok(GLContext {
            context: out,
            phantom: std::marker::PhantomData
        })
    }

    pub fn get_size_in_pixels(&mut self) -> (i32, i32) {
        // pub fn SDL_GetWindowSizeInPixels(window: *mut core::ffi::c_void, width: *mut core::ffi::c_int, height: *mut core::ffi::c_int) -> Bool;
        let mut width: core::ffi::c_int = 0;
        let mut height: core::ffi::c_int = 0;
        unsafe {
            _sdl3::SDL_GetWindowSizeInPixels(self.window, &mut width, &mut height);
        }
        return (width, height);
    }

    pub fn swap(&mut self) -> Result<(), String> {
        sdl_bool_to_result(unsafe { _sdl3::SDL_GL_SwapWindow(self.window) } )
    }
}

