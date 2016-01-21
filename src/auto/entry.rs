// This file was generated by gir (baa441b) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use CellEditable;
use Editable;
use EntryBuffer;
use EntryCompletion;
use EntryIconPosition;
use ImageType;
#[cfg(gtk_3_6)]
use InputHints;
#[cfg(gtk_3_6)]
use InputPurpose;
use Rectangle;
use Widget;
use ffi;
use gdk_pixbuf;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Entry(Object<ffi::GtkEntry>): Widget, Buildable, CellEditable, Editable;

    match fn {
        get_type => || ffi::gtk_entry_get_type(),
    }
}

impl Entry {
    pub fn new() -> Entry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_entry_new()).downcast_unchecked()
        }
    }

    pub fn new_with_buffer(buffer: &EntryBuffer) -> Entry {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_entry_new_with_buffer(buffer.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait EntryExt {
    fn get_activates_default(&self) -> bool;

    fn get_alignment(&self) -> f32;

    //#[cfg(gtk_3_6)]
    //fn get_attributes(&self) -> /*Ignored*/pango::AttrList;

    fn get_buffer(&self) -> EntryBuffer;

    fn get_completion(&self) -> Option<EntryCompletion>;

    fn get_current_icon_drag_source(&self) -> i32;

    fn get_cursor_hadjustment(&self) -> Option<Adjustment>;

    fn get_has_frame(&self) -> bool;

    fn get_icon_activatable(&self, icon_pos: EntryIconPosition) -> bool;

    fn get_icon_area(&self, icon_pos: EntryIconPosition) -> Rectangle;

    fn get_icon_at_pos(&self, x: i32, y: i32) -> i32;

    //fn get_icon_gicon(&self, icon_pos: EntryIconPosition) -> /*Ignored*/Option<gio::Icon>;

    fn get_icon_name(&self, icon_pos: EntryIconPosition) -> Option<String>;

    fn get_icon_pixbuf(&self, icon_pos: EntryIconPosition) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_icon_sensitive(&self, icon_pos: EntryIconPosition) -> bool;

    fn get_icon_stock(&self, icon_pos: EntryIconPosition) -> Option<String>;

    fn get_icon_storage_type(&self, icon_pos: EntryIconPosition) -> ImageType;

    fn get_icon_tooltip_markup(&self, icon_pos: EntryIconPosition) -> Option<String>;

    fn get_icon_tooltip_text(&self, icon_pos: EntryIconPosition) -> Option<String>;

    //fn get_inner_border(&self) -> /*Ignored*/Option<Border>;

    #[cfg(gtk_3_6)]
    fn get_input_hints(&self) -> InputHints;

    #[cfg(gtk_3_6)]
    fn get_input_purpose(&self) -> InputPurpose;

    fn get_invisible_char(&self) -> Option<char>;

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout>;

    fn get_layout_offsets(&self) -> (i32, i32);

    fn get_max_length(&self) -> i32;

    #[cfg(gtk_3_12)]
    fn get_max_width_chars(&self) -> i32;

    fn get_overwrite_mode(&self) -> bool;

    fn get_placeholder_text(&self) -> Option<String>;

    fn get_progress_fraction(&self) -> f64;

    fn get_progress_pulse_step(&self) -> f64;

    //#[cfg(gtk_3_10)]
    //fn get_tabs(&self) -> /*Ignored*/Option<pango::TabArray>;

    fn get_text(&self) -> Option<String>;

    fn get_text_area(&self) -> Rectangle;

    fn get_text_length(&self) -> u16;

    fn get_visibility(&self) -> bool;

    fn get_width_chars(&self) -> i32;

    #[cfg(gtk_3_16)]
    fn grab_focus_without_selecting(&self);

    //fn im_context_filter_keypress(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool;

    fn layout_index_to_text_index(&self, layout_index: i32) -> i32;

    fn progress_pulse(&self);

    fn reset_im_context(&self);

    fn set_activates_default(&self, setting: bool);

    fn set_alignment(&self, xalign: f32);

    //#[cfg(gtk_3_6)]
    //fn set_attributes(&self, attrs: /*Ignored*/&pango::AttrList);

    fn set_buffer(&self, buffer: &EntryBuffer);

    fn set_completion(&self, completion: Option<&EntryCompletion>);

    fn set_cursor_hadjustment(&self, adjustment: &Adjustment);

    fn set_has_frame(&self, setting: bool);

    fn set_icon_activatable(&self, icon_pos: EntryIconPosition, activatable: bool);

    //fn set_icon_drag_source(&self, icon_pos: EntryIconPosition, target_list: /*Ignored*/&TargetList, actions: gdk::DragAction);

    //fn set_icon_from_gicon<T: IsA</*Ignored*/gio::Icon>>(&self, icon_pos: EntryIconPosition, icon: Option<&T>);

    fn set_icon_from_icon_name(&self, icon_pos: EntryIconPosition, icon_name: Option<&str>);

    fn set_icon_from_pixbuf(&self, icon_pos: EntryIconPosition, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn set_icon_from_stock(&self, icon_pos: EntryIconPosition, stock_id: Option<&str>);

    fn set_icon_sensitive(&self, icon_pos: EntryIconPosition, sensitive: bool);

    fn set_icon_tooltip_markup(&self, icon_pos: EntryIconPosition, tooltip: Option<&str>);

    fn set_icon_tooltip_text(&self, icon_pos: EntryIconPosition, tooltip: Option<&str>);

    //fn set_inner_border(&self, border: /*Ignored*/Option<&Border>);

    #[cfg(gtk_3_6)]
    fn set_input_hints(&self, hints: InputHints);

    #[cfg(gtk_3_6)]
    fn set_input_purpose(&self, purpose: InputPurpose);

    fn set_invisible_char(&self, ch: Option<char>);

    fn set_max_length(&self, max: i32);

    #[cfg(gtk_3_12)]
    fn set_max_width_chars(&self, n_chars: i32);

    fn set_overwrite_mode(&self, overwrite: bool);

    fn set_placeholder_text(&self, text: Option<&str>);

    fn set_progress_fraction(&self, fraction: f64);

    fn set_progress_pulse_step(&self, fraction: f64);

    //#[cfg(gtk_3_10)]
    //fn set_tabs(&self, tabs: /*Ignored*/&mut pango::TabArray);

    fn set_text(&self, text: &str);

    fn set_visibility(&self, visible: bool);

    fn set_width_chars(&self, n_chars: i32);

    fn text_index_to_layout_index(&self, text_index: i32) -> i32;

    fn unset_invisible_char(&self);
}

impl<O: IsA<Entry>> EntryExt for O {
    fn get_activates_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_get_activates_default(self.to_glib_none().0))
        }
    }

    fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_entry_get_alignment(self.to_glib_none().0)
        }
    }

    //#[cfg(gtk_3_6)]
    //fn get_attributes(&self) -> /*Ignored*/pango::AttrList {
    //    unsafe { TODO: call ffi::gtk_entry_get_attributes() }
    //}

    fn get_buffer(&self) -> EntryBuffer {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_buffer(self.to_glib_none().0))
        }
    }

    fn get_completion(&self) -> Option<EntryCompletion> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_completion(self.to_glib_none().0))
        }
    }

    fn get_current_icon_drag_source(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_current_icon_drag_source(self.to_glib_none().0)
        }
    }

    fn get_cursor_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_cursor_hadjustment(self.to_glib_none().0))
        }
    }

    fn get_has_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_get_has_frame(self.to_glib_none().0))
        }
    }

    fn get_icon_activatable(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_get_icon_activatable(self.to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_area(&self, icon_pos: EntryIconPosition) -> Rectangle {
        unsafe {
            let mut icon_area = Rectangle::uninitialized();
            ffi::gtk_entry_get_icon_area(self.to_glib_none().0, icon_pos, icon_area.to_glib_none_mut().0);
            icon_area
        }
    }

    fn get_icon_at_pos(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_get_icon_at_pos(self.to_glib_none().0, x, y)
        }
    }

    //fn get_icon_gicon(&self, icon_pos: EntryIconPosition) -> /*Ignored*/Option<gio::Icon> {
    //    unsafe { TODO: call ffi::gtk_entry_get_icon_gicon() }
    //}

    fn get_icon_name(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_icon_name(self.to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_pixbuf(&self, icon_pos: EntryIconPosition) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_icon_pixbuf(self.to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_sensitive(&self, icon_pos: EntryIconPosition) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_get_icon_sensitive(self.to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_stock(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_icon_stock(self.to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_storage_type(&self, icon_pos: EntryIconPosition) -> ImageType {
        unsafe {
            ffi::gtk_entry_get_icon_storage_type(self.to_glib_none().0, icon_pos)
        }
    }

    fn get_icon_tooltip_markup(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_entry_get_icon_tooltip_markup(self.to_glib_none().0, icon_pos))
        }
    }

    fn get_icon_tooltip_text(&self, icon_pos: EntryIconPosition) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_entry_get_icon_tooltip_text(self.to_glib_none().0, icon_pos))
        }
    }

    //fn get_inner_border(&self) -> /*Ignored*/Option<Border> {
    //    unsafe { TODO: call ffi::gtk_entry_get_inner_border() }
    //}

    #[cfg(gtk_3_6)]
    fn get_input_hints(&self) -> InputHints {
        unsafe {
            ffi::gtk_entry_get_input_hints(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    fn get_input_purpose(&self) -> InputPurpose {
        unsafe {
            ffi::gtk_entry_get_input_purpose(self.to_glib_none().0)
        }
    }

    fn get_invisible_char(&self) -> Option<char> {
        unsafe {
            from_glib(ffi::gtk_entry_get_invisible_char(self.to_glib_none().0))
        }
    }

    //fn get_layout(&self) -> /*Ignored*/Option<pango::Layout> {
    //    unsafe { TODO: call ffi::gtk_entry_get_layout() }
    //}

    fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gtk_entry_get_layout_offsets(self.to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    fn get_max_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_max_length(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_12)]
    fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_max_width_chars(self.to_glib_none().0)
        }
    }

    fn get_overwrite_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_get_overwrite_mode(self.to_glib_none().0))
        }
    }

    fn get_placeholder_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_placeholder_text(self.to_glib_none().0))
        }
    }

    fn get_progress_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_fraction(self.to_glib_none().0)
        }
    }

    fn get_progress_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_entry_get_progress_pulse_step(self.to_glib_none().0)
        }
    }

    //#[cfg(gtk_3_10)]
    //fn get_tabs(&self) -> /*Ignored*/Option<pango::TabArray> {
    //    unsafe { TODO: call ffi::gtk_entry_get_tabs() }
    //}

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_get_text(self.to_glib_none().0))
        }
    }

    fn get_text_area(&self) -> Rectangle {
        unsafe {
            let mut text_area = Rectangle::uninitialized();
            ffi::gtk_entry_get_text_area(self.to_glib_none().0, text_area.to_glib_none_mut().0);
            text_area
        }
    }

    fn get_text_length(&self) -> u16 {
        unsafe {
            ffi::gtk_entry_get_text_length(self.to_glib_none().0)
        }
    }

    fn get_visibility(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_get_visibility(self.to_glib_none().0))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_get_width_chars(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_16)]
    fn grab_focus_without_selecting(&self) {
        unsafe {
            ffi::gtk_entry_grab_focus_without_selecting(self.to_glib_none().0);
        }
    }

    //fn im_context_filter_keypress(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool {
    //    unsafe { TODO: call ffi::gtk_entry_im_context_filter_keypress() }
    //}

    fn layout_index_to_text_index(&self, layout_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_layout_index_to_text_index(self.to_glib_none().0, layout_index)
        }
    }

    fn progress_pulse(&self) {
        unsafe {
            ffi::gtk_entry_progress_pulse(self.to_glib_none().0);
        }
    }

    fn reset_im_context(&self) {
        unsafe {
            ffi::gtk_entry_reset_im_context(self.to_glib_none().0);
        }
    }

    fn set_activates_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_entry_set_activates_default(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_alignment(&self, xalign: f32) {
        unsafe {
            ffi::gtk_entry_set_alignment(self.to_glib_none().0, xalign);
        }
    }

    //#[cfg(gtk_3_6)]
    //fn set_attributes(&self, attrs: /*Ignored*/&pango::AttrList) {
    //    unsafe { TODO: call ffi::gtk_entry_set_attributes() }
    //}

    fn set_buffer(&self, buffer: &EntryBuffer) {
        unsafe {
            ffi::gtk_entry_set_buffer(self.to_glib_none().0, buffer.to_glib_none().0);
        }
    }

    fn set_completion(&self, completion: Option<&EntryCompletion>) {
        unsafe {
            ffi::gtk_entry_set_completion(self.to_glib_none().0, completion.to_glib_none().0);
        }
    }

    fn set_cursor_hadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_entry_set_cursor_hadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    fn set_has_frame(&self, setting: bool) {
        unsafe {
            ffi::gtk_entry_set_has_frame(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_icon_activatable(&self, icon_pos: EntryIconPosition, activatable: bool) {
        unsafe {
            ffi::gtk_entry_set_icon_activatable(self.to_glib_none().0, icon_pos, activatable.to_glib());
        }
    }

    //fn set_icon_drag_source(&self, icon_pos: EntryIconPosition, target_list: /*Ignored*/&TargetList, actions: gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_entry_set_icon_drag_source() }
    //}

    //fn set_icon_from_gicon<T: IsA</*Ignored*/gio::Icon>>(&self, icon_pos: EntryIconPosition, icon: Option<&T>) {
    //    unsafe { TODO: call ffi::gtk_entry_set_icon_from_gicon() }
    //}

    fn set_icon_from_icon_name(&self, icon_pos: EntryIconPosition, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_entry_set_icon_from_icon_name(self.to_glib_none().0, icon_pos, icon_name.to_glib_none().0);
        }
    }

    fn set_icon_from_pixbuf(&self, icon_pos: EntryIconPosition, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_entry_set_icon_from_pixbuf(self.to_glib_none().0, icon_pos, pixbuf.to_glib_none().0);
        }
    }

    fn set_icon_from_stock(&self, icon_pos: EntryIconPosition, stock_id: Option<&str>) {
        unsafe {
            ffi::gtk_entry_set_icon_from_stock(self.to_glib_none().0, icon_pos, stock_id.to_glib_none().0);
        }
    }

    fn set_icon_sensitive(&self, icon_pos: EntryIconPosition, sensitive: bool) {
        unsafe {
            ffi::gtk_entry_set_icon_sensitive(self.to_glib_none().0, icon_pos, sensitive.to_glib());
        }
    }

    fn set_icon_tooltip_markup(&self, icon_pos: EntryIconPosition, tooltip: Option<&str>) {
        unsafe {
            ffi::gtk_entry_set_icon_tooltip_markup(self.to_glib_none().0, icon_pos, tooltip.to_glib_none().0);
        }
    }

    fn set_icon_tooltip_text(&self, icon_pos: EntryIconPosition, tooltip: Option<&str>) {
        unsafe {
            ffi::gtk_entry_set_icon_tooltip_text(self.to_glib_none().0, icon_pos, tooltip.to_glib_none().0);
        }
    }

    //fn set_inner_border(&self, border: /*Ignored*/Option<&Border>) {
    //    unsafe { TODO: call ffi::gtk_entry_set_inner_border() }
    //}

    #[cfg(gtk_3_6)]
    fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            ffi::gtk_entry_set_input_hints(self.to_glib_none().0, hints);
        }
    }

    #[cfg(gtk_3_6)]
    fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            ffi::gtk_entry_set_input_purpose(self.to_glib_none().0, purpose);
        }
    }

    fn set_invisible_char(&self, ch: Option<char>) {
        unsafe {
            ffi::gtk_entry_set_invisible_char(self.to_glib_none().0, ch.to_glib());
        }
    }

    fn set_max_length(&self, max: i32) {
        unsafe {
            ffi::gtk_entry_set_max_length(self.to_glib_none().0, max);
        }
    }

    #[cfg(gtk_3_12)]
    fn set_max_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_entry_set_max_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    fn set_overwrite_mode(&self, overwrite: bool) {
        unsafe {
            ffi::gtk_entry_set_overwrite_mode(self.to_glib_none().0, overwrite.to_glib());
        }
    }

    fn set_placeholder_text(&self, text: Option<&str>) {
        unsafe {
            ffi::gtk_entry_set_placeholder_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_progress_fraction(&self, fraction: f64) {
        unsafe {
            ffi::gtk_entry_set_progress_fraction(self.to_glib_none().0, fraction);
        }
    }

    fn set_progress_pulse_step(&self, fraction: f64) {
        unsafe {
            ffi::gtk_entry_set_progress_pulse_step(self.to_glib_none().0, fraction);
        }
    }

    //#[cfg(gtk_3_10)]
    //fn set_tabs(&self, tabs: /*Ignored*/&mut pango::TabArray) {
    //    unsafe { TODO: call ffi::gtk_entry_set_tabs() }
    //}

    fn set_text(&self, text: &str) {
        unsafe {
            ffi::gtk_entry_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_visibility(&self, visible: bool) {
        unsafe {
            ffi::gtk_entry_set_visibility(self.to_glib_none().0, visible.to_glib());
        }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_entry_set_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    fn text_index_to_layout_index(&self, text_index: i32) -> i32 {
        unsafe {
            ffi::gtk_entry_text_index_to_layout_index(self.to_glib_none().0, text_index)
        }
    }

    fn unset_invisible_char(&self) {
        unsafe {
            ffi::gtk_entry_unset_invisible_char(self.to_glib_none().0);
        }
    }
}
