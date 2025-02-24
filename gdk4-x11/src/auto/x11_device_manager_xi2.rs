// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkX11DeviceManagerXI2")]
    pub struct X11DeviceManagerXI2(Object<ffi::GdkX11DeviceManagerXI2, ffi::GdkX11DeviceManagerXI2Class>);

    match fn {
        type_ => || ffi::gdk_x11_device_manager_xi2_get_type(),
    }
}

impl X11DeviceManagerXI2 {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`X11DeviceManagerXI2`] objects.
    ///
    /// This method returns an instance of [`X11DeviceManagerXI2Builder`](crate::builders::X11DeviceManagerXI2Builder) which can be used to create [`X11DeviceManagerXI2`] objects.
    pub fn builder() -> X11DeviceManagerXI2Builder {
        X11DeviceManagerXI2Builder::default()
    }

    pub fn display(&self) -> Option<gdk::Display> {
        glib::ObjectExt::property(self, "display")
    }

    pub fn major(&self) -> i32 {
        glib::ObjectExt::property(self, "major")
    }

    pub fn minor(&self) -> i32 {
        glib::ObjectExt::property(self, "minor")
    }

    pub fn opcode(&self) -> i32 {
        glib::ObjectExt::property(self, "opcode")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`X11DeviceManagerXI2`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct X11DeviceManagerXI2Builder {
    display: Option<gdk::Display>,
    major: Option<i32>,
    minor: Option<i32>,
    opcode: Option<i32>,
}

impl X11DeviceManagerXI2Builder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`X11DeviceManagerXI2Builder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`X11DeviceManagerXI2`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> X11DeviceManagerXI2 {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref major) = self.major {
            properties.push(("major", major));
        }
        if let Some(ref minor) = self.minor {
            properties.push(("minor", minor));
        }
        if let Some(ref opcode) = self.opcode {
            properties.push(("opcode", opcode));
        }
        glib::Object::new::<X11DeviceManagerXI2>(&properties)
    }

    pub fn display(mut self, display: &impl IsA<gdk::Display>) -> Self {
        self.display = Some(display.clone().upcast());
        self
    }

    pub fn major(mut self, major: i32) -> Self {
        self.major = Some(major);
        self
    }

    pub fn minor(mut self, minor: i32) -> Self {
        self.minor = Some(minor);
        self
    }

    pub fn opcode(mut self, opcode: i32) -> Self {
        self.opcode = Some(opcode);
        self
    }
}

impl fmt::Display for X11DeviceManagerXI2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11DeviceManagerXI2")
    }
}
