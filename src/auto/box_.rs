// This file was generated by gir (b03ccb5) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(gtk_3_10)]
use BaselinePosition;
use Buildable;
use Container;
use Orientable;
use Orientation;
use PackType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Box(Object<ffi::GtkBox>): Widget, Container, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_box_get_type(),
    }
}

impl Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Box {
        unsafe {
            Widget::from_glib_none(ffi::gtk_box_new(orientation, spacing)).downcast_unchecked()
        }
    }

}

pub trait BoxExt {
    #[cfg(gtk_3_10)]
    fn get_baseline_position(&self) -> BaselinePosition;
    #[cfg(gtk_3_12)]
    fn get_center_widget(&self) -> Option<Widget>;
    fn get_homogeneous(&self) -> bool;
    fn get_spacing(&self) -> i32;
    fn pack_end<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32);
    fn pack_start<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32);
    fn query_child_packing<T: Upcast<Widget>>(&self, child: &T) -> (bool, bool, u32, PackType);
    fn reorder_child<T: Upcast<Widget>>(&self, child: &T, position: i32);
    #[cfg(gtk_3_10)]
    fn set_baseline_position(&self, position: BaselinePosition);
    #[cfg(gtk_3_12)]
    fn set_center_widget<T: Upcast<Widget>>(&self, widget: Option<&T>);
    fn set_child_packing<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32, pack_type: PackType);
    fn set_homogeneous(&self, homogeneous: bool);
    fn set_spacing(&self, spacing: i32);
}

impl<O: Upcast<Box>> BoxExt for O {
    #[cfg(gtk_3_10)]
    fn get_baseline_position(&self) -> BaselinePosition {
        unsafe {
            ffi::gtk_box_get_baseline_position(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_12)]
    fn get_center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_box_get_center_widget(self.to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_box_get_homogeneous(self.to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_box_get_spacing(self.to_glib_none().0)
        }
    }

    fn pack_end<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_end(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding);
        }
    }

    fn pack_start<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_start(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding);
        }
    }

    fn query_child_packing<T: Upcast<Widget>>(&self, child: &T) -> (bool, bool, u32, PackType) {
        unsafe {
            let mut expand = mem::uninitialized();
            let mut fill = mem::uninitialized();
            let mut padding = mem::uninitialized();
            let mut pack_type = mem::uninitialized();
            ffi::gtk_box_query_child_packing(self.to_glib_none().0, child.to_glib_none().0, &mut expand, &mut fill, &mut padding, &mut pack_type);
            (from_glib(expand), from_glib(fill), padding, pack_type)
        }
    }

    fn reorder_child<T: Upcast<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_box_reorder_child(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(gtk_3_10)]
    fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_set_baseline_position(self.to_glib_none().0, position);
        }
    }

    #[cfg(gtk_3_12)]
    fn set_center_widget<T: Upcast<Widget>>(&self, widget: Option<&T>) {
        unsafe {
            ffi::gtk_box_set_center_widget(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    fn set_child_packing<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32, pack_type: PackType) {
        unsafe {
            ffi::gtk_box_set_child_packing(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding, pack_type);
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

}
