// This file was generated by gir (b03ccb5) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use ToggleButton;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CheckButton(Object<ffi::GtkCheckButton>): Widget, Container, Bin, Button, ToggleButton, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_check_button_get_type(),
    }
}

impl CheckButton {
    pub fn new() -> CheckButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> CheckButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> CheckButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_button_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }

}

pub trait CheckButtonExt {
}

impl<O: Upcast<CheckButton>> CheckButtonExt for O {
}
