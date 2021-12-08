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
            match title_entry.get_text() {
                Some(gstr_title) => {
                    let title: String = String::from(gstr_title);

                    match bin_picker.get_title() {
                        Some(gstr_bin) => {
                            let bin = String::from(gstr_bin);
                            // format
                            let path = format!("/home/{}/.local/share/applications", env::var("USER").unwrap());
                            let mut formatted: String = format!("[Desktop Entry]
Version=1.0
Type=Application
Name={title}
Exec={binary}", title=title, binary=bin);
                            let path = Path::new(&path);
                            let path = &path.join(format!("{}.desktop", title));
                            match File::create(path) {
                                Ok(mut file) => {
                                    if img_picker.get_title().is_some() {
                                        formatted = format!("{}{}", formatted, format!("\nIcon={}", img_picker.get_title().unwrap()));
                                    }
                                    file.write_all(formatted.as_bytes()).expect("err: failed to write to file");
                                },
                                Err(err) => println!("err: failed to create file {}", err)
                            }
                        }
                        None => println!("err: cannot get_title()")
                    }

                },
                None => println!("err: cannot get_text()"),
            }
        });

        Self { widget }
    }
}
