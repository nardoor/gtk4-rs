use gio::Settings;
use glib::signal::Inhibit;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{CompositeTemplate, Entry, ListView, MenuButton};
use std::cell::RefCell;

use crate::task_object::TaskObject;

// Object holding the state
#[derive(CompositeTemplate)]
#[template(resource = "/org/gtk-rs/Todo3/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub menu_button: TemplateChild<MenuButton>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub current_tasks: RefCell<Option<gio::ListStore>>,
    pub settings: Settings,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            entry: TemplateChild::default(),
            menu_button: TemplateChild::default(),
            tasks_list: TemplateChild::default(),
            current_tasks: RefCell::default(),
            settings: Settings::new("org.gtk-rs.Todo3"),
        }
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Setup
        obj.setup_tasks();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_factory();
        obj.setup_actions();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {
    fn close_request(&self, window: &Self::Type) -> Inhibit {
        // Store todo data in vector
        let mut tasks = Vec::new();
        let mut position = 0;
        while let Some(item) = window.current_tasks().item(position) {
            // Get `TaskObject` from `glib::Object`
            let task_data = item
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.")
                .task_data();
            // Add todo data to vector and increase position
            tasks.push(task_data);
            position += 1;
        }

        self.settings
            .set("tasks", &tasks)
            .expect("Could not save data in settings.");

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
