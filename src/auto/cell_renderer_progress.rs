// This file was generated by gir (baa441b) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRenderer;
use Orientable;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellRendererProgress(Object<ffi::GtkCellRendererProgress>): CellRenderer, Orientable;

    match fn {
        get_type => || ffi::gtk_cell_renderer_progress_get_type(),
    }
}

impl CellRendererProgress {
    pub fn new() -> CellRendererProgress {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_progress_new()).downcast_unchecked()
        }
    }
}
