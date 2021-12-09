use glib::GString;
use std::path::Path;
use std::env;
use std::io::Write;
use std::fs::File;
use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/dragonDScript/dot-desktop-generator/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("err: failed to find the window object");
        let title_entry: gtk::Entry = builder
            .get_object("titleInput")
            .expect("Failed to find the titleInput object");
        let comment_entry: gtk::Entry = builder
            .get_object("commentInput")
            .expect("Failed to find the commentInput object");
        let img_picker: gtk::FileChooserButton = builder
            .get_object("imgPicker")
            .expect("err: failed to find the imgPicker object");
        let bin_picker: gtk::FileChooserButton = builder
            .get_object("binPicker")
            .expect("err: failed to find the binPicker object");
        let btn_send: gtk::Button = builder
            .get_object("btnSend")
            .expect("err: failed to find the btnSend object");

        btn_send.connect_clicked(move |btn_send| {
            let title = title_entry.get_text()
                .expect("err: cannot get_text()");

            let bin = bin_picker.get_filename().unwrap();

            // format
            let path = format!("/home/{}/.local/share/applications", env::var("USER").unwrap());
            let mut formatted: String = format!("[Desktop Entry]
Version=1.0
Type=Application
Name={title}
Exec={binary}", title=title, binary=bin.to_string_lossy());
            let path = Path::new(&path);
            let path = &path.join(format!("{}.desktop", title));
            match File::create(path) {
                Ok(mut file) => {
                    match img_picker.get_current_folder() {
                        Some (img) => {
                            formatted = format!("{}{}", formatted, format!("\nIcon={}", img.to_string_lossy()));
                        }
                        None => {}
                    }
                    match comment_entry.get_text() {
                        Some (cmt) => {
                            if cmt != "" {
                                formatted = format!("{}{}", formatted, format!("\nComment={}", cmt));
                            }
                        }
                        None => {}
                    }
                    file.write_all(formatted.as_bytes()).expect("err: failed to write to file");
                },
                Err(err) => println!("err: failed to create file {}", err)
            }
        });

        Self { widget }
    }
}
