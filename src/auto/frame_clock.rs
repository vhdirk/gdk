// This file was generated by gir (310a2da) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_8")]
use FrameClockPhase;
#[cfg(feature = "v3_8")]
use FrameTimings;
use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct FrameClock(Object<ffi::GdkFrameClock>);

    match fn {
        get_type => || ffi::gdk_frame_clock_get_type(),
    }
}

impl FrameClock {
    #[cfg(feature = "v3_8")]
    pub fn begin_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_begin_updating(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn end_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_end_updating(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_current_timings(&self) -> Option<FrameTimings> {
        unsafe {
            from_glib_full(ffi::gdk_frame_clock_get_current_timings(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_frame_counter(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_clock_get_frame_counter(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_frame_time(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_clock_get_frame_time(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_history_start(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_clock_get_history_start(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_timings(&self, frame_counter: i64) -> Option<FrameTimings> {
        unsafe {
            from_glib_full(ffi::gdk_frame_clock_get_timings(self.to_glib_none().0, frame_counter))
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn request_phase(&self, phase: FrameClockPhase) {
        unsafe {
            ffi::gdk_frame_clock_request_phase(self.to_glib_none().0, phase.to_glib());
        }
    }

    pub fn connect_after_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FrameClock) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "after-paint",
                transmute(after_paint_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_before_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FrameClock) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "before-paint",
                transmute(before_paint_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_flush_events<F: Fn(&FrameClock) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FrameClock) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "flush-events",
                transmute(flush_events_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_layout<F: Fn(&FrameClock) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FrameClock) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "layout",
                transmute(layout_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_paint<F: Fn(&FrameClock) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FrameClock) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "paint",
                transmute(paint_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_resume_events<F: Fn(&FrameClock) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FrameClock) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "resume-events",
                transmute(resume_events_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_update<F: Fn(&FrameClock) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FrameClock) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "update",
                transmute(update_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn after_paint_trampoline(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FrameClock) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn before_paint_trampoline(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FrameClock) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn flush_events_trampoline(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FrameClock) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn layout_trampoline(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FrameClock) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn paint_trampoline(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FrameClock) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn resume_events_trampoline(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FrameClock) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn update_trampoline(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FrameClock) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
