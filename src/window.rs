use glib::clone;
use glib::subclass::InitializingObject;
use gtk::{gio, glib, CompositeTemplate};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libadwaita::subclass::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(file = "../resources/window.ui")]
    pub struct DesktopFilesCreatorWindow {
        #[template_child]
        pub entry_name: TemplateChild<gtk::Entry>,
        #[template_child]
        pub clear_name: TemplateChild<gtk::Button>,
        #[template_child]
        pub entry_exec: TemplateChild<gtk::Entry>,
        #[template_child]
        pub button_exec: TemplateChild<gtk::Button>,
        #[template_child]
        pub clear_exec: TemplateChild<gtk::Button>,
        #[template_child]
        pub entry_icon: TemplateChild<gtk::Entry>,
        #[template_child]
        pub button_icon: TemplateChild<gtk::Button>,
        #[template_child]
        pub clear_icon: TemplateChild<gtk::Button>,
        #[template_child]
        pub entry_categories: TemplateChild<gtk::Entry>,
        #[template_child]
        pub clear_categories: TemplateChild<gtk::Button>,
        #[template_child]
        pub box_categories_list: TemplateChild<gtk::Box>,
        #[template_child]
        pub popover_categories: TemplateChild<gtk::Popover>,
        #[template_child]
        pub entry_comment: TemplateChild<gtk::Entry>,
        #[template_child]
        pub clear_comment: TemplateChild<gtk::Button>,
        #[template_child]
        pub switch_no_display: TemplateChild<gtk::Switch>,
        #[template_child]
        pub switch_terminal: TemplateChild<gtk::Switch>,
        #[template_child]
        pub button_import: TemplateChild<gtk::Button>,
        #[template_child]
        pub button_open: TemplateChild<gtk::Button>,
        #[template_child]
        pub button_create: TemplateChild<gtk::Button>,
        #[template_child]
        pub overlay: TemplateChild<libadwaita::ToastOverlay>,

        pub directory_path: RefCell<PathBuf>,
        pub active_file_chooser: RefCell<Option<gtk::FileChooserNative>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DesktopFilesCreatorWindow {
        const NAME: &'static str = "DesktopFilesCreatorWindow";
        type Type = super::DesktopFilesCreatorWindow;
        type ParentType = libadwaita::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DesktopFilesCreatorWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_default_size(631, 623);

            // Set directory path
            let home_dir = glib::home_dir();
            let mut path = PathBuf::from(home_dir);
            path.push(".local/share/applications");
            *self.directory_path.borrow_mut() = path.clone();

            if !path.exists() {
                obj.alert(
                    "Error!",
                    &format!("Path: {:?} does not exist!\nThe program will not be able to perform its functions.", path)
                );
                self.button_create.set_sensitive(false);
                self.button_open.set_sensitive(false);
            }

            // Connect signals
            
            // Name
            self.entry_name.connect_changed(clone!(@weak obj => move |entry| {
                obj.on_entry_change(entry, &obj.imp().clear_name);
            }));
            self.clear_name.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_clear_entry(&obj.imp().entry_name);
            }));

            // Exec
            self.button_exec.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_open_exec();
            }));
            self.entry_exec.connect_changed(clone!(@weak obj => move |entry| {
                obj.on_entry_change(entry, &obj.imp().clear_exec);
            }));
            self.clear_exec.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_clear_entry(&obj.imp().entry_exec);
            }));

            // Icon
            self.button_icon.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_open_icon();
            }));
            self.entry_icon.connect_changed(clone!(@weak obj => move |entry| {
                obj.on_entry_change(entry, &obj.imp().clear_icon);
            }));
            self.clear_icon.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_clear_entry(&obj.imp().entry_icon);
            }));

            // Categories
            self.entry_categories.connect_changed(clone!(@weak obj => move |entry| {
                obj.on_entry_change(entry, &obj.imp().clear_categories);
            }));
            self.clear_categories.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_clear_entry(&obj.imp().entry_categories);
            }));

            // Setup Categories List
            obj.setup_categories();

            // Comment
            self.entry_comment.connect_changed(clone!(@weak obj => move |entry| {
                obj.on_entry_change(entry, &obj.imp().clear_comment);
            }));
            self.clear_comment.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_clear_entry(&obj.imp().entry_comment);
            }));

            // Import
            self.button_import.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_import_desktop_file();
            }));

            // Drag and Drop
            let drop_target = gtk::DropTarget::new(gio::File::static_type(), gtk::gdk::DragAction::COPY);
            drop_target.connect_drop(clone!(@weak obj => @default-return false, move |_, value, _, _| {
                if let Ok(file) = value.get::<gio::File>() {
                    if let Some(path) = file.path() {
                        obj.load_from_file(&path);
                        return true;
                    }
                }
                false
            }));
            obj.add_controller(drop_target);

            // Bottom Buttons
            self.button_open.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_open_directory();
            }));
            self.button_create.connect_clicked(clone!(@weak obj => move |_| {
                obj.on_create_file_check();
            }));
        }
    }

    impl WidgetImpl for DesktopFilesCreatorWindow {}
    impl WindowImpl for DesktopFilesCreatorWindow {}
    impl ApplicationWindowImpl for DesktopFilesCreatorWindow {}
    impl AdwApplicationWindowImpl for DesktopFilesCreatorWindow {}
}

glib::wrapper! {
    pub struct DesktopFilesCreatorWindow(ObjectSubclass<imp::DesktopFilesCreatorWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, libadwaita::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl DesktopFilesCreatorWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn on_import_desktop_file(&self) {
        let file_chooser = gtk::FileChooserNative::new(
            Some("Import Desktop File"),
            Some(self),
            gtk::FileChooserAction::Open,
            Some("_Open"),
            Some("_Cancel"),
        );
        file_chooser.set_modal(true);

        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.desktop");
        filter.set_name(Some("Desktop Files"));
        file_chooser.add_filter(&filter);

        file_chooser.connect_response(clone!(@weak self as obj, @weak file_chooser => move |_, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = file_chooser.file() {
                    if let Some(path) = file.path() {
                        obj.load_from_file(&path);
                    }
                }
            }
            obj.imp().active_file_chooser.replace(None);
        }));

        self.imp().active_file_chooser.replace(Some(file_chooser.clone()));
        file_chooser.show();
    }

    fn load_from_file(&self, path: &std::path::Path) {
        let key_file = glib::KeyFile::new();
        if let Err(e) = key_file.load_from_file(path, glib::KeyFileFlags::NONE) {
            self.alert("Error loading file", &e.to_string());
            return;
        }

        let group = "Desktop Entry";
        let imp = self.imp();

        if let Ok(name) = key_file.string(group, "Name") {
            imp.entry_name.set_text(&name);
        }
        if let Ok(exec) = key_file.string(group, "Exec") {
            imp.entry_exec.set_text(&exec);
        }
        if let Ok(icon) = key_file.string(group, "Icon") {
            imp.entry_icon.set_text(&icon);
        }
        if let Ok(categories) = key_file.string(group, "Categories") {
            imp.entry_categories.set_text(&categories);
        }
        if let Ok(comment) = key_file.string(group, "Comment") {
            imp.entry_comment.set_text(&comment);
        }
        
        if let Ok(no_display) = key_file.boolean(group, "NoDisplay") {
            imp.switch_no_display.set_active(no_display);
        } else {
             imp.switch_no_display.set_active(false);
        }

        if let Ok(terminal) = key_file.boolean(group, "Terminal") {
            imp.switch_terminal.set_active(terminal);
        } else {
             imp.switch_terminal.set_active(false);
        }

        self.set_toast("File loaded successfully");
    }

    fn setup_categories(&self) {
        let categories = [
            "AudioVideo", "Audio", "Video", "Development", "Education", "Game", 
            "Graphics", "Network", "Office", "Science", "Settings", "System", "Utility"
        ];

        let imp = self.imp();
        let box_list = &imp.box_categories_list;

        for category in categories.iter() {
            let check_btn = gtk::CheckButton::builder()
                .label(*category)
                .build();
            
            check_btn.connect_toggled(clone!(@weak self as obj, @strong category => move |btn| {
                obj.on_category_toggled(btn, category);
            }));

            box_list.append(&check_btn);
        }

        imp.popover_categories.connect_visible_notify(clone!(@weak self as obj => move |popover| {
            if popover.is_visible() {
                obj.sync_categories_checkboxes();
            }
        }));
    }

    fn on_category_toggled(&self, btn: &gtk::CheckButton, category: &str) {
        // Prevent recursive updates when syncing
        if !btn.is_sensitive() { return; }

        let imp = self.imp();
        let current_text = imp.entry_categories.text();
        let mut parts: Vec<String> = current_text.split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if btn.is_active() {
            if !parts.contains(&category.to_string()) {
                parts.push(category.to_string());
            }
        } else {
            if let Some(pos) = parts.iter().position(|x| x == category) {
                parts.remove(pos);
            }
        }

        let new_text = parts.join(";");
        // Add trailing semicolon if not empty, common in desktop files
        let final_text = if new_text.is_empty() { String::new() } else { new_text + ";" };
        
        imp.entry_categories.set_text(&final_text);
    }

    fn sync_categories_checkboxes(&self) {
        let imp = self.imp();
        let current_text = imp.entry_categories.text();
        let parts: Vec<&str> = current_text.split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut child = imp.box_categories_list.first_child();
        while let Some(widget) = child {
            let next = widget.next_sibling();
            if let Ok(check_btn) = widget.downcast::<gtk::CheckButton>() {
                if let Some(label) = check_btn.label() {
                    let is_present = parts.contains(&label.as_str());
                    
                    if check_btn.is_active() != is_present {
                        check_btn.set_active(is_present);
                    }
                }
            }
            child = next;
        }
    }

    fn on_clear_entry(&self, entry: &gtk::Entry) {
        entry.set_text("");
        entry.grab_focus();
    }

    fn on_entry_change(&self, entry: &gtk::Entry, clear: &gtk::Button) {
        let text = entry.text();
        let is_empty = text.trim().is_empty();
        
        clear.set_visible(!is_empty);

        let imp = self.imp();
        if entry == &*imp.entry_exec {
            imp.button_exec.set_visible(is_empty);
        } else if entry == &*imp.entry_icon {
            imp.button_icon.set_visible(is_empty);
        }
    }

    fn on_open_exec(&self) {
        let file_chooser = gtk::FileChooserNative::new(
            Some("Open File"),
            Some(self),
            gtk::FileChooserAction::Open,
            Some("_Open"),
            Some("_Cancel"),
        );
        file_chooser.set_modal(true);
        
        file_chooser.connect_response(clone!(@weak self as obj, @weak file_chooser => move |_, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = file_chooser.file() {
                    if let Some(path) = file.path() {
                        obj.imp().entry_exec.set_text(path.to_string_lossy().as_ref());
                    }
                }
            }
            obj.imp().active_file_chooser.replace(None);
        }));
        
        self.imp().active_file_chooser.replace(Some(file_chooser.clone()));
        file_chooser.show();
    }

    fn on_open_icon(&self) {
        let file_chooser = gtk::FileChooserNative::new(
            Some("Open Image"),
            Some(self),
            gtk::FileChooserAction::Open,
            Some("_Open"),
            Some("_Cancel"),
        );
        file_chooser.set_modal(true);

        let filter = gtk::FileFilter::new();
        filter.add_mime_type("image/jpeg");
        filter.add_mime_type("image/png");
        filter.add_mime_type("image/svg+xml");
        filter.add_mime_type("image/x-xpixmap");
        filter.add_mime_type("image/vnd.microsoft.icon");
        file_chooser.set_filter(&filter);
        
        file_chooser.connect_response(clone!(@weak self as obj, @weak file_chooser => move |_, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = file_chooser.file() {
                    if let Some(path) = file.path() {
                        obj.imp().entry_icon.set_text(path.to_string_lossy().as_ref());
                    }
                }
            }
            obj.imp().active_file_chooser.replace(None);
        }));

        self.imp().active_file_chooser.replace(Some(file_chooser.clone()));
        file_chooser.show();
    }

    fn on_open_directory(&self) {
        let path = self.imp().directory_path.borrow();
        let uri = format!("file://{}", path.to_string_lossy());
        
        if let Some(window) = self.root().and_then(|r| r.downcast::<gtk::Window>().ok()) {
             let _ = gtk::show_uri(Some(&window), &uri, 0);
        }
    }

    fn on_create_file_check(&self) {
        let imp = self.imp();
        let name = imp.entry_name.text();
        
        if name.trim().is_empty() {
            self.set_toast("Enter the name");
            imp.entry_name.grab_focus();
            return;
        }

        let mut path = imp.directory_path.borrow().clone();
        path.push(format!("{}.desktop", name.trim()));

        if path.exists() {
            self.alert("Error!", "A file with the same name already exists");
            imp.entry_name.grab_focus();
            return;
        }

        let dialog = gtk::MessageDialog::builder()
            .text(&format!("Create file {}?", path.file_name().unwrap().to_string_lossy()))
            .modal(true)
            .transient_for(self)
            .buttons(gtk::ButtonsType::OkCancel)
            .build();
        
        dialog.connect_response(clone!(@weak self as obj => move |d, response| {
            if response == gtk::ResponseType::Ok {
                obj.create_desktop_file();
            }
            d.close();
        }));
        
        dialog.present();
    }

    fn create_desktop_file(&self) {
        let imp = self.imp();
        
        let display = if imp.switch_no_display.is_active() { "true" } else { "false" };
        let terminal = if imp.switch_terminal.is_active() { "true" } else { "false" };
        
        let content = format!("[Desktop Entry]\nType=Application\nNoDisplay={}\nTerminal={}\nExec={}\nIcon={}\nName={}\nComment={}\nCategories={}",
            display,
            terminal,
            imp.entry_exec.text().trim(),
            imp.entry_icon.text().trim(),
            imp.entry_name.text().trim(),
            imp.entry_comment.text().trim(),
            imp.entry_categories.text().trim()
        );

        let mut path = imp.directory_path.borrow().clone();
        path.push(format!("{}.desktop", imp.entry_name.text().trim()));

        match std::fs::write(&path, content) {
            Ok(_) => {
                self.alert("File Created!", &format!("Path: {:?}", path));
            },
            Err(e) => {
                self.alert("Error!", &format!("Could not create file: {}", e));
            }
        }
    }

    fn set_toast(&self, text: &str) {
        let toast = libadwaita::Toast::new(text);
        toast.set_timeout(3);
        self.imp().overlay.add_toast(toast);
    }

    fn alert(&self, heading: &str, body: &str) {
        let dialog = gtk::MessageDialog::builder()
            .text(heading)
            .secondary_text(body)
            .modal(true)
            .transient_for(self)
            .buttons(gtk::ButtonsType::Ok)
            .build();
        
        dialog.connect_response(|d, _| d.close());
        dialog.present();
    }
}