// This file was generated by gir (b03ccb5) from gir-files (11e0e6d)
// DO NOT EDIT

use JunctionSides;
use RegionFlags;
use StateFlags;
use StateType;
use TextDirection;
use ffi;
use gdk;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct StyleContext(Object<ffi::GtkStyleContext>);

    match fn {
        get_type => || ffi::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    pub fn new() -> StyleContext {
        unsafe {
            from_glib_full(ffi::gtk_style_context_new())
        }
    }

    pub fn add_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_add_class(self.to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    //pub fn add_provider<T: Upcast</*Ignored*/StyleProvider>>(&self, provider: &T, priority: u32) {
    //    unsafe { TODO: call ffi::gtk_style_context_add_provider() }
    //}

    pub fn add_region(&self, region_name: &str, flags: RegionFlags) {
        unsafe {
            ffi::gtk_style_context_add_region(self.to_glib_none().0, region_name.to_glib_none().0, flags);
        }
    }

    //pub fn cancel_animations(&self, region_id: Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_style_context_cancel_animations() }
    //}

    //pub fn get(&self, state: StateFlags, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_context_get() }
    //}

    //pub fn get_background_color(&self, state: StateFlags, color: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_background_color() }
    //}

    //pub fn get_border(&self, state: StateFlags, border: /*Ignored*/Border) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_border() }
    //}

    //pub fn get_border_color(&self, state: StateFlags, color: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_border_color() }
    //}

    //pub fn get_color(&self, state: StateFlags, color: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_color() }
    //}

    pub fn get_direction(&self) -> TextDirection {
        unsafe {
            ffi::gtk_style_context_get_direction(self.to_glib_none().0)
        }
    }

    //pub fn get_font(&self, state: StateFlags) -> /*Ignored*/pango::FontDescription {
    //    unsafe { TODO: call ffi::gtk_style_context_get_font() }
    //}

    //#[cfg(gtk_3_8)]
    //pub fn get_frame_clock(&self) -> /*Ignored*/Option<gdk::FrameClock> {
    //    unsafe { TODO: call ffi::gtk_style_context_get_frame_clock() }
    //}

    pub fn get_junction_sides(&self) -> JunctionSides {
        unsafe {
            ffi::gtk_style_context_get_junction_sides(self.to_glib_none().0)
        }
    }

    //pub fn get_margin(&self, state: StateFlags, margin: /*Ignored*/Border) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_margin() }
    //}

    //pub fn get_padding(&self, state: StateFlags, padding: /*Ignored*/Border) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_padding() }
    //}

    #[cfg(gtk_3_4)]
    pub fn get_parent(&self) -> Option<StyleContext> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_parent(self.to_glib_none().0))
        }
    }

    //pub fn get_path(&self) -> /*Ignored*/WidgetPath {
    //    unsafe { TODO: call ffi::gtk_style_context_get_path() }
    //}

    //pub fn get_property(&self, property: &str, state: StateFlags, value: /*Ignored*/gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_property() }
    //}

    #[cfg(gtk_3_10)]
    pub fn get_scale(&self) -> i32 {
        unsafe {
            ffi::gtk_style_context_get_scale(self.to_glib_none().0)
        }
    }

    pub fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_screen(self.to_glib_none().0))
        }
    }

    //pub fn get_section(&self, property: &str) -> /*Ignored*/CssSection {
    //    unsafe { TODO: call ffi::gtk_style_context_get_section() }
    //}

    pub fn get_state(&self) -> StateFlags {
        unsafe {
            ffi::gtk_style_context_get_state(self.to_glib_none().0)
        }
    }

    //pub fn get_style(&self, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_style() }
    //}

    //pub fn get_style_property(&self, property_name: &str, value: /*Ignored*/&gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_style_property() }
    //}

    //pub fn get_style_valist(&self, args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_style_valist() }
    //}

    //pub fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_valist() }
    //}

    pub fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_style_context_has_class(self.to_glib_none().0, class_name.to_glib_none().0))
        }
    }

    pub fn has_region(&self, region_name: &str) -> Option<RegionFlags> {
        unsafe {
            let mut flags_return = mem::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_has_region(self.to_glib_none().0, region_name.to_glib_none().0, &mut flags_return));
            if ret { Some(flags_return) } else { None }
        }
    }

    pub fn invalidate(&self) {
        unsafe {
            ffi::gtk_style_context_invalidate(self.to_glib_none().0);
        }
    }

    //pub fn lookup_color(&self, color_name: &str, color: /*Ignored*/gdk::RGBA) -> bool {
    //    unsafe { TODO: call ffi::gtk_style_context_lookup_color() }
    //}

    //pub fn lookup_icon_set(&self, stock_id: &str) -> /*Ignored*/IconSet {
    //    unsafe { TODO: call ffi::gtk_style_context_lookup_icon_set() }
    //}

    //pub fn notify_state_change(&self, window: &gdk::Window, region_id: Option<Fundamental: Pointer>, state: StateType, state_value: bool) {
    //    unsafe { TODO: call ffi::gtk_style_context_notify_state_change() }
    //}

    pub fn pop_animatable_region(&self) {
        unsafe {
            ffi::gtk_style_context_pop_animatable_region(self.to_glib_none().0);
        }
    }

    //pub fn push_animatable_region(&self, region_id: Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_style_context_push_animatable_region() }
    //}

    pub fn remove_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_class(self.to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    //pub fn remove_provider<T: Upcast</*Ignored*/StyleProvider>>(&self, provider: &T) {
    //    unsafe { TODO: call ffi::gtk_style_context_remove_provider() }
    //}

    pub fn remove_region(&self, region_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_region(self.to_glib_none().0, region_name.to_glib_none().0);
        }
    }

    pub fn restore(&self) {
        unsafe {
            ffi::gtk_style_context_restore(self.to_glib_none().0);
        }
    }

    pub fn save(&self) {
        unsafe {
            ffi::gtk_style_context_save(self.to_glib_none().0);
        }
    }

    pub fn scroll_animations(&self, window: &gdk::Window, dx: i32, dy: i32) {
        unsafe {
            ffi::gtk_style_context_scroll_animations(self.to_glib_none().0, window.to_glib_none().0, dx, dy);
        }
    }

    pub fn set_background(&self, window: &gdk::Window) {
        unsafe {
            ffi::gtk_style_context_set_background(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    pub fn set_direction(&self, direction: TextDirection) {
        unsafe {
            ffi::gtk_style_context_set_direction(self.to_glib_none().0, direction);
        }
    }

    //#[cfg(gtk_3_8)]
    //pub fn set_frame_clock(&self, frame_clock: /*Ignored*/&gdk::FrameClock) {
    //    unsafe { TODO: call ffi::gtk_style_context_set_frame_clock() }
    //}

    pub fn set_junction_sides(&self, sides: JunctionSides) {
        unsafe {
            ffi::gtk_style_context_set_junction_sides(self.to_glib_none().0, sides);
        }
    }

    #[cfg(gtk_3_4)]
    pub fn set_parent(&self, parent: Option<&StyleContext>) {
        unsafe {
            ffi::gtk_style_context_set_parent(self.to_glib_none().0, parent.to_glib_none().0);
        }
    }

    //pub fn set_path(&self, path: /*Ignored*/&WidgetPath) {
    //    unsafe { TODO: call ffi::gtk_style_context_set_path() }
    //}

    #[cfg(gtk_3_10)]
    pub fn set_scale(&self, scale: i32) {
        unsafe {
            ffi::gtk_style_context_set_scale(self.to_glib_none().0, scale);
        }
    }

    pub fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_style_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    pub fn set_state(&self, flags: StateFlags) {
        unsafe {
            ffi::gtk_style_context_set_state(self.to_glib_none().0, flags);
        }
    }

    pub fn state_is_running(&self, state: StateType) -> Option<f64> {
        unsafe {
            let mut progress = mem::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_state_is_running(self.to_glib_none().0, state, &mut progress));
            if ret { Some(progress) } else { None }
        }
    }

    //pub fn add_provider_for_screen<T: Upcast</*Ignored*/StyleProvider>>(screen: &gdk::Screen, provider: &T, priority: u32) {
    //    unsafe { TODO: call ffi::gtk_style_context_add_provider_for_screen() }
    //}

    //pub fn remove_provider_for_screen<T: Upcast</*Ignored*/StyleProvider>>(screen: &gdk::Screen, provider: &T) {
    //    unsafe { TODO: call ffi::gtk_style_context_remove_provider_for_screen() }
    //}

    pub fn reset_widgets(screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_style_context_reset_widgets(screen.to_glib_none().0);
        }
    }

}
