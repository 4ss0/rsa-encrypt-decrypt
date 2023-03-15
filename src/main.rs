extern crate gtk;
use rust_project1::key_gen;
use gtk::prelude::*;
use gtk::{Button, Entry, Window, WindowType};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello, World!");
    window.set_default_size(350, 70);

    let button = Button::new();
    button.set_label("Click me!");

    let pub_key = key_gen::pub_key().to_owned();

    let entry = Rc::new(RefCell::new(Entry::new()));
    entry.borrow().set_text(&pub_key);

    let entry2 = Rc::new(RefCell::new(Entry::new()));
    entry2.borrow().set_text("Insert text here");

    let entry_clone = entry.clone();
    let entry_clone2 = entry2.clone();
    button.connect_clicked(move |_| {
        let text = entry_clone.borrow().get_text().to_string();
        let text2 = entry_clone2.borrow().get_text().to_string();
        println!("Button clicked with text: {}", text);
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&*entry.borrow());
    vbox.add(&*entry2.borrow());
    vbox.add(&button);

    window.add(&vbox);

    window.show_all();

    gtk::main();
}
