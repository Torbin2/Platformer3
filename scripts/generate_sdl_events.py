import sys
import io

def preprocess(src: str) -> str:
    out = io.StringIO()

    comment = None
    for i, c in enumerate(src):
        if comment is None:
            if c == '/' and i + 1 < len(src) and src[i+1] == '*':
                comment = 'multi'
            elif c == '/' and i > 0 and src[i-1] == '/':
                comment = 'single'
            elif c == '#':
                comment = 'single'
            else:
                out.write(c)
        elif comment == 'single':
            if c == '\n' and src[i-1] != '\\':
                comment = None
                out.write('\n')
        elif comment == 'multi':
            if c == '/' and src[i - 1] == '*':
                comment = None
        else:
            raise AssertionError(comment)

    return out.getvalue()

def event_to_type_name(event_name: str) -> str:
    conversions = {
            # Generated using scripts/union_members.py, manually filled in.

            'SDL_EVENT_AUDIO_DEVICE_ADDED': 'AudioDeviceEvent',
            'SDL_EVENT_AUDIO_DEVICE_REMOVED': 'AudioDeviceEvent',
            'SDL_EVENT_AUDIO_DEVICE_FORMAT_CHANGED': 'AudioDeviceEvent',

            'SDL_EVENT_CAMERA_DEVICE_ADDED': 'CameraDeviceEvent',
            'SDL_EVENT_CAMERA_DEVICE_REMOVED': 'CameraDeviceEvent',
            'SDL_EVENT_CAMERA_DEVICE_APPROVED': 'CameraDeviceEvent',
            'SDL_EVENT_CAMERA_DEVICE_DENIED': 'CameraDeviceEvent',

            'SDL_EVENT_CLIPBOARD_UPDATE': 'ClipboardEvent',

            'SDL_EVENT_DISPLAY_ORIENTATION': 'DisplayEvent',
            'SDL_EVENT_DISPLAY_ADDED': 'DisplayEvent',
            'SDL_EVENT_DISPLAY_REMOVED': 'DisplayEvent',
            'SDL_EVENT_DISPLAY_MOVED': 'DisplayEvent',
            'SDL_EVENT_DISPLAY_DESKTOP_MODE_CHANGED': 'DisplayEvent',
            'SDL_EVENT_DISPLAY_CURRENT_MODE_CHANGED': 'DisplayEvent',
            'SDL_EVENT_DISPLAY_CONTENT_SCALE_CHANGED': 'DisplayEvent',
            'SDL_EVENT_DISPLAY_USABLE_BOUNDS_CHANGED': 'DisplayEvent',

            'SDL_EVENT_GAMEPAD_ADDED': 'GamepadDeviceEvent',
            'SDL_EVENT_GAMEPAD_REMOVED': 'GamepadDeviceEvent',
            'SDL_EVENT_GAMEPAD_REMAPPED': 'GamepadDeviceEvent',
            'SDL_EVENT_GAMEPAD_UPDATE_COMPLETE': 'GamepadDeviceEvent',
            'SDL_EVENT_GAMEPAD_STEAM_HANDLE_UPDATED': 'GamepadDeviceEvent',

            'SDL_EVENT_GAMEPAD_AXIS_MOTION': 'GamepadAxisEvent',

            'SDL_EVENT_GAMEPAD_BUTTON_DOWN': 'GamepadButtonEvent',
            'SDL_EVENT_GAMEPAD_BUTTON_UP': 'GamepadButtonEvent',

            'SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN': 'GamepadTouchpadEvent',
            'SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION': 'GamepadTouchpadEvent',
            'SDL_EVENT_GAMEPAD_TOUCHPAD_UP': 'GamepadTouchpadEvent',

            'SDL_EVENT_GAMEPAD_SENSOR_UPDATE': 'GamepadSensorEvent',

            'SDL_EVENT_DROP_FILE': 'DropEvent',
            'SDL_EVENT_DROP_TEXT': 'DropEvent',
            'SDL_EVENT_DROP_BEGIN': 'DropEvent',
            'SDL_EVENT_DROP_COMPLETE': 'DropEvent',
            'SDL_EVENT_DROP_POSITION': 'DropEvent',

            'SDL_EVENT_FINGER_MOTION': 'TouchFingerEvent',
            'SDL_EVENT_FINGER_DOWN': 'TouchFingerEvent',
            'SDL_EVENT_FINGER_UP': 'TouchFingerEvent',

            'SDL_EVENT_PINCH_BEGIN': 'PinchFingerEvent',
            'SDL_EVENT_PINCH_UPDATE': 'PinchFingerEvent',
            'SDL_EVENT_PINCH_END': 'PinchFingerEvent',

            'SDL_EVENT_KEYBOARD_ADDED': 'KeyboardDeviceEvent',
            'SDL_EVENT_KEYBOARD_REMOVED': 'KeyboardDeviceEvent',

            'SDL_EVENT_KEY_DOWN': 'KeyboardEvent',
            'SDL_EVENT_KEY_UP': 'KeyboardEvent',

            'SDL_EVENT_JOYSTICK_ADDED': 'JoyDeviceEvent',
            'SDL_EVENT_JOYSTICK_REMOVED': 'JoyDeviceEvent',
            'SDL_EVENT_JOYSTICK_UPDATE_COMPLETE': 'JoyDeviceEvent',

            'SDL_EVENT_JOYSTICK_AXIS_MOTION': 'JoyAxisEvent',

            'SDL_EVENT_JOYSTICK_BALL_MOTION': 'JoyBallEvent',

            'SDL_EVENT_JOYSTICK_HAT_MOTION': 'JoyHatEvent',

            'SDL_EVENT_JOYSTICK_BATTERY_UPDATED': 'JoyBatteryEvent',

            'SDL_EVENT_JOYSTICK_BUTTON_DOWN': 'JoyButtonEvent',
            'SDL_EVENT_JOYSTICK_BUTTON_UP': 'JoyButtonEvent',

            'SDL_EVENT_MOUSE_ADDED': 'MouseDeviceEvent',
            'SDL_EVENT_MOUSE_REMOVED': 'MouseDeviceEvent',

            'SDL_EVENT_MOUSE_MOTION': 'MouseMotionEvent',

            'SDL_EVENT_MOUSE_BUTTON_DOWN': 'MouseButtonEvent',
            'SDL_EVENT_MOUSE_BUTTON_UP': 'MouseButtonEvent',

            'SDL_EVENT_MOUSE_WHEEL': 'MouseWheelEvent',

            'SDL_EVENT_PEN_PROXIMITY_IN': 'PenProximityEvent',
            'SDL_EVENT_PEN_PROXIMITY_OUT': 'PenProximityEvent',

            'SDL_EVENT_PEN_DOWN': 'PenTouchEvent',
            'SDL_EVENT_PEN_UP': 'PenTouchEvent',

            'SDL_EVENT_PEN_MOTION': 'PenMotionEvent',

            'SDL_EVENT_PEN_BUTTON_DOWN': 'PenButtonEvent',
            'SDL_EVENT_PEN_BUTTON_UP': 'PenButtonEvent',

            'SDL_EVENT_PEN_AXIS': 'PenAxisEvent',

            'SDL_EVENT_QUIT': 'QuitEvent',

            'SDL_EVENT_SENSOR_UPDATE': 'SensorEvent',

            'SDL_EVENT_TEXT_EDITING': 'TextEditingEvent',

            'SDL_EVENT_TEXT_EDITING_CANDIDATES': 'TextEditingCandidatesEvent',

            'SDL_EVENT_TEXT_INPUT': 'TextInputEvent',

            'SDL_EVENT_USER': 'UserEvent',

            'SDL_EVENT_WINDOW_SHOWN': 'WindowEvent',
            'SDL_EVENT_WINDOW_HIDDEN': 'WindowEvent',
            'SDL_EVENT_WINDOW_EXPOSED': 'WindowEvent',
            'SDL_EVENT_WINDOW_MOVED': 'WindowEvent',
            'SDL_EVENT_WINDOW_RESIZED': 'WindowEvent',
            'SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED': 'WindowEvent',
            'SDL_EVENT_WINDOW_METAL_VIEW_RESIZED': 'WindowEvent',
            'SDL_EVENT_WINDOW_MINIMIZED': 'WindowEvent',
            'SDL_EVENT_WINDOW_MAXIMIZED': 'WindowEvent',
            'SDL_EVENT_WINDOW_RESTORED': 'WindowEvent',
            'SDL_EVENT_WINDOW_MOUSE_ENTER': 'WindowEvent',
            'SDL_EVENT_WINDOW_MOUSE_LEAVE': 'WindowEvent',
            'SDL_EVENT_WINDOW_FOCUS_GAINED': 'WindowEvent',
            'SDL_EVENT_WINDOW_FOCUS_LOST': 'WindowEvent',
            'SDL_EVENT_WINDOW_CLOSE_REQUESTED': 'WindowEvent',
            'SDL_EVENT_WINDOW_HIT_TEST': 'WindowEvent',
            'SDL_EVENT_WINDOW_ICCPROF_CHANGED': 'WindowEvent',
            'SDL_EVENT_WINDOW_DISPLAY_CHANGED': 'WindowEvent',
            'SDL_EVENT_WINDOW_DISPLAY_SCALE_CHANGED': 'WindowEvent',
            'SDL_EVENT_WINDOW_SAFE_AREA_CHANGED': 'WindowEvent',
            'SDL_EVENT_WINDOW_OCCLUDED': 'WindowEvent',
            'SDL_EVENT_WINDOW_ENTER_FULLSCREEN': 'WindowEvent',
            'SDL_EVENT_WINDOW_LEAVE_FULLSCREEN': 'WindowEvent',
            'SDL_EVENT_WINDOW_DESTROYED': 'WindowEvent',
            'SDL_EVENT_WINDOW_HDR_STATE_CHANGED': 'WindowEvent',
    }
    if event_name in conversions:
        return conversions[event_name]
    else:
        print(f'WARNING: {event_name!r} not in conversions, defaulting to CommonEvent', file=sys.stderr)
        return 'CommonEvent'

if __name__ == '__main__':
    if len(sys.argv) != 2:
        raise ValueError(f"Expected 1 argument: {sys.argv[0]} PATH_TO_SDL_EVENTS.h")
    sdl_event_h_filename = sys.argv[1]
    with open(sdl_event_h_filename) as file:
        sdl_event_h = file.read()

    p = preprocess(sdl_event_h)
    # print(p)
    # print('-' * 80)

    print(f'// Generated using: python3 {" ".join(sys.argv)}')
    print('#![allow(non_camel_case_types)]')
    print('#![allow(non_snake_case)]')

    lines = p.split('\n')

    def peek() -> str:
        return lines[0]
    def get() -> str:
        return lines.pop(0)
    def try_get() -> str | None:
        if lines:
            return lines.pop(0)
        else:
            return None

    def skip_empty() -> None:
        while peek().strip() == '':
            get()

    skip_empty()
    assert get().strip() == 'extern "C" {'   # }
    skip_empty()
    assert get().strip() == 'typedef enum SDL_EventType'
    assert get().strip() == '{'  # }

    event_enum: list[tuple[str, int]] = []
    enum_value = -1
    while peek().strip() != '} SDL_EventType;':
        line = get().strip()
        if line == '':
            continue

        name, _, value = line.removesuffix(',').partition('=')
        name = name.strip()
        value = value.strip()
        if name.endswith('FIRST') or name.endswith('LAST') or name == 'SDL_EVENT_ENUM_PADDING':
            continue
        if (
            name == "SDL_EVENT_PRIVATE0"
            or name == "SDL_EVENT_PRIVATE1"
            or name == "SDL_EVENT_PRIVATE2"
            or name == "SDL_EVENT_PRIVATE3"
            or name == "SDL_EVENT_POLL_SENTINEL"
            or name == "SDL_EVENT_USER"
        ):
            continue

        enum_value += 1
        if value:
            enum_value = int(value, base=16)
        event_enum.append((name, enum_value))
    assert get().strip() == '} SDL_EventType;'

    last_value = -1
    print('#[repr(u32)]')
    print('pub enum Event {')
    for name, value in event_enum:
        if value != last_value:
            v = f' = 0x{value:x}'
        else:
            v = ''
        print('    ' + name + '(' + event_to_type_name(name) + ')' + v + ',')
        last_value = value
    print('}')

    # -----
    while (line := try_get()) is not None:
        line = line.strip()
        if line.startswith('extern') or line.startswith('SDL_COMPILE_TIME_ASSERT'):
            continue
        if line.startswith('typedef union'):
            lines.insert(0, line)
            break

        if line.startswith('typedef'):
            struct_name = line.removeprefix('typedef struct ').removeprefix('SDL_')
            assert get().strip() == '{'
            print('#[repr(C)]')
            print(f'pub struct {struct_name} {{')
            while not (line := get()).startswith('}'):
                line = line.strip()
                type_name, _, member = line.rpartition(' ')
                type_name = type_name.strip()
                member = member.removesuffix(';').strip()
                while member.startswith('*'):
                    member = member[1:]
                    type_name += '*'
                c_to_rust_typename = {
                    'Uint8': 'u8',
                    'Uint16': 'u16',
                    'Uint32': 'u32',
                    'Uint64': 'u64',
                    'Sint16': 'i16',
                    'Sint32': 'i32',
                    'Sint64': 'i64',
                    'int': 'core::ffi::c_int',
                    'bool': 'bool',
                    'float': 'f32',

                    'const char*': '*const core::ffi::c_char',
                    'const char * const*': '*const *const core::ffi::c_char',
                    'const char**': '*const *const core::ffi::c_char',
                    'void*': '*mut core::ffi::c_void',

                    'SDL_EventType': 'u32',
                    'SDL_DisplayID': 'DisplayID',    # u32
                    'SDL_WindowID': 'WindowID',      # u32
                    'SDL_KeyboardID': 'KeyboardID',  # u32
                    'SDL_Scancode': 'Scancode',      # Tested to be u32
                    'SDL_Keycode': 'Keycode',        # Tested to be u32
                    'SDL_Keymod': 'Keymod',          # u16
                    'SDL_MouseID': 'MouseID',        # u32
                    'SDL_MouseButtonFlags': 'MouseButtonFlags',  # u32
                    'SDL_MouseWheelDirection': 'MouseWheelDirection',  # Enum
                    'SDL_JoystickID': 'JoystickID',  # u32
                    'SDL_PowerState': 'PowerState',   # Enum
                    'SDL_AudioDeviceID': 'AudioDeviceID',  # u32
                    'SDL_CameraID': 'CameraID',      # u32
                    'SDL_TouchID': 'TouchID',        # u32
                    'SDL_FingerID': 'FingerID',      # u64
                    'SDL_PenID': 'PenID',            # u32
                    'SDL_PenInputFlags': 'PenInputFlags',  # u32
                    'SDL_PenAxis': 'PenAxis',        # Enum
                    'SDL_SensorID': 'SensorID',      # u32
                }
                type_name = c_to_rust_typename[type_name]
                if member == 'type':
                    member = 'event_type'
                elif member == 'mod':
                    member = 'keymod'
                elif member.endswith(']'):  # TODO: Proper parsing of arrays
                    member, _, l = member.partition('[')
                    l = l.removesuffix(']')
                    type_name = f'[{type_name}; {l}]'
                print(f'    pub {member}: {type_name},')
                assert '[' not in member, (member, type_name, line)
                assert '*' not in member, (member, type_name, line)
            print('}')

    typedefs = {
        'DisplayID': 'u32',
        'WindowID': 'u32',
        'KeyboardID': 'u32',
        'Scancode': 'u32',
        'Keycode': 'u32',
        'Keymod': 'u16',
        'MouseID': 'u32',
        'MouseButtonFlags': 'u32',
        'MouseWheelDirection': 'Enum',
        'JoystickID': 'u32',
        'PowerState': 'Enum',
        'AudioDeviceID': 'u32',
        'CameraID': 'u32',
        'TouchID': 'u32',
        'FingerID': 'u64',
        'PenID': 'u32',
        'PenInputFlags': 'u32',
        'PenAxis': 'Enum',
        'SensorID': 'u32',
    }
    for name, t in typedefs.items():
        if t == 'Enum':
            print(f'pub enum {name} {{}}')
        else:
            print('#[repr(C)]')
            print(f'pub struct {name}({t});')

