// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::BaselinePosition;
use crate::LayoutManager;
use crate::Orientation;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkCenterLayout")]
    pub struct CenterLayout(Object<ffi::GtkCenterLayout, ffi::GtkCenterLayoutClass>) @extends LayoutManager;

    match fn {
        type_ => || ffi::gtk_center_layout_get_type(),
    }
}

impl CenterLayout {
    #[doc(alias = "gtk_center_layout_new")]
    pub fn new() -> CenterLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(ffi::gtk_center_layout_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_center_layout_get_baseline_position")]
    #[doc(alias = "get_baseline_position")]
    pub fn baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_center_layout_get_baseline_position(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_get_center_widget")]
    #[doc(alias = "get_center_widget")]
    pub fn center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_center_layout_get_center_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_get_end_widget")]
    #[doc(alias = "get_end_widget")]
    pub fn end_widget(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_center_layout_get_end_widget(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_center_layout_get_orientation")]
    #[doc(alias = "get_orientation")]
    pub fn orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_center_layout_get_orientation(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_get_start_widget")]
    #[doc(alias = "get_start_widget")]
    pub fn start_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_center_layout_get_start_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_center_layout_set_baseline_position")]
    pub fn set_baseline_position(&self, baseline_position: BaselinePosition) {
        unsafe {
            ffi::gtk_center_layout_set_baseline_position(
                self.to_glib_none().0,
                baseline_position.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_center_layout_set_center_widget")]
    pub fn set_center_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_center_layout_set_center_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_center_layout_set_end_widget")]
    pub fn set_end_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_center_layout_set_end_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_center_layout_set_orientation")]
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_center_layout_set_orientation(self.to_glib_none().0, orientation.into_glib());
        }
    }

    #[doc(alias = "gtk_center_layout_set_start_widget")]
    pub fn set_start_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_center_layout_set_start_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl Default for CenterLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CenterLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CenterLayout")
    }
}
