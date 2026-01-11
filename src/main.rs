mod window;

use gtk::prelude::*;
use window::DesktopFilesCreatorWindow;

fn main() {
    let app = libadwaita::Application::builder()
        .application_id("com.github.cylin577.desktop-files-creator")
        .build();

    app.connect_activate(|app| {
        let window = DesktopFilesCreatorWindow::new(app);
        // Fuck Mutter WM
        window.set_default_size(509, 503);
        window.present();
    });

    app.run();
}
