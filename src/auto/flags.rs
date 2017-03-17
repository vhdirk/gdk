// This file was generated by gir (310a2da) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::translate::*;

#[cfg(feature = "v3_22")]
bitflags! {
    pub flags AnchorHints: u32 {
        const ANCHOR_FLIP_X = 1,
        const ANCHOR_FLIP_Y = 2,
        const ANCHOR_SLIDE_X = 4,
        const ANCHOR_SLIDE_Y = 8,
        const ANCHOR_RESIZE_X = 16,
        const ANCHOR_RESIZE_Y = 32,
        const ANCHOR_FLIP = 3,
        const ANCHOR_SLIDE = 12,
        const ANCHOR_RESIZE = 48,
    }
}

#[cfg(feature = "v3_22")]
#[doc(hidden)]
impl ToGlib for AnchorHints {
    type GlibType = ffi::GdkAnchorHints;

    fn to_glib(&self) -> ffi::GdkAnchorHints {
        ffi::GdkAnchorHints::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_22")]
#[doc(hidden)]
impl FromGlib<ffi::GdkAnchorHints> for AnchorHints {
    fn from_glib(value: ffi::GdkAnchorHints) -> AnchorHints {
        skip_assert_initialized!();
        AnchorHints::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v3_22")]
bitflags! {
    pub flags AxisFlags: u32 {
        const AXIS_FLAG_X = 2,
        const AXIS_FLAG_Y = 4,
        const AXIS_FLAG_PRESSURE = 8,
        const AXIS_FLAG_XTILT = 16,
        const AXIS_FLAG_YTILT = 32,
        const AXIS_FLAG_WHEEL = 64,
        const AXIS_FLAG_DISTANCE = 128,
        const AXIS_FLAG_ROTATION = 256,
        const AXIS_FLAG_SLIDER = 512,
    }
}

#[cfg(feature = "v3_22")]
#[doc(hidden)]
impl ToGlib for AxisFlags {
    type GlibType = ffi::GdkAxisFlags;

    fn to_glib(&self) -> ffi::GdkAxisFlags {
        ffi::GdkAxisFlags::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_22")]
#[doc(hidden)]
impl FromGlib<ffi::GdkAxisFlags> for AxisFlags {
    fn from_glib(value: ffi::GdkAxisFlags) -> AxisFlags {
        skip_assert_initialized!();
        AxisFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags DragAction: u32 {
        const ACTION_DEFAULT = 1,
        const ACTION_COPY = 2,
        const ACTION_MOVE = 4,
        const ACTION_LINK = 8,
        const ACTION_PRIVATE = 16,
        const ACTION_ASK = 32,
    }
}

#[doc(hidden)]
impl ToGlib for DragAction {
    type GlibType = ffi::GdkDragAction;

    fn to_glib(&self) -> ffi::GdkDragAction {
        ffi::GdkDragAction::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkDragAction> for DragAction {
    fn from_glib(value: ffi::GdkDragAction) -> DragAction {
        skip_assert_initialized!();
        DragAction::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags EventMask: u32 {
        const EXPOSURE_MASK = 2,
        const POINTER_MOTION_MASK = 4,
        const POINTER_MOTION_HINT_MASK = 8,
        const BUTTON_MOTION_MASK = 16,
        const BUTTON1_MOTION_MASK = 32,
        const BUTTON2_MOTION_MASK = 64,
        const BUTTON3_MOTION_MASK = 128,
        const BUTTON_PRESS_MASK = 256,
        const BUTTON_RELEASE_MASK = 512,
        const KEY_PRESS_MASK = 1024,
        const KEY_RELEASE_MASK = 2048,
        const ENTER_NOTIFY_MASK = 4096,
        const LEAVE_NOTIFY_MASK = 8192,
        const FOCUS_CHANGE_MASK = 16384,
        const STRUCTURE_MASK = 32768,
        const PROPERTY_CHANGE_MASK = 65536,
        const VISIBILITY_NOTIFY_MASK = 131072,
        const PROXIMITY_IN_MASK = 262144,
        const PROXIMITY_OUT_MASK = 524288,
        const SUBSTRUCTURE_MASK = 1048576,
        const SCROLL_MASK = 2097152,
        const TOUCH_MASK = 4194304,
        const SMOOTH_SCROLL_MASK = 8388608,
        const TOUCHPAD_GESTURE_MASK = 16777216,
        const TABLET_PAD_MASK = 33554432,
        const ALL_EVENTS_MASK = 16777214,
    }
}

#[doc(hidden)]
impl ToGlib for EventMask {
    type GlibType = ffi::GdkEventMask;

    fn to_glib(&self) -> ffi::GdkEventMask {
        ffi::GdkEventMask::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkEventMask> for EventMask {
    fn from_glib(value: ffi::GdkEventMask) -> EventMask {
        skip_assert_initialized!();
        EventMask::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v3_8")]
bitflags! {
    pub flags FrameClockPhase: u32 {
        const FRAME_CLOCK_PHASE_NONE = 0,
        const FRAME_CLOCK_PHASE_FLUSH_EVENTS = 1,
        const FRAME_CLOCK_PHASE_BEFORE_PAINT = 2,
        const FRAME_CLOCK_PHASE_UPDATE = 4,
        const FRAME_CLOCK_PHASE_LAYOUT = 8,
        const FRAME_CLOCK_PHASE_PAINT = 16,
        const FRAME_CLOCK_PHASE_RESUME_EVENTS = 32,
        const FRAME_CLOCK_PHASE_AFTER_PAINT = 64,
    }
}

#[cfg(feature = "v3_8")]
#[doc(hidden)]
impl ToGlib for FrameClockPhase {
    type GlibType = ffi::GdkFrameClockPhase;

    fn to_glib(&self) -> ffi::GdkFrameClockPhase {
        ffi::GdkFrameClockPhase::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_8")]
#[doc(hidden)]
impl FromGlib<ffi::GdkFrameClockPhase> for FrameClockPhase {
    fn from_glib(value: ffi::GdkFrameClockPhase) -> FrameClockPhase {
        skip_assert_initialized!();
        FrameClockPhase::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags ModifierType: u32 {
        const SHIFT_MASK = 1,
        const LOCK_MASK = 2,
        const CONTROL_MASK = 4,
        const MOD1_MASK = 8,
        const MOD2_MASK = 16,
        const MOD3_MASK = 32,
        const MOD4_MASK = 64,
        const MOD5_MASK = 128,
        const BUTTON1_MASK = 256,
        const BUTTON2_MASK = 512,
        const BUTTON3_MASK = 1024,
        const BUTTON4_MASK = 2048,
        const BUTTON5_MASK = 4096,
        const MODIFIER_RESERVED_13_MASK = 8192,
        const MODIFIER_RESERVED_14_MASK = 16384,
        const MODIFIER_RESERVED_15_MASK = 32768,
        const MODIFIER_RESERVED_16_MASK = 65536,
        const MODIFIER_RESERVED_17_MASK = 131072,
        const MODIFIER_RESERVED_18_MASK = 262144,
        const MODIFIER_RESERVED_19_MASK = 524288,
        const MODIFIER_RESERVED_20_MASK = 1048576,
        const MODIFIER_RESERVED_21_MASK = 2097152,
        const MODIFIER_RESERVED_22_MASK = 4194304,
        const MODIFIER_RESERVED_23_MASK = 8388608,
        const MODIFIER_RESERVED_24_MASK = 16777216,
        const MODIFIER_RESERVED_25_MASK = 33554432,
        const SUPER_MASK = 67108864,
        const HYPER_MASK = 134217728,
        const META_MASK = 268435456,
        const MODIFIER_RESERVED_29_MASK = 536870912,
        const RELEASE_MASK = 1073741824,
        const MODIFIER_MASK = 1543512063,
    }
}

#[doc(hidden)]
impl ToGlib for ModifierType {
    type GlibType = ffi::GdkModifierType;

    fn to_glib(&self) -> ffi::GdkModifierType {
        ffi::GdkModifierType::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkModifierType> for ModifierType {
    fn from_glib(value: ffi::GdkModifierType) -> ModifierType {
        skip_assert_initialized!();
        ModifierType::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v3_20")]
bitflags! {
    pub flags SeatCapabilities: u32 {
        const SEAT_CAPABILITY_NONE = 0,
        const SEAT_CAPABILITY_POINTER = 1,
        const SEAT_CAPABILITY_TOUCH = 2,
        const SEAT_CAPABILITY_TABLET_STYLUS = 4,
        const SEAT_CAPABILITY_KEYBOARD = 8,
        const SEAT_CAPABILITY_ALL_POINTING = 7,
        const SEAT_CAPABILITY_ALL = 15,
    }
}

#[cfg(feature = "v3_20")]
#[doc(hidden)]
impl ToGlib for SeatCapabilities {
    type GlibType = ffi::GdkSeatCapabilities;

    fn to_glib(&self) -> ffi::GdkSeatCapabilities {
        ffi::GdkSeatCapabilities::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_20")]
#[doc(hidden)]
impl FromGlib<ffi::GdkSeatCapabilities> for SeatCapabilities {
    fn from_glib(value: ffi::GdkSeatCapabilities) -> SeatCapabilities {
        skip_assert_initialized!();
        SeatCapabilities::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags WMDecoration: u32 {
        const DECOR_ALL = 1,
        const DECOR_BORDER = 2,
        const DECOR_RESIZEH = 4,
        const DECOR_TITLE = 8,
        const DECOR_MENU = 16,
        const DECOR_MINIMIZE = 32,
        const DECOR_MAXIMIZE = 64,
    }
}

#[doc(hidden)]
impl ToGlib for WMDecoration {
    type GlibType = ffi::GdkWMDecoration;

    fn to_glib(&self) -> ffi::GdkWMDecoration {
        ffi::GdkWMDecoration::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWMDecoration> for WMDecoration {
    fn from_glib(value: ffi::GdkWMDecoration) -> WMDecoration {
        skip_assert_initialized!();
        WMDecoration::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags WMFunction: u32 {
        const FUNC_ALL = 1,
        const FUNC_RESIZE = 2,
        const FUNC_MOVE = 4,
        const FUNC_MINIMIZE = 8,
        const FUNC_MAXIMIZE = 16,
        const FUNC_CLOSE = 32,
    }
}

#[doc(hidden)]
impl ToGlib for WMFunction {
    type GlibType = ffi::GdkWMFunction;

    fn to_glib(&self) -> ffi::GdkWMFunction {
        ffi::GdkWMFunction::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWMFunction> for WMFunction {
    fn from_glib(value: ffi::GdkWMFunction) -> WMFunction {
        skip_assert_initialized!();
        WMFunction::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags WindowHints: u32 {
        const HINT_POS = 1,
        const HINT_MIN_SIZE = 2,
        const HINT_MAX_SIZE = 4,
        const HINT_BASE_SIZE = 8,
        const HINT_ASPECT = 16,
        const HINT_RESIZE_INC = 32,
        const HINT_WIN_GRAVITY = 64,
        const HINT_USER_POS = 128,
        const HINT_USER_SIZE = 256,
    }
}

#[doc(hidden)]
impl ToGlib for WindowHints {
    type GlibType = ffi::GdkWindowHints;

    fn to_glib(&self) -> ffi::GdkWindowHints {
        ffi::GdkWindowHints::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWindowHints> for WindowHints {
    fn from_glib(value: ffi::GdkWindowHints) -> WindowHints {
        skip_assert_initialized!();
        WindowHints::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags WindowState: u32 {
        const WINDOW_STATE_WITHDRAWN = 1,
        const WINDOW_STATE_ICONIFIED = 2,
        const WINDOW_STATE_MAXIMIZED = 4,
        const WINDOW_STATE_STICKY = 8,
        const WINDOW_STATE_FULLSCREEN = 16,
        const WINDOW_STATE_ABOVE = 32,
        const WINDOW_STATE_BELOW = 64,
        const WINDOW_STATE_FOCUSED = 128,
        const WINDOW_STATE_TILED = 256,
    }
}

#[doc(hidden)]
impl ToGlib for WindowState {
    type GlibType = ffi::GdkWindowState;

    fn to_glib(&self) -> ffi::GdkWindowState {
        ffi::GdkWindowState::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWindowState> for WindowState {
    fn from_glib(value: ffi::GdkWindowState) -> WindowState {
        skip_assert_initialized!();
        WindowState::from_bits_truncate(value.bits())
    }
}

