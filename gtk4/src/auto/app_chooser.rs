// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkAppChooser")]
    pub struct AppChooser(Interface<ffi::GtkAppChooser>) @requires Widget, Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_app_chooser_get_type(),
    }
}

impl AppChooser {
    pub const NONE: Option<&'static AppChooser> = None;
}

pub trait AppChooserExt: 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_get_app_info")]
    #[doc(alias = "get_app_info")]
    fn app_info(&self) -> Option<gio::AppInfo>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_get_content_type")]
    #[doc(alias = "get_content_type")]
    fn content_type(&self) -> glib::GString;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_app_chooser_refresh")]
    fn refresh(&self);
}

impl<O: IsA<AppChooser>> AppChooserExt for O {
    fn app_info(&self) -> Option<gio::AppInfo> {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_app_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn content_type(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_app_chooser_get_content_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn refresh(&self) {
        unsafe {
            ffi::gtk_app_chooser_refresh(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for AppChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppChooser")
    }
}
