use gtk::prelude::*;
use gtk::{glib, Box, Button, Entry, Label, ListBox, Orientation, ScrolledWindow, TextView, CheckButton, ListBoxRow};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::data::{Note, TodoItem, AppData};

pub struct NotesView {
    widget: Box,
    app_data: Rc<RefCell<AppData>>,
    notes_list: ListBox,
    note_entry: Entry,
    note_text: TextView,
    current_note_id: Rc<RefCell<Option<Uuid>>>,
}

impl NotesView {
    pub fn new(app_data: Rc<RefCell<AppData>>) -> Self {
        let widget = Box::new(Orientation::Horizontal, 12);
        widget.set_margin_start(12);
        widget.set_margin_end(12);
        widget.set_margin_top(12);
        widget.set_margin_bottom(12);

        // Left panel for notes list
        let left_panel = Box::new(Orientation::Vertical, 6);
        left_panel.set_hexpand(false);
        left_panel.set_width_request(300);

        let notes_header = Box::new(Orientation::Horizontal, 6);
        let notes_label = Label::new(Some("Notes"));
        notes_label.set_markup("<b>Notes</b>");
        let add_note_btn = Button::with_label("New Note");
        notes_header.append(&notes_label);
        notes_header.append(&add_note_btn);

        let notes_list = ListBox::new();
        notes_list.set_selection_mode(gtk::SelectionMode::Single);
        let notes_scrolled = ScrolledWindow::new();
        notes_scrolled.set_child(Some(&notes_list));
        notes_scrolled.set_vexpand(true);

        left_panel.append(&notes_header);
        left_panel.append(&notes_scrolled);

        // Right panel for note editing
        let right_panel = Box::new(Orientation::Vertical, 6);
        right_panel.set_hexpand(true);

        let note_entry = Entry::new();
        note_entry.set_placeholder_text(Some("Note title..."));

        let note_text = TextView::new();
        note_text.set_wrap_mode(gtk::WrapMode::Word);
        let text_scrolled = ScrolledWindow::new();
        text_scrolled.set_child(Some(&note_text));
        text_scrolled.set_vexpand(true);

        let save_btn = Button::with_label("Save Note");

        right_panel.append(&note_entry);
        right_panel.append(&text_scrolled);
        right_panel.append(&save_btn);

        widget.append(&left_panel);
        widget.append(&right_panel);

        let current_note_id = Rc::new(RefCell::new(None));

        let view = Self {
            widget,
            app_data: app_data.clone(),
            notes_list,
            note_entry,
            note_text,
            current_note_id,
        };

        // Connect signals
        let app_data_clone = app_data.clone();
        let notes_list_clone = view.notes_list.clone();
        let note_entry_clone = view.note_entry.clone();
        let note_text_clone = view.note_text.clone();
        let current_note_id_clone = view.current_note_id.clone();

        add_note_btn.connect_clicked(move |_| {
            let note = Note::new("New Note".to_string(), String::new());
            let note_id = note.id;
            app_data_clone.borrow_mut().add_note(note);
            
            refresh_notes_list(&notes_list_clone, &app_data_clone);
            
            // Select the new note
            *current_note_id_clone.borrow_mut() = Some(note_id);
            if let Some(note) = app_data_clone.borrow().notes.iter().find(|n| n.id == note_id) {
                note_entry_clone.set_text(&note.title);
                note_text_clone.buffer().set_text(&note.content);
            }
        });

        let app_data_clone = app_data.clone();
        let note_entry_clone = view.note_entry.clone();
        let note_text_clone = view.note_text.clone();
        let current_note_id_clone = view.current_note_id.clone();

        save_btn.connect_clicked(move |_| {
            if let Some(note_id) = *current_note_id_clone.borrow() {
                let title = note_entry_clone.text().to_string();
                let buffer = note_text_clone.buffer();
                let content = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false).to_string();
                
                if let Some(note) = app_data_clone.borrow_mut().notes.iter_mut().find(|n| n.id == note_id) {
                    note.update(title, content);
                }
                app_data_clone.borrow().save();
            }
        });

        let app_data_clone = app_data.clone();
        let note_entry_clone = view.note_entry.clone();
        let note_text_clone = view.note_text.clone();
        let current_note_id_clone = view.current_note_id.clone();

        view.notes_list.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                if let Some(note_id) = row.child().and_then(|w| {
                    w.downcast::<Box>().ok().and_then(|b| {
                        b.first_child().and_then(|l| {
                            l.downcast::<Label>().ok().and_then(|label| {
                                Uuid::parse_str(&label.widget_name()).ok()
                            })
                        })
                    })
                }) {
                    *current_note_id_clone.borrow_mut() = Some(note_id);
                    if let Some(note) = app_data_clone.borrow().notes.iter().find(|n| n.id == note_id) {
                        note_entry_clone.set_text(&note.title);
                        note_text_clone.buffer().set_text(&note.content);
                    }
                }
            }
        });

        // Initial population
        refresh_notes_list(&view.notes_list, &app_data);

        view
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}

pub struct TodoView {
    widget: Box,
    app_data: Rc<RefCell<AppData>>,
    todos_list: ListBox,
    todo_entry: Entry,
}

impl TodoView {
    pub fn new(app_data: Rc<RefCell<AppData>>) -> Self {
        let widget = Box::new(Orientation::Vertical, 12);
        widget.set_margin_start(12);
        widget.set_margin_end(12);
        widget.set_margin_top(12);
        widget.set_margin_bottom(12);

        // Header
        let header = Box::new(Orientation::Horizontal, 6);
        let todo_entry = Entry::new();
        todo_entry.set_placeholder_text(Some("Add a new todo..."));
        todo_entry.set_hexpand(true);
        let add_btn = Button::with_label("Add");

        header.append(&todo_entry);
        header.append(&add_btn);

        // Todos list
        let todos_list = ListBox::new();
        let todos_scrolled = ScrolledWindow::new();
        todos_scrolled.set_child(Some(&todos_list));
        todos_scrolled.set_vexpand(true);

        widget.append(&header);
        widget.append(&todos_scrolled);

        let view = Self {
            widget,
            app_data: app_data.clone(),
            todos_list,
            todo_entry,
        };

        // Connect signals
        let app_data_clone = app_data.clone();
        let todos_list_clone = view.todos_list.clone();
        let todo_entry_clone = view.todo_entry.clone();

        let add_todo = move || {
            let text = todo_entry_clone.text().to_string();
            if !text.trim().is_empty() {
                let todo = TodoItem::new(text);
                app_data_clone.borrow_mut().add_todo(todo);
                todo_entry_clone.set_text("");
                refresh_todos_list(&todos_list_clone, &app_data_clone);
                app_data_clone.borrow().save();
            }
        };

        let add_todo_clone = add_todo.clone();
        add_btn.connect_clicked(move |_| add_todo_clone());

        view.todo_entry.connect_activate(move |_| add_todo());

        // Initial population
        refresh_todos_list(&view.todos_list, &app_data);

        view
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}

fn refresh_notes_list(notes_list: &ListBox, app_data: &Rc<RefCell<AppData>>) {
    // Clear existing items
    while let Some(child) = notes_list.first_child() {
        notes_list.remove(&child);
    }

    // Add notes
    for note in &app_data.borrow().notes {
        let row = ListBoxRow::new();
        let note_box = Box::new(Orientation::Vertical, 3);
        note_box.set_margin_start(6);
        note_box.set_margin_end(6);
        note_box.set_margin_top(6);
        note_box.set_margin_bottom(6);

        let title_label = Label::new(Some(&note.title));
        title_label.set_widget_name(&note.id.to_string());
        title_label.set_halign(gtk::Align::Start);
        title_label.set_markup(&format!("<b>{}</b>", glib::markup_escape_text(&note.title)));

        let date_label = Label::new(Some(&note.updated_at.format("%Y-%m-%d %H:%M").to_string()));
        date_label.set_halign(gtk::Align::Start);
        date_label.add_css_class("dim-label");

        note_box.append(&title_label);
        note_box.append(&date_label);
        row.set_child(Some(&note_box));
        notes_list.append(&row);
    }
}

fn refresh_todos_list(todos_list: &ListBox, app_data: &Rc<RefCell<AppData>>) {
    // Clear existing items
    while let Some(child) = todos_list.first_child() {
        todos_list.remove(&child);
    }

    // Add todos
    let app_data_clone = app_data.clone();
    for todo in &app_data.borrow().todos {
        let row = ListBoxRow::new();
        let todo_box = Box::new(Orientation::Horizontal, 6);
        todo_box.set_margin_start(6);
        todo_box.set_margin_end(6);
        todo_box.set_margin_top(6);
        todo_box.set_margin_bottom(6);

        let checkbox = CheckButton::new();
        checkbox.set_active(todo.completed);

        let text_label = Label::new(Some(&todo.text));
        text_label.set_halign(gtk::Align::Start);
        text_label.set_hexpand(true);

        if todo.completed {
            text_label.add_css_class("dim-label");
        }

        let delete_btn = Button::with_label("Delete");
        delete_btn.add_css_class("destructive-action");

        todo_box.append(&checkbox);
        todo_box.append(&text_label);
        todo_box.append(&delete_btn);

        let todo_id = todo.id;
        let app_data_clone2 = app_data_clone.clone();
        let todos_list_clone = todos_list.clone();
        checkbox.connect_toggled(move |_| {
            app_data_clone2.borrow_mut().toggle_todo(&todo_id);
            refresh_todos_list(&todos_list_clone, &app_data_clone2);
            app_data_clone2.borrow().save();
        });

        let app_data_clone3 = app_data_clone.clone();
        let todos_list_clone2 = todos_list.clone();
        delete_btn.connect_clicked(move |_| {
            app_data_clone3.borrow_mut().remove_todo(&todo_id);
            refresh_todos_list(&todos_list_clone2, &app_data_clone3);
            app_data_clone3.borrow().save();
        });

        row.set_child(Some(&todo_box));
        todos_list.append(&row);
    }
}
