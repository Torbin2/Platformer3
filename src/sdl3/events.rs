// Generated using: python3 scripts/generate_sdl_events.py /usr/include/SDL3/SDL_events.h
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[repr(u32)]
pub enum Event {
    SDL_EVENT_QUIT(QuitEvent) = 0x100,
    SDL_EVENT_TERMINATING(CommonEvent) = 0x101,
    SDL_EVENT_LOW_MEMORY(CommonEvent) = 0x102,
    SDL_EVENT_WILL_ENTER_BACKGROUND(CommonEvent) = 0x103,
    SDL_EVENT_DID_ENTER_BACKGROUND(CommonEvent) = 0x104,
    SDL_EVENT_WILL_ENTER_FOREGROUND(CommonEvent) = 0x105,
    SDL_EVENT_DID_ENTER_FOREGROUND(CommonEvent) = 0x106,
    SDL_EVENT_LOCALE_CHANGED(CommonEvent) = 0x107,
    SDL_EVENT_SYSTEM_THEME_CHANGED(CommonEvent) = 0x108,
    SDL_EVENT_DISPLAY_ORIENTATION(DisplayEvent) = 0x151,
    SDL_EVENT_DISPLAY_ADDED(DisplayEvent) = 0x152,
    SDL_EVENT_DISPLAY_REMOVED(DisplayEvent) = 0x153,
    SDL_EVENT_DISPLAY_MOVED(DisplayEvent) = 0x154,
    SDL_EVENT_DISPLAY_DESKTOP_MODE_CHANGED(DisplayEvent) = 0x155,
    SDL_EVENT_DISPLAY_CURRENT_MODE_CHANGED(DisplayEvent) = 0x156,
    SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED(DisplayEvent) = 0x157,
    SDL_EVENT_DISPLAY_USABLE_BOUNDS_CHANGED(DisplayEvent) = 0x158,
    SDL_EVENT_WINDOW_SHOWN(WindowEvent) = 0x202,
    SDL_EVENT_WINDOW_HIDDEN(WindowEvent) = 0x203,
    SDL_EVENT_WINDOW_EXPOSED(WindowEvent) = 0x204,
    SDL_EVENT_WINDOW_MOVED(WindowEvent) = 0x205,
    SDL_EVENT_WINDOW_RESIZED(WindowEvent) = 0x206,
    SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED(WindowEvent) = 0x207,
    SDL_EVENT_WINDOW_METAL_VIEW_RESIZED(WindowEvent) = 0x208,
    SDL_EVENT_WINDOW_MINIMIZED(WindowEvent) = 0x209,
    SDL_EVENT_WINDOW_MAXIMIZED(WindowEvent) = 0x20a,
    SDL_EVENT_WINDOW_RESTORED(WindowEvent) = 0x20b,
    SDL_EVENT_WINDOW_MOUSE_ENTER(WindowEvent) = 0x20c,
    SDL_EVENT_WINDOW_MOUSE_LEAVE(WindowEvent) = 0x20d,
    SDL_EVENT_WINDOW_FOCUS_GAINED(WindowEvent) = 0x20e,
    SDL_EVENT_WINDOW_FOCUS_LOST(WindowEvent) = 0x20f,
    SDL_EVENT_WINDOW_CLOSE_REQUESTED(WindowEvent) = 0x210,
    SDL_EVENT_WINDOW_HIT_TEST(WindowEvent) = 0x211,
    SDL_EVENT_WINDOW_ICCPROF_CHANGED(WindowEvent) = 0x212,
    SDL_EVENT_WINDOW_DISPLAY_CHANGED(WindowEvent) = 0x213,
    SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED(WindowEvent) = 0x214,
    SDL_EVENT_WINDOW_SAFE_AREA_CHANGED(WindowEvent) = 0x215,
    SDL_EVENT_WINDOW_OCCLUDED(WindowEvent) = 0x216,
    SDL_EVENT_WINDOW_ENTER_FULLSCREEN(WindowEvent) = 0x217,
    SDL_EVENT_WINDOW_LEAVE_FULLSCREEN(WindowEvent) = 0x218,
    SDL_EVENT_WINDOW_DESTROYED(WindowEvent) = 0x219,
    SDL_EVENT_WINDOW_HDR_STATE_CHANGED(WindowEvent) = 0x21a,
    SDL_EVENT_KEY_DOWN(KeyboardEvent) = 0x300,
    SDL_EVENT_KEY_UP(KeyboardEvent) = 0x301,
    SDL_EVENT_TEXT_EDITING(TextEditingEvent) = 0x302,
    SDL_EVENT_TEXT_INPUT(TextInputEvent) = 0x303,
    SDL_EVENT_KEYMAP_CHANGED(CommonEvent) = 0x304,
    SDL_EVENT_KEYBOARD_ADDED(KeyboardDeviceEvent) = 0x305,
    SDL_EVENT_KEYBOARD_REMOVED(KeyboardDeviceEvent) = 0x306,
    SDL_EVENT_TEXT_EDITING_CANDIDATES(TextEditingCandidatesEvent) = 0x307,
    SDL_EVENT_SCREEN_KEYBOARD_SHOWN(CommonEvent) = 0x308,
    SDL_EVENT_SCREEN_KEYBOARD_HIDDEN(CommonEvent) = 0x309,
    SDL_EVENT_MOUSE_MOTION(MouseMotionEvent) = 0x400,
    SDL_EVENT_MOUSE_BUTTON_DOWN(MouseButtonEvent) = 0x401,
    SDL_EVENT_MOUSE_BUTTON_UP(MouseButtonEvent) = 0x402,
    SDL_EVENT_MOUSE_WHEEL(MouseWheelEvent) = 0x403,
    SDL_EVENT_MOUSE_ADDED(MouseDeviceEvent) = 0x404,
    SDL_EVENT_MOUSE_REMOVED(MouseDeviceEvent) = 0x405,
    SDL_EVENT_JOYSTICK_AXIS_MOTION(JoyAxisEvent) = 0x600,
    SDL_EVENT_JOYSTICK_BALL_MOTION(JoyBallEvent) = 0x601,
    SDL_EVENT_JOYSTICK_HAT_MOTION(JoyHatEvent) = 0x602,
    SDL_EVENT_JOYSTICK_BUTTON_DOWN(JoyButtonEvent) = 0x603,
    SDL_EVENT_JOYSTICK_BUTTON_UP(JoyButtonEvent) = 0x604,
    SDL_EVENT_JOYSTICK_ADDED(JoyDeviceEvent) = 0x605,
    SDL_EVENT_JOYSTICK_REMOVED(JoyDeviceEvent) = 0x606,
    SDL_EVENT_JOYSTICK_BATTERY_UPDATED(JoyBatteryEvent) = 0x607,
    SDL_EVENT_JOYSTICK_UPDATE_COMPLETE(JoyDeviceEvent) = 0x608,
    SDL_EVENT_GAMEPAD_AXIS_MOTION(GamepadAxisEvent) = 0x650,
    SDL_EVENT_GAMEPAD_BUTTON_DOWN(GamepadButtonEvent) = 0x651,
    SDL_EVENT_GAMEPAD_BUTTON_UP(GamepadButtonEvent) = 0x652,
    SDL_EVENT_GAMEPAD_ADDED(GamepadDeviceEvent) = 0x653,
    SDL_EVENT_GAMEPAD_REMOVED(GamepadDeviceEvent) = 0x654,
    SDL_EVENT_GAMEPAD_REMAPPED(GamepadDeviceEvent) = 0x655,
    SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN(GamepadTouchpadEvent) = 0x656,
    SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION(GamepadTouchpadEvent) = 0x657,
    SDL_EVENT_GAMEPAD_TOUCHPAD_UP(GamepadTouchpadEvent) = 0x658,
    SDL_EVENT_GAMEPAD_SENSOR_UPDATE(GamepadSensorEvent) = 0x659,
    SDL_EVENT_GAMEPAD_UPDATE_COMPLETE(GamepadDeviceEvent) = 0x65a,
    SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED(GamepadDeviceEvent) = 0x65b,
    SDL_EVENT_FINGER_DOWN(TouchFingerEvent) = 0x700,
    SDL_EVENT_FINGER_UP(TouchFingerEvent) = 0x701,
    SDL_EVENT_FINGER_MOTION(TouchFingerEvent) = 0x702,
    SDL_EVENT_FINGER_CANCELED(CommonEvent) = 0x703,
    SDL_EVENT_PINCH_BEGIN(PinchFingerEvent) = 0x710,
    SDL_EVENT_PINCH_UPDATE(PinchFingerEvent) = 0x711,
    SDL_EVENT_PINCH_END(PinchFingerEvent) = 0x712,
    SDL_EVENT_CLIPBOARD_UPDATE(ClipboardEvent) = 0x900,
    SDL_EVENT_DROP_FILE(DropEvent) = 0x1000,
    SDL_EVENT_DROP_TEXT(DropEvent) = 0x1001,
    SDL_EVENT_DROP_BEGIN(DropEvent) = 0x1002,
    SDL_EVENT_DROP_COMPLETE(DropEvent) = 0x1003,
    SDL_EVENT_DROP_POSITION(DropEvent) = 0x1004,
    SDL_EVENT_AUDIO_DEVICE_ADDED(AudioDeviceEvent) = 0x1100,
    SDL_EVENT_AUDIO_DEVICE_REMOVED(AudioDeviceEvent) = 0x1101,
    SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED(AudioDeviceEvent) = 0x1102,
    SDL_EVENT_SENSOR_UPDATE(SensorEvent) = 0x1200,
    SDL_EVENT_PEN_PROXIMITY_IN(PenProximityEvent) = 0x1300,
    SDL_EVENT_PEN_PROXIMITY_OUT(PenProximityEvent) = 0x1301,
    SDL_EVENT_PEN_DOWN(PenTouchEvent) = 0x1302,
    SDL_EVENT_PEN_UP(PenTouchEvent) = 0x1303,
    SDL_EVENT_PEN_BUTTON_DOWN(PenButtonEvent) = 0x1304,
    SDL_EVENT_PEN_BUTTON_UP(PenButtonEvent) = 0x1305,
    SDL_EVENT_PEN_MOTION(PenMotionEvent) = 0x1306,
    SDL_EVENT_PEN_AXIS(PenAxisEvent) = 0x1307,
    SDL_EVENT_CAMERA_DEVICE_ADDED(CameraDeviceEvent) = 0x1400,
    SDL_EVENT_CAMERA_DEVICE_REMOVED(CameraDeviceEvent) = 0x1401,
    SDL_EVENT_CAMERA_DEVICE_APPROVED(CameraDeviceEvent) = 0x1402,
    SDL_EVENT_CAMERA_DEVICE_DENIED(CameraDeviceEvent) = 0x1403,
    SDL_EVENT_RENDER_TARGETS_RESET(CommonEvent) = 0x2000,
    SDL_EVENT_RENDER_DEVICE_RESET(CommonEvent) = 0x2001,
    SDL_EVENT_RENDER_DEVICE_LOST(CommonEvent) = 0x2002,
}
#[repr(C)]
pub struct CommonEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
}
#[repr(C)]
pub struct DisplayEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub displayID: DisplayID,
    pub data1: i32,
    pub data2: i32,
}
#[repr(C)]
pub struct WindowEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub data1: i32,
    pub data2: i32,
}
#[repr(C)]
pub struct KeyboardDeviceEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: KeyboardID,
}
#[repr(C)]
pub struct KeyboardEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: KeyboardID,
    pub scancode: Scancode,
    pub key: Keycode,
    pub keymod: Keymod,
    pub raw: u16,
    pub down: bool,
    pub repeat: bool,
}
#[repr(C)]
pub struct TextEditingEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub text: *const core::ffi::c_char,
    pub start: i32,
    pub length: i32,
}
#[repr(C)]
pub struct TextEditingCandidatesEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub candidates: *const *const core::ffi::c_char,
    pub num_candidates: i32,
    pub selected_candidate: i32,
    pub horizontal: bool,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
}
#[repr(C)]
pub struct TextInputEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub text: *const core::ffi::c_char,
}
#[repr(C)]
pub struct MouseDeviceEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: MouseID,
}
#[repr(C)]
pub struct MouseMotionEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: MouseID,
    pub state: MouseButtonFlags,
    pub x: f32,
    pub y: f32,
    pub xrel: f32,
    pub yrel: f32,
}
#[repr(C)]
pub struct MouseButtonEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: MouseID,
    pub button: u8,
    pub down: bool,
    pub clicks: u8,
    pub padding: u8,
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
pub struct MouseWheelEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: MouseID,
    pub x: f32,
    pub y: f32,
    pub direction: MouseWheelDirection,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub integer_x: i32,
    pub integer_y: i32,
}
#[repr(C)]
pub struct JoyAxisEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}
#[repr(C)]
pub struct JoyBallEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub ball: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub xrel: i16,
    pub yrel: i16,
}
#[repr(C)]
pub struct JoyHatEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub hat: u8,
    pub value: u8,
    pub padding1: u8,
    pub padding2: u8,
}
#[repr(C)]
pub struct JoyButtonEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub button: u8,
    pub down: bool,
    pub padding1: u8,
    pub padding2: u8,
}
#[repr(C)]
pub struct JoyDeviceEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
}
#[repr(C)]
pub struct JoyBatteryEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub state: PowerState,
    pub percent: core::ffi::c_int,
}
#[repr(C)]
pub struct GamepadAxisEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}
#[repr(C)]
pub struct GamepadButtonEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub button: u8,
    pub down: bool,
    pub padding1: u8,
    pub padding2: u8,
}
#[repr(C)]
pub struct GamepadDeviceEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
}
#[repr(C)]
pub struct GamepadTouchpadEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub touchpad: i32,
    pub finger: i32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}
#[repr(C)]
pub struct GamepadSensorEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: JoystickID,
    pub sensor: i32,
    pub data: [f32; 3],
    pub sensor_timestamp: u64,
}
#[repr(C)]
pub struct AudioDeviceEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: AudioDeviceID,
    pub recording: bool,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
}
#[repr(C)]
pub struct CameraDeviceEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: CameraID,
}
#[repr(C)]
pub struct RenderEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
}
#[repr(C)]
pub struct TouchFingerEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub touchID: TouchID,
    pub fingerID: FingerID,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub pressure: f32,
    pub windowID: WindowID,
}
#[repr(C)]
pub struct PinchFingerEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub scale: f32,
    pub windowID: WindowID,
}
#[repr(C)]
pub struct PenProximityEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: PenID,
}
#[repr(C)]
pub struct PenMotionEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: PenID,
    pub pen_state: PenInputFlags,
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
pub struct PenTouchEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: PenID,
    pub pen_state: PenInputFlags,
    pub x: f32,
    pub y: f32,
    pub eraser: bool,
    pub down: bool,
}
#[repr(C)]
pub struct PenButtonEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: PenID,
    pub pen_state: PenInputFlags,
    pub x: f32,
    pub y: f32,
    pub button: u8,
    pub down: bool,
}
#[repr(C)]
pub struct PenAxisEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub which: PenID,
    pub pen_state: PenInputFlags,
    pub x: f32,
    pub y: f32,
    pub axis: PenAxis,
    pub value: f32,
}
#[repr(C)]
pub struct DropEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub x: f32,
    pub y: f32,
    pub source: *const core::ffi::c_char,
    pub data: *const core::ffi::c_char,
}
#[repr(C)]
pub struct ClipboardEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub owner: bool,
    pub num_mime_types: i32,
    pub mime_types: *const *const core::ffi::c_char,
}
#[repr(C)]
pub struct SensorEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub which: SensorID,
    pub data: [f32; 6],
    pub sensor_timestamp: u64,
}
#[repr(C)]
pub struct QuitEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
}
#[repr(C)]
pub struct UserEvent {
    pub event_type: u32,
    pub reserved: u32,
    pub timestamp: u64,
    pub windowID: WindowID,
    pub code: i32,
    pub data1: *mut core::ffi::c_void,
    pub data2: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct DisplayID(u32);
#[repr(C)]
pub struct WindowID(u32);
#[repr(C)]
pub struct KeyboardID(u32);
#[repr(C)]
pub struct Scancode(u32);
#[repr(C)]
pub struct Keycode(u32);
#[repr(C)]
pub struct Keymod(u16);
#[repr(C)]
pub struct MouseID(u32);
#[repr(C)]
pub struct MouseButtonFlags(u32);
pub enum MouseWheelDirection {}
#[repr(C)]
pub struct JoystickID(u32);
pub enum PowerState {}
#[repr(C)]
pub struct AudioDeviceID(u32);
#[repr(C)]
pub struct CameraID(u32);
#[repr(C)]
pub struct TouchID(u32);
#[repr(C)]
pub struct FingerID(u64);
#[repr(C)]
pub struct PenID(u32);
#[repr(C)]
pub struct PenInputFlags(u32);
pub enum PenAxis {}
#[repr(C)]
pub struct SensorID(u32);
