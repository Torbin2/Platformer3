// Generated using: python3 scripts/generate_sdl_events.py /usr/include/SDL3/SDL_events.h
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(u32)]
pub enum Event {
    SDL_EVENT_QUIT/*QuitEvent*/ { reserved: u32, timestamp: u64 } = 0x100,
    SDL_EVENT_TERMINATING/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_LOW_MEMORY/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_WILL_ENTER_BACKGROUND/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_DID_ENTER_BACKGROUND/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_WILL_ENTER_FOREGROUND/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_DID_ENTER_FOREGROUND/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_LOCALE_CHANGED/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_SYSTEM_THEME_CHANGED/*CommonEvent*/ { reserved: u32, timestamp: u64 },

    SDL_EVENT_DISPLAY_ORIENTATION/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 } = 0x151,
    SDL_EVENT_DISPLAY_ADDED/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 },
    SDL_EVENT_DISPLAY_REMOVED/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 },
    SDL_EVENT_DISPLAY_MOVED/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 },
    SDL_EVENT_DISPLAY_DESKTOP_MODE_CHANGED/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 },
    SDL_EVENT_DISPLAY_CURRENT_MODE_CHANGED/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 },
    SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 },
    SDL_EVENT_DISPLAY_USABLE_BOUNDS_CHANGED/*DisplayEvent*/ { reserved: u32, timestamp: u64, displayID: DisplayID, data1: i32, data2: i32 },

    SDL_EVENT_WINDOW_SHOWN/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 } = 0x202,
    SDL_EVENT_WINDOW_HIDDEN/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_EXPOSED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_MOVED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_RESIZED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_METAL_VIEW_RESIZED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_MINIMIZED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_MAXIMIZED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_RESTORED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_MOUSE_ENTER/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_MOUSE_LEAVE/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_FOCUS_GAINED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_FOCUS_LOST/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_CLOSE_REQUESTED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_HIT_TEST/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_ICCPROF_CHANGED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_DISPLAY_CHANGED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_SAFE_AREA_CHANGED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_OCCLUDED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_ENTER_FULLSCREEN/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_LEAVE_FULLSCREEN/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_DESTROYED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },
    SDL_EVENT_WINDOW_HDR_STATE_CHANGED/*WindowEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, data1: i32, data2: i32 },

    SDL_EVENT_KEY_DOWN/*KeyboardEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: KeyboardID, scancode: Scancode, key: Keycode, keymod: Keymod, raw: u16, down: bool, repeat: bool } = 0x300,
    SDL_EVENT_KEY_UP/*KeyboardEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: KeyboardID, scancode: Scancode, key: Keycode, keymod: Keymod, raw: u16, down: bool, repeat: bool },
    SDL_EVENT_TEXT_EDITING/*TextEditingEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, text: *const core::ffi::c_char, start: i32, length: i32 },
    SDL_EVENT_TEXT_INPUT/*TextInputEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, text: *const core::ffi::c_char },
    SDL_EVENT_KEYMAP_CHANGED/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_KEYBOARD_ADDED/*KeyboardDeviceEvent*/ { reserved: u32, timestamp: u64, which: KeyboardID },
    SDL_EVENT_KEYBOARD_REMOVED/*KeyboardDeviceEvent*/ { reserved: u32, timestamp: u64, which: KeyboardID },
    SDL_EVENT_TEXT_EDITING_CANDIDATES/*TextEditingCandidatesEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, candidates: *const *const core::ffi::c_char, num_candidates: i32, selected_candidate: i32, horizontal: bool, padding1: u8, padding2: u8, padding3: u8 },
    SDL_EVENT_SCREEN_KEYBOARD_SHOWN/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_SCREEN_KEYBOARD_HIDDEN/*CommonEvent*/ { reserved: u32, timestamp: u64 },

    SDL_EVENT_MOUSE_MOTION/*MouseMotionEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: MouseID, state: MouseButtonFlags, x: f32, y: f32, xrel: f32, yrel: f32 } = 0x400,
    SDL_EVENT_MOUSE_BUTTON_DOWN/*MouseButtonEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: MouseID, button: u8, down: bool, clicks: u8, padding: u8, x: f32, y: f32 },
    SDL_EVENT_MOUSE_BUTTON_UP/*MouseButtonEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: MouseID, button: u8, down: bool, clicks: u8, padding: u8, x: f32, y: f32 },
    SDL_EVENT_MOUSE_WHEEL/*MouseWheelEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: MouseID, x: f32, y: f32, direction: MouseWheelDirection, mouse_x: f32, mouse_y: f32, integer_x: i32, integer_y: i32 },
    SDL_EVENT_MOUSE_ADDED/*MouseDeviceEvent*/ { reserved: u32, timestamp: u64, which: MouseID },
    SDL_EVENT_MOUSE_REMOVED/*MouseDeviceEvent*/ { reserved: u32, timestamp: u64, which: MouseID },

    SDL_EVENT_JOYSTICK_AXIS_MOTION/*JoyAxisEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, axis: u8, padding1: u8, padding2: u8, padding3: u8, value: i16, padding4: u16 } = 0x600,
    SDL_EVENT_JOYSTICK_BALL_MOTION/*JoyBallEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, ball: u8, padding1: u8, padding2: u8, padding3: u8, xrel: i16, yrel: i16 },
    SDL_EVENT_JOYSTICK_HAT_MOTION/*JoyHatEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, hat: u8, value: u8, padding1: u8, padding2: u8 },
    SDL_EVENT_JOYSTICK_BUTTON_DOWN/*JoyButtonEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, button: u8, down: bool, padding1: u8, padding2: u8 },
    SDL_EVENT_JOYSTICK_BUTTON_UP/*JoyButtonEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, button: u8, down: bool, padding1: u8, padding2: u8 },
    SDL_EVENT_JOYSTICK_ADDED/*JoyDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },
    SDL_EVENT_JOYSTICK_REMOVED/*JoyDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },
    SDL_EVENT_JOYSTICK_BATTERY_UPDATED/*JoyBatteryEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, state: PowerState, percent: core::ffi::c_int },
    SDL_EVENT_JOYSTICK_UPDATE_COMPLETE/*JoyDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },

    SDL_EVENT_GAMEPAD_AXIS_MOTION/*GamepadAxisEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, axis: u8, padding1: u8, padding2: u8, padding3: u8, value: i16, padding4: u16 } = 0x650,
    SDL_EVENT_GAMEPAD_BUTTON_DOWN/*GamepadButtonEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, button: u8, down: bool, padding1: u8, padding2: u8 },
    SDL_EVENT_GAMEPAD_BUTTON_UP/*GamepadButtonEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, button: u8, down: bool, padding1: u8, padding2: u8 },
    SDL_EVENT_GAMEPAD_ADDED/*GamepadDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },
    SDL_EVENT_GAMEPAD_REMOVED/*GamepadDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },
    SDL_EVENT_GAMEPAD_REMAPPED/*GamepadDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },
    SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN/*GamepadTouchpadEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, touchpad: i32, finger: i32, x: f32, y: f32, pressure: f32 },
    SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION/*GamepadTouchpadEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, touchpad: i32, finger: i32, x: f32, y: f32, pressure: f32 },
    SDL_EVENT_GAMEPAD_TOUCHPAD_UP/*GamepadTouchpadEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, touchpad: i32, finger: i32, x: f32, y: f32, pressure: f32 },
    SDL_EVENT_GAMEPAD_SENSOR_UPDATE/*GamepadSensorEvent*/ { reserved: u32, timestamp: u64, which: JoystickID, sensor: i32, data: [f32; 3], sensor_timestamp: u64 },
    SDL_EVENT_GAMEPAD_UPDATE_COMPLETE/*GamepadDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },
    SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED/*GamepadDeviceEvent*/ { reserved: u32, timestamp: u64, which: JoystickID },

    SDL_EVENT_FINGER_DOWN/*TouchFingerEvent*/ { reserved: u32, timestamp: u64, touchID: TouchID, fingerID: FingerID, x: f32, y: f32, dx: f32, dy: f32, pressure: f32, windowID: WindowID } = 0x700,
    SDL_EVENT_FINGER_UP/*TouchFingerEvent*/ { reserved: u32, timestamp: u64, touchID: TouchID, fingerID: FingerID, x: f32, y: f32, dx: f32, dy: f32, pressure: f32, windowID: WindowID },
    SDL_EVENT_FINGER_MOTION/*TouchFingerEvent*/ { reserved: u32, timestamp: u64, touchID: TouchID, fingerID: FingerID, x: f32, y: f32, dx: f32, dy: f32, pressure: f32, windowID: WindowID },
    SDL_EVENT_FINGER_CANCELED/*CommonEvent*/ { reserved: u32, timestamp: u64 },

    SDL_EVENT_PINCH_BEGIN/*PinchFingerEvent*/ { reserved: u32, timestamp: u64, scale: f32, windowID: WindowID } = 0x710,
    SDL_EVENT_PINCH_UPDATE/*PinchFingerEvent*/ { reserved: u32, timestamp: u64, scale: f32, windowID: WindowID },
    SDL_EVENT_PINCH_END/*PinchFingerEvent*/ { reserved: u32, timestamp: u64, scale: f32, windowID: WindowID },

    SDL_EVENT_CLIPBOARD_UPDATE/*ClipboardEvent*/ { reserved: u32, timestamp: u64, owner: bool, num_mime_types: i32, mime_types: *const *const core::ffi::c_char } = 0x900,

    SDL_EVENT_DROP_FILE/*DropEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, x: f32, y: f32, source: *const core::ffi::c_char, data: *const core::ffi::c_char } = 0x1000,
    SDL_EVENT_DROP_TEXT/*DropEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, x: f32, y: f32, source: *const core::ffi::c_char, data: *const core::ffi::c_char },
    SDL_EVENT_DROP_BEGIN/*DropEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, x: f32, y: f32, source: *const core::ffi::c_char, data: *const core::ffi::c_char },
    SDL_EVENT_DROP_COMPLETE/*DropEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, x: f32, y: f32, source: *const core::ffi::c_char, data: *const core::ffi::c_char },
    SDL_EVENT_DROP_POSITION/*DropEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, x: f32, y: f32, source: *const core::ffi::c_char, data: *const core::ffi::c_char },

    SDL_EVENT_AUDIO_DEVICE_ADDED/*AudioDeviceEvent*/ { reserved: u32, timestamp: u64, which: AudioDeviceID, recording: bool, padding1: u8, padding2: u8, padding3: u8 } = 0x1100,
    SDL_EVENT_AUDIO_DEVICE_REMOVED/*AudioDeviceEvent*/ { reserved: u32, timestamp: u64, which: AudioDeviceID, recording: bool, padding1: u8, padding2: u8, padding3: u8 },
    SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED/*AudioDeviceEvent*/ { reserved: u32, timestamp: u64, which: AudioDeviceID, recording: bool, padding1: u8, padding2: u8, padding3: u8 },

    SDL_EVENT_SENSOR_UPDATE/*SensorEvent*/ { reserved: u32, timestamp: u64, which: SensorID, data: [f32; 6], sensor_timestamp: u64 } = 0x1200,

    SDL_EVENT_PEN_PROXIMITY_IN/*PenProximityEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, device_type: PenDeviceType } = 0x1300,
    SDL_EVENT_PEN_PROXIMITY_OUT/*PenProximityEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, device_type: PenDeviceType },
    SDL_EVENT_PEN_DOWN/*PenTouchEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, x: f32, y: f32, eraser: bool, down: bool, device_type: PenDeviceType },
    SDL_EVENT_PEN_UP/*PenTouchEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, x: f32, y: f32, eraser: bool, down: bool, device_type: PenDeviceType },
    SDL_EVENT_PEN_BUTTON_DOWN/*PenButtonEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, x: f32, y: f32, button: u8, down: bool, device_type: PenDeviceType },
    SDL_EVENT_PEN_BUTTON_UP/*PenButtonEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, x: f32, y: f32, button: u8, down: bool, device_type: PenDeviceType },
    SDL_EVENT_PEN_MOTION/*PenMotionEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, x: f32, y: f32, device_type: PenDeviceType },
    SDL_EVENT_PEN_AXIS/*PenAxisEvent*/ { reserved: u32, timestamp: u64, windowID: WindowID, which: PenID, pen_state: PenInputFlags, x: f32, y: f32, axis: PenAxis, value: f32, device_type: PenDeviceType },

    SDL_EVENT_CAMERA_DEVICE_ADDED/*CameraDeviceEvent*/ { reserved: u32, timestamp: u64, which: CameraID } = 0x1400,
    SDL_EVENT_CAMERA_DEVICE_REMOVED/*CameraDeviceEvent*/ { reserved: u32, timestamp: u64, which: CameraID },
    SDL_EVENT_CAMERA_DEVICE_APPROVED/*CameraDeviceEvent*/ { reserved: u32, timestamp: u64, which: CameraID },
    SDL_EVENT_CAMERA_DEVICE_DENIED/*CameraDeviceEvent*/ { reserved: u32, timestamp: u64, which: CameraID },

    SDL_EVENT_RENDER_TARGETS_RESET/*CommonEvent*/ { reserved: u32, timestamp: u64 } = 0x2000,
    SDL_EVENT_RENDER_DEVICE_RESET/*CommonEvent*/ { reserved: u32, timestamp: u64 },
    SDL_EVENT_RENDER_DEVICE_LOST/*CommonEvent*/ { reserved: u32, timestamp: u64 },
}

#[repr(transparent)]
pub struct DisplayID(pub u32);
#[repr(transparent)]
pub struct WindowID(pub u32);
#[repr(transparent)]
pub struct KeyboardID(pub u32);
#[repr(transparent)]
pub struct Scancode(pub u32);
#[repr(transparent)]
pub struct Keycode(pub u32);
#[repr(transparent)]
pub struct Keymod(pub u16);
#[repr(transparent)]
pub struct MouseID(pub u32);
#[repr(transparent)]
pub struct MouseButtonFlags(pub u32);
#[repr(transparent)]
pub struct MouseWheelDirection(pub u32);
#[repr(transparent)]
pub struct JoystickID(pub u32);
#[repr(transparent)]
pub struct PowerState(pub u32);
#[repr(transparent)]
pub struct AudioDeviceID(pub u32);
#[repr(transparent)]
pub struct CameraID(pub u32);
#[repr(transparent)]
pub struct TouchID(pub u32);
#[repr(transparent)]
pub struct FingerID(pub u64);
#[repr(transparent)]
pub struct PenID(pub u32);
#[repr(transparent)]
pub struct PenInputFlags(pub u32);
#[repr(transparent)]
pub struct PenAxis(pub u32);
#[repr(transparent)]
pub struct SensorID(pub u32);
#[repr(transparent)]
pub struct PenDeviceType(pub u32);
