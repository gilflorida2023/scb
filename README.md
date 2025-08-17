scb: Save ClipboardA simple command-line utility written in Rust that saves clipboard content (either text or an image) to a file using a native GTK save dialog.PrerequisitesTo build and run this application, you need to have the following installed:Rust and Cargo: The official Rust toolchain.GTK 3 Development Headers: Required for the graphical file dialog.On a Debian-based Linux distribution (like Ubuntu or Pop!_OS), you can install the GTK headers with:sudo apt-get install libgtk-3-dev
BuildTo compile the project in release mode for an optimized, production-ready binary, navigate to the project root and run:cargo build --release
This will create the executable at target/release/scb.InstallationThe project includes an install.sh script to simplify the installation process. This script automatically detects if you are a regular user or a root user and installs the binary to the correct location.To run the installation script, first, make it executable:chmod +x install.sh
Then, run the script:# For a regular user (installs to ~/.local/bin)
./install.sh

# If you want to install for all users (installs to /usr/local/bin)
sudo ./install.sh
UsageAfter building or installing the application, you can run it from your terminal.# If you built it but didn't install it
./target/release/scb

# If you installed it via the script
scb
The application will check your clipboard for content and open a save dialog if it finds either an image or text.
