// A Rust program to save clipboard content to a file.
// It uses the GTK 3 bindings for clipboard access and file dialogs.
//
// To compile and run:
// 1. Ensure you have Rust and Cargo installed.
// 2. Ensure you have the GTK 3 development headers installed (same as for C).
//    sudo apt-get install libgtk-3-dev
// 3. Make sure to have a `Cargo.toml` file in the same directory.
// 4. Run `cargo build` and then `target/debug/scb` or `cargo run` from the command line.

// --- src/main.rs ---
use gtk::prelude::*;
use gdk_pixbuf::Pixbuf;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

// Use gdk-pixbuf-sys for direct C function call
use gdk_pixbuf_sys::gdk_pixbuf_save;

// Main function to handle the clipboard content.
fn save_content() {
    // Get the default clipboard.
    // In this GTK version, it only takes the selection type.
    let clipboard = gtk::Clipboard::get(&gtk::gdk::SELECTION_CLIPBOARD);

    // Check for image data first.
    if let Some(image) = clipboard.wait_for_image() {
        println!("Image data detected. Opening save dialog...");
        // Pass the image to the save function. The type is now correct.
        save_image(&image);
    } else {
        // If no image, check for text data.
        if let Some(text) = clipboard.wait_for_text() {
            println!("Text data detected. Opening save dialog...");
            save_text(&text);
        } else {
            println!("Clipboard is empty or contains an unsupported format.");
        }
    }
}

// Saves text content using a native file chooser dialog.
fn save_text(text: &str) {
    let dialog = gtk::FileChooserNative::new(
        Some("Save Text File"),
        None::<&gtk::Window>,
        gtk::FileChooserAction::Save,
        Some("_Save"),
        Some("_Cancel")
    );

    dialog.set_current_name("clipboard_text.txt");

    let filter = gtk::FileFilter::new();
    filter.set_name(Some("Text Files (*.txt)"));
    filter.add_pattern("*.txt");
    dialog.add_filter(filter);

    if dialog.run() == gtk::ResponseType::Accept {
        if let Some(path) = dialog.file() {
            // Correct way to convert from a Gio File object to a PathBuf.
            let path_buf: PathBuf = path.path().unwrap();
            let mut file = match File::create(&path_buf) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                    return;
                }
            };
            if let Err(e) = file.write_all(text.as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            } else {
                println!("Text successfully saved to: {}", path_buf.display());
            }
        }
    } else {
        println!("Text save canceled.");
    }
}

// Saves image content using a native file chooser dialog.
fn save_image(image: &Pixbuf) {
    let dialog = gtk::FileChooserNative::new(
        Some("Save Image File"),
        None::<&gtk::Window>,
        gtk::FileChooserAction::Save,
        Some("_Save"),
        Some("_Cancel")
    );

    dialog.set_current_name("clipboard_image.png");

    let png_filter = gtk::FileFilter::new();
    png_filter.set_name(Some("PNG Image (*.png)"));
    png_filter.add_mime_type("image/png");
    dialog.add_filter(png_filter);

    let jpeg_filter = gtk::FileFilter::new();
    jpeg_filter.set_name(Some("JPEG Image (*.jpg, *.jpeg)"));
    jpeg_filter.add_mime_type("image/jpeg");
    dialog.add_filter(jpeg_filter);

    if dialog.run() == gtk::ResponseType::Accept {
        if let Some(path) = dialog.file() {
            let path_buf: PathBuf = path.path().unwrap();
            let file_ext = path_buf.extension().and_then(|ext| ext.to_str()).unwrap_or("png");
            
            // Get the C-style representation of the file path and type
            let file_path_cstr = std::ffi::CString::new(path_buf.to_str().unwrap()).unwrap();
            let file_ext_cstr = std::ffi::CString::new(file_ext).unwrap();
            
            // Use unsafe block to call the raw C function
            // This is necessary because we are interacting with a foreign function interface (FFI)
            let result = unsafe {
                gdk_pixbuf_save(
                    image.as_ptr(), // Use as_ptr instead of as_mut_ptr
                    file_path_cstr.as_ptr(),
                    file_ext_cstr.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut::<*mut glib_sys::GError>()
                )
            };

            if result == 0 {
                eprintln!("Error saving image. The gdk_pixbuf_save function failed.");
            } else {
                println!("Image successfully saved to: {}", path_buf.display());
            }
        }
    } else {
        println!("Image save canceled.");
    }
}

fn main() {
    // Create a new GTK Application.
    let application = gtk::Application::new(
        Some("com.example.savecb"),
        Default::default(),
    );

    // When the application is activated, call our main function.
    application.connect_activate(move |_| {
        save_content();
    });

    // Run the application.
    let args: Vec<String> = std::env::args().collect();
    application.run_with_args(&args);
}

