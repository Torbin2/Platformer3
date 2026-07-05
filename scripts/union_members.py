# https://wiki.libsdl.org/SDL3/SDL_Event#relationships-between-event-types-and-union-members
src = '''SDL_EVENT_AUDIO_DEVICE_ADDED, SDL_EVENT_AUDIO_DEVICE_REMOVED, etc 	SDL_AudioDeviceEvent 	adevice
SDL_EVENT_CAMERA_DEVICE_ADDED, SDL_EVENT_CAMERA_DEVICE_APPROVED, etc 	SDL_CameraDeviceEvent 	cdevice
SDL_EVENT_CLIPBOARD_UPDATE 	SDL_ClipboardEvent 	clipboard
SDL_EVENT_DISPLAY_ADDED, SDL_EVENT_DISPLAY_REMOVED, etc 	SDL_DisplayEvent 	display
SDL_EVENT_GAMEPAD_ADDED, SDL_EVENT_GAMEPAD_REMOVED, or SDL_EVENT_GAMEPAD_REMAPPED, etc 	SDL_GamepadDeviceEvent 	gdevice
SDL_EVENT_GAMEPAD_AXIS_MOTION 	SDL_GamepadAxisEvent 	gaxis
SDL_EVENT_GAMEPAD_BUTTON_DOWN, SDL_EVENT_GAMEPAD_BUTTON_UP 	SDL_GamepadButtonEvent 	gbutton
SDL_EVENT_GAMEPAD_TOUCHPAD_DOWN, SDL_EVENT_GAMEPAD_TOUCHPAD_MOTION, etc 	SDL_GamepadTouchpadEvent 	gtouchpad
SDL_EVENT_GAMEPAD_SENSOR_UPDATE 	SDL_GamepadSensorEvent 	gsensor
SDL_EVENT_DROP_BEGIN, SDL_EVENT_DROP_FILE, SDL_EVENT_DROP_TEXT, etc 	SDL_DropEvent 	drop
SDL_EVENT_FINGER_MOTION, SDL_EVENT_FINGER_DOWN, SDL_EVENT_FINGER_UP 	SDL_TouchFingerEvent 	tfinger
SDL_EVENT_PINCH_BEGIN, SDL_EVENT_PINCH_UPDATE, SDL_EVENT_PINCH_END 	SDL_PinchFingerEvent 	pinch
SDL_EVENT_KEYBOARD_ADDED, SDL_EVENT_KEYBOARD_REMOVED 	SDL_KeyboardDeviceEvent 	kdevice
SDL_EVENT_KEY_DOWN, SDL_EVENT_KEY_UP 	SDL_KeyboardEvent 	key
SDL_EVENT_JOYSTICK_ADDED, SDL_EVENT_JOYSTICK_REMOVED, SDL_EVENT_JOYSTICK_UPDATE_COMPLETE 	SDL_JoyDeviceEvent 	jdevice
SDL_EVENT_JOYSTICK_AXIS_MOTION 	SDL_JoyAxisEvent 	jaxis
SDL_EVENT_JOYSTICK_BALL_MOTION 	SDL_JoyBallEvent 	jball
SDL_EVENT_JOYSTICK_HAT_MOTION 	SDL_JoyHatEvent 	jhat
SDL_EVENT_JOYSTICK_BATTERY_UPDATED 	SDL_JoyBatteryEvent 	jbattery
SDL_EVENT_JOYSTICK_BUTTON_DOWN, SDL_EVENT_JOYSTICK_BUTTON_UP 	SDL_JoyButtonEvent 	jbutton
SDL_EVENT_MOUSE_ADDED, SDL_EVENT_MOUSE_REMOVED 	SDL_MouseDeviceEvent 	mdevice
SDL_EVENT_MOUSE_MOTION 	SDL_MouseMotionEvent 	motion
SDL_EVENT_MOUSE_BUTTON_DOWN, SDL_EVENT_MOUSE_BUTTON_UP 	SDL_MouseButtonEvent 	button
SDL_EVENT_MOUSE_WHEEL 	SDL_MouseWheelEvent 	wheel
SDL_EVENT_PEN_PROXIMITY_IN, SDL_EVENT_PEN_PROXIMITY_OUT 	SDL_PenProximityEvent 	pproximity
SDL_EVENT_PEN_DOWN, SDL_EVENT_PEN_UP 	SDL_PenTouchEvent 	ptouch
SDL_EVENT_PEN_MOTION 	SDL_PenMotionEvent 	pmotion
SDL_EVENT_PEN_BUTTON_DOWN, SDL_EVENT_PEN_BUTTON_UP 	SDL_PenButtonEvent 	pbutton
SDL_EVENT_PEN_AXIS 	SDL_PenAxisEvent 	paxis
SDL_EVENT_QUIT 	SDL_QuitEvent 	quit
SDL_EVENT_SENSOR_UPDATE 	SDL_SensorEvent 	sensor
SDL_EVENT_TEXT_EDITING 	SDL_TextEditingEvent 	edit
SDL_EVENT_TEXT_EDITING_CANDIDATES 	SDL_TextEditingCandidatesEvent 	edit_candidates
SDL_EVENT_TEXT_INPUT 	SDL_TextInputEvent 	text
SDL_EVENT_USER 	SDL_UserEvent 	user
SDL_EVENT_WINDOW_RESIZED, SDL_EVENT_WINDOW_MOVED, etc 	SDL_WindowEvent 	window'''

for line in src.split('\n'):
    items = [item.strip() for item in line.split('\t')]
    event_names, struct_name, _ = items
    event_names = event_names.split(', ')
    struct_name = struct_name.removeprefix('SDL_')
    for event_name in event_names:
        if event_name == 'etc':
            print(f'...: {struct_name!r},')
        else:
            print(f'{event_name!r}: {struct_name!r},')
    print()

