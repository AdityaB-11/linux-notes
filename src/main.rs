use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Orientation, HeaderBar, Stack, StackSwitcher};
use std::cell::RefCell;
use std::rc::Rc;

mod data;
mod ui;

use data::AppData;
use ui::{NotesView, TodoView};

const APP_ID: &str = "com.example.LinuxNotes";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    // Initialize libadwaita
    adw::init().unwrap();

    let app_data = Rc::new(RefCell::new(AppData::load()));

    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Linux Notes")
        .default_width(800)
        .default_height(600)
        .build();

    // Create header bar
    let header_bar = HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    // Create main container
    let main_box = Box::new(Orientation::Vertical, 0);

    // Create stack for switching between notes and todos
    let stack = Stack::new();
    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    header_bar.set_title_widget(Some(&stack_switcher));

    // Create notes view
    let notes_view = NotesView::new(app_data.clone());
    stack.add_titled(notes_view.widget(), Some("notes"), "Notes");

    // Create todo view
    let todo_view = TodoView::new(app_data.clone());
    stack.add_titled(todo_view.widget(), Some("todos"), "To-Do");

    main_box.append(&stack);
    window.set_child(Some(&main_box));

    // Save data when window is closed
    let app_data_clone = app_data.clone();
    window.connect_close_request(move |_| {
        app_data_clone.borrow().save();
        glib::Propagation::Proceed
    });

    window.present();
}
