extern crate gtk;

use rust_project1::key_gen;
use gtk::prelude::*;
use gtk::{Button, Entry, TextView, Window, WindowType};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let key_pair: (String, String) = key_gen::initialize_key_pair();

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Encrypt with public key");
    window.set_default_size(350, 70);

    let button = Button::new();
    button.set_label("Encrypt");

    let button_decrypt = Button::new();
    button_decrypt.set_label("Decrypt");

    let private_key = key_pair.0;
    let pub_key = key_pair.1;

    let entry = Rc::new(RefCell::new(Entry::new()));
    entry.borrow().set_text(&pub_key);

    let entry2 = Rc::new(RefCell::new(Entry::new()));
    entry2.borrow().set_text("Insert text to encrypt");

    let entry3 = Rc::new(RefCell::new(Entry::new()));
    entry3.borrow().set_text("Insert text to decrypt");

    let text_view = Rc::new(RefCell::new(TextView::new()));
    let buffer = text_view.borrow().get_buffer().unwrap();

    let entry_clone = entry.clone();
    let entry_clone2 = entry2.clone();
    let buffer_clone = buffer.clone();
    let tx = buffer_clone.clone();

    button.connect_clicked(move |_| {
        let pub_key_string = entry_clone.borrow().get_text().to_string();
        let text_to_encypt = entry_clone2.borrow().get_text().to_string();
        let text_encrypted = key_gen::encrypt_with_rsa_public_key(pub_key_string, text_to_encypt);
        let text = match text_encrypted {
            Ok(text) => text,
            Err(err) => {
                println!("Error: {:?}", err);
                return;
            }
        };
        println!("Encrypted text:\n{}", text.clone());
        tx.set_text(&text);
    });

    let entry_clone3 = entry3.clone();
    let pvt_cloned = private_key.clone();
    let text_view_clone = text_view.clone();

    button_decrypt.connect_clicked(move |_| {
        let text_encrypted = entry_clone3.borrow().get_text().to_string();
        let text_decrypted = key_gen::decrypt_with_rsa_private_key(pvt_cloned.clone(), text_encrypted.clone());
        let text = match text_decrypted {
            Ok(text) => text,
            Err(err) => {
                println!("Error: {:?}", err);
                return;
            }
        };
        println!("Decrypted text:\n{}", text.clone());
        let buffer = text_view_clone.borrow().get_buffer().unwrap();
        buffer.set_text(&text);
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&*entry.borrow());
    vbox.add(&*entry2.borrow());
    vbox.add(&*entry3.borrow());
    vbox.add(&button);
    vbox.add(&button_decrypt);
    vbox.add(&*text_view.borrow());

    window.add(&vbox);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    std::thread::spawn(move || {
        gtk::main();
    });

    gtk::main();
}
