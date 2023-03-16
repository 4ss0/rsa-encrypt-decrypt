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
    
    let key_pair: (String, String) = key_gen::initialize_key_pair();

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello, World!");
    window.set_default_size(350, 70);

    let button = Button::new();
    button.set_label("Encrypt");

    // Get Public Key
    let private_key = key_pair.0;
    let pub_key = key_pair.1;

    println!("{}", private_key);

    let entry = Rc::new(RefCell::new(Entry::new()));
    entry.borrow().set_text(&pub_key);

    let entry2 = Rc::new(RefCell::new(Entry::new()));
    entry2.borrow().set_text("Insert text to encrypt");

    let (tx, rx) = std::sync::mpsc::channel();

    let entry_clone = entry.clone();
    let entry_clone2 = entry2.clone();

    button.connect_clicked(move |_| {
        let pub_key_string = entry_clone.borrow().get_text().to_string();
        let text_to_encypt = entry_clone2.borrow().get_text().to_string();
        let text_encrypted = key_gen::encrypt_with_rsa_public_key(pub_key_string, text_to_encypt);
        println!("Encrypted text:\n{}", text_encrypted.clone().unwrap());
        tx.send(text_encrypted.unwrap()).unwrap();
    });
    std::thread::spawn(move || {
        let text = rx.recv().unwrap();
        let text_decrypted = key_gen::decrypt_with_rsa_private_key(private_key.clone(), text.clone());
        println!("Decrypted text:\n{}", text_decrypted.clone().unwrap());
    });

   
    

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&*entry.borrow());
    vbox.add(&*entry2.borrow());
    vbox.add(&button);

    window.add(&vbox);

    window.show_all();

    gtk::main();
}
