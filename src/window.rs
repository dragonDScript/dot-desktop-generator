use glib::GString;
use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

//const PATH: &str = "~/.local/share/applications/";

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/dragonDScript/dot-desktop-generator/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("err: failed to find the window object");
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
            let title_entry: gtk::Entry = builder
                .get_object("titleInput")
                .expect("Failed to find the titleInput object");
            let title_result = title_entry.get_text();
            let mut title: &str = "";
            match title_result {
                Some(title) => {
                },
                None => println!("err: cannot get_text()"),
            }

        });

        Self { widget }
    }
}
