// Extracted from glfw/include/GLFW/glfw3.h

/*************************************************************************
 * GLFW API tokens
 *************************************************************************/

use std::ffi::c_int;

/*************************************************************************
 * GLFW API tokens
 *************************************************************************/

/** @name GLFW version macros
*  @{
*/

/** @brief The major version number of the GLFW header.
*
*  The major version number of the GLFW header.  This is incremented when the
*  API is changed in non-compatible ways.
*  @ingroup init
 */

pub const VERSION_MAJOR: c_int = 3;
/** @brief The minor version number of the GLFW header.
*
*  The minor version number of the GLFW header.  This is incremented when
*  features are added to the API but it remains backward-compatible.
*  @ingroup init
 */
pub const VERSION_MINOR: c_int = 4;
/** @brief The revision number of the GLFW header.
*
*  The revision number of the GLFW header.  This is incremented when a bug fix
*  release is made that does not contain any API changes.
*  @ingroup init
 */
pub const VERSION_REVISION: c_int = 0;
/** @} */

/** @brief One.
*
*  This is only semantic sugar for the number 1.  You can instead use `1` or
*  `true` or `_True` or `GL_TRUE` or `VK_TRUE` or anything else that is equal
*  to one.
*
*  @ingroup init
 */
pub const TRUE: c_int = 1;
/** @brief Zero.
*
*  This is only semantic sugar for the number 0.  You can instead use `0` or
*  `false` or `_False` or `GL_FALSE` or `VK_FALSE` or anything else that is
*  equal to zero.
*
*  @ingroup init
 */
pub const FALSE: c_int = 0;

/** @name Key and button actions
*  @{ */
/** @brief The key or mouse button was released.
*
*  The key or mouse button was released.
*
*  @ingroup input
 */
pub const RELEASE: c_int = 0;
/** @brief The key or mouse button was pressed.
*
*  The key or mouse button was pressed.
*
*  @ingroup input
 */
pub const PRESS: c_int = 1;
/** @brief The key was held down until it repeated.
*
*  The key was held down until it repeated.
*
*  @ingroup input
 */
pub const REPEAT: c_int = 2;
/** @} */

/** @defgroup hat_state Joystick hat states
*  @brief Joystick hat states.
*
*  See [joystick hat input](@ref joystick_hat) for how these are used.
*
*  @ingroup input
*  @{ */
pub const HAT_CENTERED: c_int = 0;
pub const HAT_UP: c_int = 1;
pub const HAT_RIGHT: c_int = 2;
pub const HAT_DOWN: c_int = 4;
pub const HAT_LEFT: c_int = 8;
pub const HAT_RIGHT_UP: c_int = HAT_RIGHT | HAT_UP;
pub const HAT_RIGHT_DOWN: c_int = HAT_RIGHT | HAT_DOWN;
pub const HAT_LEFT_UP: c_int = HAT_LEFT | HAT_UP;
pub const HAT_LEFT_DOWN: c_int = HAT_LEFT | HAT_DOWN;

/** @ingroup input
 */
pub const KEY_UNKNOWN: c_int = -1;

/** @} */

/** @defgroup keys Keyboard key tokens
*  @brief Keyboard key tokens.
*
*  See [key input](@ref input_key) for how these are used.
*
*  These key codes are inspired by the _USB HID Usage Tables v1.12_ (p. 53-60),
*  but re-arranged to map to 7-bit ASCII for printable keys (function keys are
*  put in the 256+ range).
*
*  The naming of the key codes follow these rules:
*   - The US keyboard layout is used
*   - Names of printable alphanumeric characters are used (e.g. "A", "R",
*     "3", etc.)
*   - For non-alphanumeric characters, Unicode:ish names are used (e.g.
*     "COMMA", "LEFT_SQUARE_BRACKET", etc.). Note that some names do not
*     correspond to the Unicode standard (usually for brevity)
*   - Keys that lack a clear US mapping are named "WORLD_x"
*   - For non-printable keys, custom names are used (e.g. "F4",
*     "BACKSPACE", etc.)
*
*  @ingroup input
*  @{
 */

/* Printable keys */
pub const KEY_SPACE: c_int = 32;
pub const KEY_APOSTROPHE: c_int = 39  /* ' */;
pub const KEY_COMMA: c_int = 44  /* , */;
pub const KEY_MINUS: c_int = 45  /* - */;
pub const KEY_PERIOD: c_int = 46  /* . */;
pub const KEY_SLASH: c_int = 47  /* / */;
pub const KEY_0: c_int = 48;
pub const KEY_1: c_int = 49;
pub const KEY_2: c_int = 50;
pub const KEY_3: c_int = 51;
pub const KEY_4: c_int = 52;
pub const KEY_5: c_int = 53;
pub const KEY_6: c_int = 54;
pub const KEY_7: c_int = 55;
pub const KEY_8: c_int = 56;
pub const KEY_9: c_int = 57;
pub const KEY_SEMICOLON: c_int = 59  /* ; */;
pub const KEY_EQUAL: c_int = 61  /* = */;
pub const KEY_A: c_int = 65;
pub const KEY_B: c_int = 66;
pub const KEY_C: c_int = 67;
pub const KEY_D: c_int = 68;
pub const KEY_E: c_int = 69;
pub const KEY_F: c_int = 70;
pub const KEY_G: c_int = 71;
pub const KEY_H: c_int = 72;
pub const KEY_I: c_int = 73;
pub const KEY_J: c_int = 74;
pub const KEY_K: c_int = 75;
pub const KEY_L: c_int = 76;
pub const KEY_M: c_int = 77;
pub const KEY_N: c_int = 78;
pub const KEY_O: c_int = 79;
pub const KEY_P: c_int = 80;
pub const KEY_Q: c_int = 81;
pub const KEY_R: c_int = 82;
pub const KEY_S: c_int = 83;
pub const KEY_T: c_int = 84;
pub const KEY_U: c_int = 85;
pub const KEY_V: c_int = 86;
pub const KEY_W: c_int = 87;
pub const KEY_X: c_int = 88;
pub const KEY_Y: c_int = 89;
pub const KEY_Z: c_int = 90;
pub const KEY_LEFT_BRACKET: c_int = 91  /* [ */;
pub const KEY_BACKSLASH: c_int = 92  /* \ */;
pub const KEY_RIGHT_BRACKET: c_int = 93  /* ] */;
pub const KEY_GRAVE_ACCENT: c_int = 96  /* ` */;
pub const KEY_WORLD_1: c_int = 161 /* non-US #1 */;
pub const KEY_WORLD_2: c_int = 162 /* non-US #2 */;

/* Function keys */
pub const KEY_ESCAPE: c_int = 256;
pub const KEY_ENTER: c_int = 257;
pub const KEY_TAB: c_int = 258;
pub const KEY_BACKSPACE: c_int = 259;
pub const KEY_INSERT: c_int = 260;
pub const KEY_DELETE: c_int = 261;
pub const KEY_RIGHT: c_int = 262;
pub const KEY_LEFT: c_int = 263;
pub const KEY_DOWN: c_int = 264;
pub const KEY_UP: c_int = 265;
pub const KEY_PAGE_UP: c_int = 266;
pub const KEY_PAGE_DOWN: c_int = 267;
pub const KEY_HOME: c_int = 268;
pub const KEY_END: c_int = 269;
pub const KEY_CAPS_LOCK: c_int = 280;
pub const KEY_SCROLL_LOCK: c_int = 281;
pub const KEY_NUM_LOCK: c_int = 282;
pub const KEY_PRINT_SCREEN: c_int = 283;
pub const KEY_PAUSE: c_int = 284;
pub const KEY_F1: c_int = 290;
pub const KEY_F2: c_int = 291;
pub const KEY_F3: c_int = 292;
pub const KEY_F4: c_int = 293;
pub const KEY_F5: c_int = 294;
pub const KEY_F6: c_int = 295;
pub const KEY_F7: c_int = 296;
pub const KEY_F8: c_int = 297;
pub const KEY_F9: c_int = 298;
pub const KEY_F10: c_int = 299;
pub const KEY_F11: c_int = 300;
pub const KEY_F12: c_int = 301;
pub const KEY_F13: c_int = 302;
pub const KEY_F14: c_int = 303;
pub const KEY_F15: c_int = 304;
pub const KEY_F16: c_int = 305;
pub const KEY_F17: c_int = 306;
pub const KEY_F18: c_int = 307;
pub const KEY_F19: c_int = 308;
pub const KEY_F20: c_int = 309;
pub const KEY_F21: c_int = 310;
pub const KEY_F22: c_int = 311;
pub const KEY_F23: c_int = 312;
pub const KEY_F24: c_int = 313;
pub const KEY_F25: c_int = 314;
pub const KEY_KP_0: c_int = 320;
pub const KEY_KP_1: c_int = 321;
pub const KEY_KP_2: c_int = 322;
pub const KEY_KP_3: c_int = 323;
pub const KEY_KP_4: c_int = 324;
pub const KEY_KP_5: c_int = 325;
pub const KEY_KP_6: c_int = 326;
pub const KEY_KP_7: c_int = 327;
pub const KEY_KP_8: c_int = 328;
pub const KEY_KP_9: c_int = 329;
pub const KEY_KP_DECIMAL: c_int = 330;
pub const KEY_KP_DIVIDE: c_int = 331;
pub const KEY_KP_MULTIPLY: c_int = 332;
pub const KEY_KP_SUBTRACT: c_int = 333;
pub const KEY_KP_ADD: c_int = 334;
pub const KEY_KP_ENTER: c_int = 335;
pub const KEY_KP_EQUAL: c_int = 336;
pub const KEY_LEFT_SHIFT: c_int = 340;
pub const KEY_LEFT_CONTROL: c_int = 341;
pub const KEY_LEFT_ALT: c_int = 342;
pub const KEY_LEFT_SUPER: c_int = 343;
pub const KEY_RIGHT_SHIFT: c_int = 344;
pub const KEY_RIGHT_CONTROL: c_int = 345;
pub const KEY_RIGHT_ALT: c_int = 346;
pub const KEY_RIGHT_SUPER: c_int = 347;
pub const KEY_MENU: c_int = 348;

pub const KEY_LAST: c_int = KEY_MENU;

/** @} */

/** @defgroup mods Modifier key flags
*  @brief Modifier key flags.
*
*  See [key input](@ref input_key) for how these are used.
*
*  @ingroup input
*  @{ */

/** @brief If this bit is set one or more Shift keys were held down.
*
*  If this bit is set one or more Shift keys were held down.
 */
pub const MOD_SHIFT: c_int = 0x0001;
/** @brief If this bit is set one or more Control keys were held down.
*
*  If this bit is set one or more Control keys were held down.
 */
pub const MOD_CONTROL: c_int = 0x0002;
/** @brief If this bit is set one or more Alt keys were held down.
*
*  If this bit is set one or more Alt keys were held down.
 */
pub const MOD_ALT: c_int = 0x0004;
/** @brief If this bit is set one or more Super keys were held down.
*
*  If this bit is set one or more Super keys were held down.
 */
pub const MOD_SUPER: c_int = 0x0008;
/** @brief If this bit is set the Caps Lock key is enabled.
*
*  If this bit is set the Caps Lock key is enabled and the @ref
*  GLFW_LOCK_KEY_MODS input mode is set.
 */
pub const MOD_CAPS_LOCK: c_int = 0x0010;
/** @brief If this bit is set the Num Lock key is enabled.
*
*  If this bit is set the Num Lock key is enabled and the @ref
*  GLFW_LOCK_KEY_MODS input mode is set.
 */
pub const MOD_NUM_LOCK: c_int = 0x0020;

/** @} */

/** @defgroup buttons Mouse buttons
*  @brief Mouse button IDs.
*
*  See [mouse button input](@ref input_mouse_button) for how these are used.
*
*  @ingroup input
*  @{ */
pub const MOUSE_BUTTON_1: c_int = 0;
pub const MOUSE_BUTTON_2: c_int = 1;
pub const MOUSE_BUTTON_3: c_int = 2;
pub const MOUSE_BUTTON_4: c_int = 3;
pub const MOUSE_BUTTON_5: c_int = 4;
pub const MOUSE_BUTTON_6: c_int = 5;
pub const MOUSE_BUTTON_7: c_int = 6;
pub const MOUSE_BUTTON_8: c_int = 7;
pub const MOUSE_BUTTON_LAST: c_int = MOUSE_BUTTON_8;
pub const MOUSE_BUTTON_LEFT: c_int = MOUSE_BUTTON_1;
pub const MOUSE_BUTTON_RIGHT: c_int = MOUSE_BUTTON_2;
pub const MOUSE_BUTTON_MIDDLE: c_int = MOUSE_BUTTON_3;
/** @} */

/** @defgroup joysticks Joysticks
*  @brief Joystick IDs.
*
*  See [joystick input](@ref joystick) for how these are used.
*
*  @ingroup input
*  @{ */
pub const JOYSTICK_1: c_int = 0;
pub const JOYSTICK_2: c_int = 1;
pub const JOYSTICK_3: c_int = 2;
pub const JOYSTICK_4: c_int = 3;
pub const JOYSTICK_5: c_int = 4;
pub const JOYSTICK_6: c_int = 5;
pub const JOYSTICK_7: c_int = 6;
pub const JOYSTICK_8: c_int = 7;
pub const JOYSTICK_9: c_int = 8;
pub const JOYSTICK_10: c_int = 9;
pub const JOYSTICK_11: c_int = 10;
pub const JOYSTICK_12: c_int = 11;
pub const JOYSTICK_13: c_int = 12;
pub const JOYSTICK_14: c_int = 13;
pub const JOYSTICK_15: c_int = 14;
pub const JOYSTICK_16: c_int = 15;
pub const JOYSTICK_LAST: c_int = JOYSTICK_16;
/** @} */

/** @defgroup gamepad_buttons Gamepad buttons
*  @brief Gamepad buttons.
*
*  See @ref gamepad for how these are used.
*
*  @ingroup input
*  @{ */
pub const GAMEPAD_BUTTON_A: c_int = 0;
pub const GAMEPAD_BUTTON_B: c_int = 1;
pub const GAMEPAD_BUTTON_X: c_int = 2;
pub const GAMEPAD_BUTTON_Y: c_int = 3;
pub const GAMEPAD_BUTTON_LEFT_BUMPER: c_int = 4;
pub const GAMEPAD_BUTTON_RIGHT_BUMPER: c_int = 5;
pub const GAMEPAD_BUTTON_BACK: c_int = 6;
pub const GAMEPAD_BUTTON_START: c_int = 7;
pub const GAMEPAD_BUTTON_GUIDE: c_int = 8;
pub const GAMEPAD_BUTTON_LEFT_THUMB: c_int = 9;
pub const GAMEPAD_BUTTON_RIGHT_THUMB: c_int = 10;
pub const GAMEPAD_BUTTON_DPAD_UP: c_int = 11;
pub const GAMEPAD_BUTTON_DPAD_RIGHT: c_int = 12;
pub const GAMEPAD_BUTTON_DPAD_DOWN: c_int = 13;
pub const GAMEPAD_BUTTON_DPAD_LEFT: c_int = 14;
pub const GAMEPAD_BUTTON_LAST: c_int = GAMEPAD_BUTTON_DPAD_LEFT;

pub const GAMEPAD_BUTTON_CROSS: c_int = GAMEPAD_BUTTON_A;
pub const GAMEPAD_BUTTON_CIRCLE: c_int = GAMEPAD_BUTTON_B;
pub const GAMEPAD_BUTTON_SQUARE: c_int = GAMEPAD_BUTTON_X;
pub const GAMEPAD_BUTTON_TRIANGLE: c_int = GAMEPAD_BUTTON_Y;
/** @} */

/** @defgroup gamepad_axes Gamepad axes
*  @brief Gamepad axes.
*
*  See @ref gamepad for how these are used.
*
*  @ingroup input
*  @{ */
pub const GAMEPAD_AXIS_LEFT_X: c_int = 0;
pub const GAMEPAD_AXIS_LEFT_Y: c_int = 1;
pub const GAMEPAD_AXIS_RIGHT_X: c_int = 2;
pub const GAMEPAD_AXIS_RIGHT_Y: c_int = 3;
pub const GAMEPAD_AXIS_LEFT_TRIGGER: c_int = 4;
pub const GAMEPAD_AXIS_RIGHT_TRIGGER: c_int = 5;
pub const GAMEPAD_AXIS_LAST: c_int = GAMEPAD_AXIS_RIGHT_TRIGGER;
/** @} */

/** @defgroup errors Error codes
*  @brief Error codes.
*
*  See [error handling](@ref error_handling) for how these are used.
*
*  @ingroup init
*  @{ */
/** @brief No error has occurred.
*
*  No error has occurred.
*
*  @analysis Yay.
 */
pub const NO_ERROR: c_int = 0;
/** @brief GLFW has not been initialized.
*
*  This occurs if a GLFW function was called that must not be called unless the
*  library is [initialized](@ref intro_init).
*
*  @analysis Application programmer error.  Initialize GLFW before calling any
*  function that requires initialization.
 */
pub const NOT_INITIALIZED: c_int = 0x00010001;
/** @brief No context is current for this thread.
*
*  This occurs if a GLFW function was called that needs and operates on the
*  current OpenGL or OpenGL ES context but no context is current on the calling
*  thread.  One such function is @ref glfwSwapInterval.
*
*  @analysis Application programmer error.  Ensure a context is current before
*  calling functions that require a current context.
 */
pub const NO_CURRENT_CONTEXT: c_int = 0x00010002;
/** @brief One of the arguments to the function was an invalid enum value.
*
*  One of the arguments to the function was an invalid enum value, for example
*  requesting @ref GLFW_RED_BITS with @ref glfwGetWindowAttrib.
*
*  @analysis Application programmer error.  Fix the offending call.
 */
pub const INVALID_ENUM: c_int = 0x00010003;
/** @brief One of the arguments to the function was an invalid value.
*
*  One of the arguments to the function was an invalid value, for example
*  requesting a non-existent OpenGL or OpenGL ES version like 2.7.
*
*  Requesting a valid but unavailable OpenGL or OpenGL ES version will instead
*  result in a @ref GLFW_VERSION_UNAVAILABLE error.
*
*  @analysis Application programmer error.  Fix the offending call.
 */
pub const INVALID_VALUE: c_int = 0x00010004;
/** @brief A memory allocation failed.
*
*  A memory allocation failed.
*
*  @analysis A bug in GLFW or the underlying operating system.  Report the bug
*  to our [issue tracker](https://github.com/glfw/glfw/issues).
 */
pub const OUT_OF_MEMORY: c_int = 0x00010005;
/** @brief GLFW could not find support for the requested API on the system.
*
*  GLFW could not find support for the requested API on the system.
*
*  @analysis The installed graphics driver does not support the requested
*  API, or does not support it via the chosen context creation API.
*  Below are a few examples.
*
*  @par
*  Some pre-installed Windows graphics drivers do not support OpenGL.  AMD only
*  supports OpenGL ES via EGL, while Nvidia and Intel only support it via
*  a WGL or GLX extension.  macOS does not provide OpenGL ES at all.  The Mesa
*  EGL, OpenGL and OpenGL ES libraries do not interface with the Nvidia binary
*  driver.  Older graphics drivers do not support Vulkan.
 */
pub const API_UNAVAILABLE: c_int = 0x00010006;
/** @brief The requested OpenGL or OpenGL ES version is not available.
*
*  The requested OpenGL or OpenGL ES version (including any requested context
*  or framebuffer hints) is not available on this machine.
*
*  @analysis The machine does not support your requirements.  If your
*  application is sufficiently flexible, downgrade your requirements and try
*  again.  Otherwise, inform the user that their machine does not match your
*  requirements.
*
*  @par
*  Future invalid OpenGL and OpenGL ES versions, for example OpenGL 4.8 if 5.0
*  comes out before the 4.x series gets that far, also fail with this error and
*  not @ref GLFW_INVALID_VALUE, because GLFW cannot know what future versions
*  will exist.
 */
pub const VERSION_UNAVAILABLE: c_int = 0x00010007;
/** @brief A platform-specific error occurred that does not match any of the
*  more specific categories.
*
*  A platform-specific error occurred that does not match any of the more
*  specific categories.
*
*  @analysis A bug or configuration error in GLFW, the underlying operating
*  system or its drivers, or a lack of required resources.  Report the issue to
*  our [issue tracker](https://github.com/glfw/glfw/issues).
 */
pub const PLATFORM_ERROR: c_int = 0x00010008;
/** @brief The requested format is not supported or available.
*
*  If emitted during window creation, the requested pixel format is not
*  supported.
*
*  If emitted when querying the clipboard, the contents of the clipboard could
*  not be converted to the requested format.
*
*  @analysis If emitted during window creation, one or more
*  [hard pub constraints](@ref window_hints_hard) did not match any of the
*  available pixel formats.  If your application is sufficiently flexible,
*  downgrade your requirements and try again.  Otherwise, inform the user that
*  their machine does not match your requirements.
*
*  @par
*  If emitted when querying the clipboard, ignore the error or report it to
*  the user, as appropriate.
 */
pub const FORMAT_UNAVAILABLE: c_int = 0x00010009;
/** @brief The specified window does not have an OpenGL or OpenGL ES context.
*
*  A window that does not have an OpenGL or OpenGL ES context was passed to
*  a function that requires it to have one.
*
*  @analysis Application programmer error.  Fix the offending call.
 */
pub const NO_WINDOW_CONTEXT: c_int = 0x0001000A;
/** @brief The specified cursor shape is not available.
*
*  The specified standard cursor shape is not available, either because the
*  current platform cursor theme does not provide it or because it is not
*  available on the platform.
*
*  @analysis Platform or system settings limitation.  Pick another
*  [standard cursor shape](@ref shapes) or create a
*  [custom cursor](@ref cursor_custom).
 */
pub const CURSOR_UNAVAILABLE: c_int = 0x0001000B;
/** @brief The requested feature is not provided by the platform.
*
*  The requested feature is not provided by the platform, so GLFW is unable to
*  implement it.  The documentation for each function notes if it could emit
*  this error.
*
*  @analysis Platform or platform version limitation.  The error can be ignored
*  unless the feature is critical to the application.
*
*  @par
*  A function call that emits this error has no effect other than the error and
*  updating any existing out parameters.
 */
pub const FEATURE_UNAVAILABLE: c_int = 0x0001000C;
/** @brief The requested feature is not implemented for the platform.
*
*  The requested feature has not yet been implemented in GLFW for this platform.
*
*  @analysis An incomplete implementation of GLFW for this platform, hopefully
*  fixed in a future release.  The error can be ignored unless the feature is
*  critical to the application.
*
*  @par
*  A function call that emits this error has no effect other than the error and
*  updating any existing out parameters.
 */
pub const FEATURE_UNIMPLEMENTED: c_int = 0x0001000D;
/** @brief Platform unavailable or no matching platform was found.
*
*  If emitted during initialization, no matching platform was found.  If the @ref
*  GLFW_PLATFORM init hint was set to `GLFW_ANY_PLATFORM`, GLFW could not detect any of
*  the platforms supported by this library binary, except for the Null platform.  If the
*  init hint was set to a specific platform, it is either not supported by this library
*  binary or GLFW was not able to detect it.
*
*  If emitted by a native access function, GLFW was initialized for a different platform
*  than the function is for.
*
*  @analysis Failure to detect any platform usually only happens on non-macOS Unix
*  systems, either when no window system is running or the program was run from
*  a terminal that does not have the necessary environment variables.  Fall back to
*  a different platform if possible or notify the user that no usable platform was
*  detected.
*
*  Failure to detect a specific platform may have the same cause as above or be because
*  support for that platform was not compiled in.  Call @ref glfwPlatformSupported to
*  check whether a specific platform is supported by a library binary.
 */
pub const PLATFORM_UNAVAILABLE: c_int = 0x0001000E;
/** @} */

/** @addtogroup window
*  @{ */
/** @brief Input focus window hint and attribute
*
*  Input focus [window hint](@ref GLFW_FOCUSED_hint) or
*  [window attribute](@ref GLFW_FOCUSED_attrib).
 */
pub const FOCUSED: c_int = 0x00020001;
/** @brief Window iconification window attribute
*
*  Window iconification [window attribute](@ref GLFW_ICONIFIED_attrib).
 */
pub const ICONIFIED: c_int = 0x00020002;
/** @brief Window resize-ability window hint and attribute
*
*  Window resize-ability [window hint](@ref GLFW_RESIZABLE_hint) and
*  [window attribute](@ref GLFW_RESIZABLE_attrib).
 */
pub const RESIZABLE: c_int = 0x00020003;
/** @brief Window visibility window hint and attribute
*
*  Window visibility [window hint](@ref GLFW_VISIBLE_hint) and
*  [window attribute](@ref GLFW_VISIBLE_attrib).
 */
pub const VISIBLE: c_int = 0x00020004;
/** @brief Window decoration window hint and attribute
*
*  Window decoration [window hint](@ref GLFW_DECORATED_hint) and
*  [window attribute](@ref GLFW_DECORATED_attrib).
 */
pub const DECORATED: c_int = 0x00020005;
/** @brief Window auto-iconification window hint and attribute
*
*  Window auto-iconification [window hint](@ref GLFW_AUTO_ICONIFY_hint) and
*  [window attribute](@ref GLFW_AUTO_ICONIFY_attrib).
 */
pub const AUTO_ICONIFY: c_int = 0x00020006;
/** @brief Window decoration window hint and attribute
*
*  Window decoration [window hint](@ref GLFW_FLOATING_hint) and
*  [window attribute](@ref GLFW_FLOATING_attrib).
 */
pub const FLOATING: c_int = 0x00020007;
/** @brief Window maximization window hint and attribute
*
*  Window maximization [window hint](@ref GLFW_MAXIMIZED_hint) and
*  [window attribute](@ref GLFW_MAXIMIZED_attrib).
 */
pub const MAXIMIZED: c_int = 0x00020008;
/** @brief Cursor centering window hint
*
*  Cursor centering [window hint](@ref GLFW_CENTER_CURSOR_hint).
 */
pub const CENTER_CURSOR: c_int = 0x00020009;
/** @brief Window framebuffer transparency hint and attribute
*
*  Window framebuffer transparency
*  [window hint](@ref GLFW_TRANSPARENT_FRAMEBUFFER_hint) and
*  [window attribute](@ref GLFW_TRANSPARENT_FRAMEBUFFER_attrib).
 */
pub const TRANSPARENT_FRAMEBUFFER: c_int = 0x0002000A;
/** @brief Mouse cursor hover window attribute.
*
*  Mouse cursor hover [window attribute](@ref GLFW_HOVERED_attrib).
 */
pub const HOVERED: c_int = 0x0002000B;
/** @brief Input focus on calling show window hint and attribute
*
*  Input focus [window hint](@ref GLFW_FOCUS_ON_SHOW_hint) or
*  [window attribute](@ref GLFW_FOCUS_ON_SHOW_attrib).
 */
pub const FOCUS_ON_SHOW: c_int = 0x0002000C;

/** @brief Mouse input transparency window hint and attribute
*
*  Mouse input transparency [window hint](@ref GLFW_MOUSE_PASSTHROUGH_hint) or
*  [window attribute](@ref GLFW_MOUSE_PASSTHROUGH_attrib).
 */
pub const MOUSE_PASSTHROUGH: c_int = 0x0002000D;

/** @brief Initial position x-coordinate window hint.
*
*  Initial position x-coordinate [window hint](@ref GLFW_POSITION_X).
 */
pub const POSITION_X: c_int = 0x0002000E;

/** @brief Initial position y-coordinate window hint.
*
*  Initial position y-coordinate [window hint](@ref GLFW_POSITION_Y).
 */
pub const POSITION_Y: c_int = 0x0002000F;

/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_RED_BITS).
 */
pub const RED_BITS: c_int = 0x00021001;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_GREEN_BITS).
 */
pub const GREEN_BITS: c_int = 0x00021002;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_BLUE_BITS).
 */
pub const BLUE_BITS: c_int = 0x00021003;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_ALPHA_BITS).
 */
pub const ALPHA_BITS: c_int = 0x00021004;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_DEPTH_BITS).
 */
pub const DEPTH_BITS: c_int = 0x00021005;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_STENCIL_BITS).
 */
pub const STENCIL_BITS: c_int = 0x00021006;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_ACCUM_RED_BITS).
 */
pub const ACCUM_RED_BITS: c_int = 0x00021007;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_ACCUM_GREEN_BITS).
 */
pub const ACCUM_GREEN_BITS: c_int = 0x00021008;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_ACCUM_BLUE_BITS).
 */
pub const ACCUM_BLUE_BITS: c_int = 0x00021009;
/** @brief Framebuffer bit depth hint.
*
*  Framebuffer bit depth [hint](@ref GLFW_ACCUM_ALPHA_BITS).
 */
pub const ACCUM_ALPHA_BITS: c_int = 0x0002100A;
/** @brief Framebuffer auxiliary buffer hint.
*
*  Framebuffer auxiliary buffer [hint](@ref GLFW_AUX_BUFFERS).
 */
pub const AUX_BUFFERS: c_int = 0x0002100B;
/** @brief OpenGL stereoscopic rendering hint.
*
*  OpenGL stereoscopic rendering [hint](@ref GLFW_STEREO).
 */
pub const STEREO: c_int = 0x0002100C;
/** @brief Framebuffer MSAA samples hint.
*
*  Framebuffer MSAA samples [hint](@ref GLFW_SAMPLES).
 */
pub const SAMPLES: c_int = 0x0002100D;
/** @brief Framebuffer sRGB hint.
*
*  Framebuffer sRGB [hint](@ref GLFW_SRGB_CAPABLE).
 */
pub const SRGB_CAPABLE: c_int = 0x0002100E;
/** @brief Monitor refresh rate hint.
*
*  Monitor refresh rate [hint](@ref GLFW_REFRESH_RATE).
 */
pub const REFRESH_RATE: c_int = 0x0002100F;
/** @brief Framebuffer double buffering hint and attribute.
*
*  Framebuffer double buffering [hint](@ref GLFW_DOUBLEBUFFER_hint) and
*  [attribute](@ref GLFW_DOUBLEBUFFER_attrib).
 */
pub const DOUBLEBUFFER: c_int = 0x00021010;

/** @brief Context client API hint and attribute.
*
*  Context client API [hint](@ref GLFW_CLIENT_API_hint) and
*  [attribute](@ref GLFW_CLIENT_API_attrib).
 */
pub const CLIENT_API: c_int = 0x00022001;
/** @brief Context client API major version hint and attribute.
*
*  Context client API major version [hint](@ref GLFW_CONTEXT_VERSION_MAJOR_hint)
*  and [attribute](@ref GLFW_CONTEXT_VERSION_MAJOR_attrib).
 */
pub const CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
/** @brief Context client API minor version hint and attribute.
*
*  Context client API minor version [hint](@ref GLFW_CONTEXT_VERSION_MINOR_hint)
*  and [attribute](@ref GLFW_CONTEXT_VERSION_MINOR_attrib).
 */
pub const CONTEXT_VERSION_MINOR: c_int = 0x00022003;
/** @brief Context client API revision number attribute.
*
*  Context client API revision number
*  [attribute](@ref GLFW_CONTEXT_REVISION_attrib).
 */
pub const CONTEXT_REVISION: c_int = 0x00022004;
/** @brief Context robustness hint and attribute.
*
*  Context client API revision number [hint](@ref GLFW_CONTEXT_ROBUSTNESS_hint)
*  and [attribute](@ref GLFW_CONTEXT_ROBUSTNESS_attrib).
 */
pub const CONTEXT_ROBUSTNESS: c_int = 0x00022005;
/** @brief OpenGL forward-compatibility hint and attribute.
*
*  OpenGL forward-compatibility [hint](@ref GLFW_OPENGL_FORWARD_COMPAT_hint)
*  and [attribute](@ref GLFW_OPENGL_FORWARD_COMPAT_attrib).
 */
pub const OPENGL_FORWARD_COMPAT: c_int = 0x00022006;
/** @brief Debug mode context hint and attribute.
*
*  Debug mode context [hint](@ref GLFW_CONTEXT_DEBUG_hint) and
*  [attribute](@ref GLFW_CONTEXT_DEBUG_attrib).
 */
pub const CONTEXT_DEBUG: c_int = 0x00022007;
/** @brief Legacy name for compatibility.
*
*  This is an alias for compatibility with earlier versions.
 */
pub const OPENGL_DEBUG_CONTEXT: c_int = CONTEXT_DEBUG;
/** @brief OpenGL profile hint and attribute.
*
*  OpenGL profile [hint](@ref GLFW_OPENGL_PROFILE_hint) and
*  [attribute](@ref GLFW_OPENGL_PROFILE_attrib).
 */
pub const OPENGL_PROFILE: c_int = 0x00022008;
/** @brief Context flush-on-release hint and attribute.
*
*  Context flush-on-release [hint](@ref GLFW_CONTEXT_RELEASE_BEHAVIOR_hint) and
*  [attribute](@ref GLFW_CONTEXT_RELEASE_BEHAVIOR_attrib).
 */
pub const CONTEXT_RELEASE_BEHAVIOR: c_int = 0x00022009;
/** @brief Context error suppression hint and attribute.
*
*  Context error suppression [hint](@ref GLFW_CONTEXT_NO_ERROR_hint) and
*  [attribute](@ref GLFW_CONTEXT_NO_ERROR_attrib).
 */
pub const CONTEXT_NO_ERROR: c_int = 0x0002200A;
/** @brief Context creation API hint and attribute.
*
*  Context creation API [hint](@ref GLFW_CONTEXT_CREATION_API_hint) and
*  [attribute](@ref GLFW_CONTEXT_CREATION_API_attrib).
 */
pub const CONTEXT_CREATION_API: c_int = 0x0002200B;
/** @brief Window content area scaling window
*  [window hint](@ref GLFW_SCALE_TO_MONITOR).
 */
pub const SCALE_TO_MONITOR: c_int = 0x0002200C;
/** @brief Window framebuffer scaling
*  [window hint](@ref GLFW_SCALE_FRAMEBUFFER_hint).
 */
pub const SCALE_FRAMEBUFFER: c_int = 0x0002200D;
/** @brief Legacy name for compatibility.
*
*  This is an alias for the
*  [GLFW_SCALE_FRAMEBUFFER](@ref GLFW_SCALE_FRAMEBUFFER_hint) window hint for
*  compatibility with earlier versions.
 */
pub const COCOA_RETINA_FRAMEBUFFER: c_int = 0x00023001;
/** @brief macOS specific
*  [window hint](@ref GLFW_COCOA_FRAME_NAME_hint).
 */
pub const COCOA_FRAME_NAME: c_int = 0x00023002;
/** @brief macOS specific
*  [window hint](@ref GLFW_COCOA_GRAPHICS_SWITCHING_hint).
 */
pub const COCOA_GRAPHICS_SWITCHING: c_int = 0x00023003;
/** @brief X11 specific
*  [window hint](@ref GLFW_X11_CLASS_NAME_hint).
 */
pub const X11_CLASS_NAME: c_int = 0x00024001;
/** @brief X11 specific
*  [window hint](@ref GLFW_X11_CLASS_NAME_hint).
 */
pub const X11_INSTANCE_NAME: c_int = 0x00024002;
pub const WIN32_KEYBOARD_MENU: c_int = 0x00025001;
/** @brief Win32 specific [window hint](@ref GLFW_WIN32_SHOWDEFAULT_hint).
 */
pub const WIN32_SHOWDEFAULT: c_int = 0x00025002;
/** @brief Wayland specific
*  [window hint](@ref GLFW_WAYLAND_APP_ID_hint).
*
*  Allows specification of the Wayland app_id.
 */
pub const WAYLAND_APP_ID: c_int = 0x00026001;
/** @} */

pub const NO_API: c_int = 0;
pub const OPENGL_API: c_int = 0x00030001;
pub const OPENGL_ES_API: c_int = 0x00030002;

pub const NO_ROBUSTNESS: c_int = 0;
pub const NO_RESET_NOTIFICATION: c_int = 0x00031001;
pub const LOSE_CONTEXT_ON_RESET: c_int = 0x00031002;

pub const OPENGL_ANY_PROFILE: c_int = 0;
pub const OPENGL_CORE_PROFILE: c_int = 0x00032001;
pub const OPENGL_COMPAT_PROFILE: c_int = 0x00032002;

pub const CURSOR: c_int = 0x00033001;
pub const STICKY_KEYS: c_int = 0x00033002;
pub const STICKY_MOUSE_BUTTONS: c_int = 0x00033003;
pub const LOCK_KEY_MODS: c_int = 0x00033004;
pub const RAW_MOUSE_MOTION: c_int = 0x00033005;

pub const CURSOR_NORMAL: c_int = 0x00034001;
pub const CURSOR_HIDDEN: c_int = 0x00034002;
pub const CURSOR_DISABLED: c_int = 0x00034003;
pub const CURSOR_CAPTURED: c_int = 0x00034004;

pub const ANY_RELEASE_BEHAVIOR: c_int = 0;
pub const RELEASE_BEHAVIOR_FLUSH: c_int = 0x00035001;
pub const RELEASE_BEHAVIOR_NONE: c_int = 0x00035002;

pub const NATIVE_CONTEXT_API: c_int = 0x00036001;
pub const EGL_CONTEXT_API: c_int = 0x00036002;
pub const OSMESA_CONTEXT_API: c_int = 0x00036003;

pub const ANGLE_PLATFORM_TYPE_NONE: c_int = 0x00037001;
pub const ANGLE_PLATFORM_TYPE_OPENGL: c_int = 0x00037002;
pub const ANGLE_PLATFORM_TYPE_OPENGLES: c_int = 0x00037003;
pub const ANGLE_PLATFORM_TYPE_D3D9: c_int = 0x00037004;
pub const ANGLE_PLATFORM_TYPE_D3D11: c_int = 0x00037005;
pub const ANGLE_PLATFORM_TYPE_VULKAN: c_int = 0x00037007;
pub const ANGLE_PLATFORM_TYPE_METAL: c_int = 0x00037008;

pub const WAYLAND_PREFER_LIBDECOR: c_int = 0x00038001;
pub const WAYLAND_DISABLE_LIBDECOR: c_int = 0x00038002;

// pub const ANY_POSITION: c_int = 0x80000000; // TODO too big for c_int?

/** @defgroup shapes Standard cursor shapes
*  @brief Standard system cursor shapes.
*
*  These are the [standard cursor shapes](@ref cursor_standard) that can be
*  requested from the platform (window system).
*
*  @ingroup input
*  @{ */

/** @brief The regular arrow cursor shape.
*
*  The regular arrow cursor shape.
 */
pub const ARROW_CURSOR: c_int = 0x00036001;
/** @brief The text input I-beam cursor shape.
*
*  The text input I-beam cursor shape.
 */
pub const IBEAM_CURSOR: c_int = 0x00036002;
/** @brief The crosshair cursor shape.
*
*  The crosshair cursor shape.
 */
pub const CROSSHAIR_CURSOR: c_int = 0x00036003;
/** @brief The pointing hand cursor shape.
*
*  The pointing hand cursor shape.
 */
pub const POINTING_HAND_CURSOR: c_int = 0x00036004;
/** @brief The horizontal resize/move arrow shape.
*
*  The horizontal resize/move arrow shape.  This is usually a horizontal
*  double-headed arrow.
 */
pub const RESIZE_EW_CURSOR: c_int = 0x00036005;
/** @brief The vertical resize/move arrow shape.
*
*  The vertical resize/move shape.  This is usually a vertical double-headed
*  arrow.
 */
pub const RESIZE_NS_CURSOR: c_int = 0x00036006;
/** @brief The top-left to bottom-right diagonal resize/move arrow shape.
*
*  The top-left to bottom-right diagonal resize/move shape.  This is usually
*  a diagonal double-headed arrow.
*
*  @note @macos This shape is provided by a private system API and may fail
*  with @ref GLFW_CURSOR_UNAVAILABLE in the future.
*
*  @note @wayland This shape is provided by a newer standard not supported by
*  all cursor themes.
*
*  @note @x11 This shape is provided by a newer standard not supported by all
*  cursor themes.
 */
pub const RESIZE_NWSE_CURSOR: c_int = 0x00036007;
/** @brief The top-right to bottom-left diagonal resize/move arrow shape.
*
*  The top-right to bottom-left diagonal resize/move shape.  This is usually
*  a diagonal double-headed arrow.
*
*  @note @macos This shape is provided by a private system API and may fail
*  with @ref GLFW_CURSOR_UNAVAILABLE in the future.
*
*  @note @wayland This shape is provided by a newer standard not supported by
*  all cursor themes.
*
*  @note @x11 This shape is provided by a newer standard not supported by all
*  cursor themes.
 */
pub const RESIZE_NESW_CURSOR: c_int = 0x00036008;
/** @brief The omni-directional resize/move cursor shape.
*
*  The omni-directional resize cursor/move shape.  This is usually either
*  a combined horizontal and vertical double-headed arrow or a grabbing hand.
 */
pub const RESIZE_ALL_CURSOR: c_int = 0x00036009;
/** @brief The operation-not-allowed shape.
*
*  The operation-not-allowed shape.  This is usually a circle with a diagonal
*  line through it.
*
*  @note @wayland This shape is provided by a newer standard not supported by
*  all cursor themes.
*
*  @note @x11 This shape is provided by a newer standard not supported by all
*  cursor themes.
 */
pub const NOT_ALLOWED_CURSOR: c_int = 0x0003600A;
/** @brief Legacy name for compatibility.
*
*  This is an alias for compatibility with earlier versions.
 */
pub const HRESIZE_CURSOR: c_int = RESIZE_EW_CURSOR;
/** @brief Legacy name for compatibility.
*
*  This is an alias for compatibility with earlier versions.
 */
pub const VRESIZE_CURSOR: c_int = RESIZE_NS_CURSOR;
/** @brief Legacy name for compatibility.
*
*  This is an alias for compatibility with earlier versions.
 */
pub const HAND_CURSOR: c_int = POINTING_HAND_CURSOR;
/** @} */

pub const CONNECTED: c_int = 0x00040001;
pub const DISCONNECTED: c_int = 0x00040002;

/** @addtogroup init
*  @{ */
/** @brief Joystick hat buttons init hint.
*
*  Joystick hat buttons [init hint](@ref GLFW_JOYSTICK_HAT_BUTTONS).
 */
pub const JOYSTICK_HAT_BUTTONS: c_int = 0x00050001;
/** @brief ANGLE rendering backend init hint.
*
*  ANGLE rendering backend [init hint](@ref GLFW_ANGLE_PLATFORM_TYPE_hint).
 */
pub const ANGLE_PLATFORM_TYPE: c_int = 0x00050002;
/** @brief Platform selection init hint.
*
*  Platform selection [init hint](@ref GLFW_PLATFORM).
 */
pub const PLATFORM: c_int = 0x00050003;
/** @brief macOS specific init hint.
*
*  macOS specific [init hint](@ref GLFW_COCOA_CHDIR_RESOURCES_hint).
 */
pub const COCOA_CHDIR_RESOURCES: c_int = 0x00051001;
/** @brief macOS specific init hint.
*
*  macOS specific [init hint](@ref GLFW_COCOA_MENUBAR_hint).
 */
pub const COCOA_MENUBAR: c_int = 0x00051002;
/** @brief X11 specific init hint.
*
*  X11 specific [init hint](@ref GLFW_X11_XCB_VULKAN_SURFACE_hint).
 */
pub const X11_XCB_VULKAN_SURFACE: c_int = 0x00052001;
/** @brief Wayland specific init hint.
*
*  Wayland specific [init hint](@ref GLFW_WAYLAND_LIBDECOR_hint).
 */
pub const WAYLAND_LIBDECOR: c_int = 0x00053001;
/** @} */

/** @addtogroup init
*  @{ */
/** @brief Hint value that enables automatic platform selection.
*
*  Hint value for @ref GLFW_PLATFORM that enables automatic platform selection.
 */
pub const ANY_PLATFORM: c_int = 0x00060000;
pub const PLATFORM_WIN32: c_int = 0x00060001;
pub const PLATFORM_COCOA: c_int = 0x00060002;
pub const PLATFORM_WAYLAND: c_int = 0x00060003;
pub const PLATFORM_X11: c_int = 0x00060004;
pub const PLATFORM_NULL: c_int = 0x00060005;
/** @} */

pub const DONT_CARE: c_int = -1;
