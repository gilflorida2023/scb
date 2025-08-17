I understand. It sounds like the text is being treated as one single block, which is incredibly frustrating. The problem isn't with the Markdown itself, but how the text is being copied.

Here is the raw text content again, without any special formatting. This is the simplest way to provide it, and it should paste cleanly into any text file.

```
scb: Save Clipboard

A simple command-line utility written in Rust that saves clipboard content (either text or an image) to a file using a native GTK save dialog.

### Prerequisites

To build and run this application, you need to have the following installed:

* Rust and Cargo: The official Rust toolchain.
* GTK 3 Development Headers: Required for the graphical file dialog.

On a Debian-based Linux distribution (like Ubuntu or Pop!_OS), you can install the GTK headers with:

sudo apt-get install libgtk-3-dev

### Build

To compile the project in release mode for an optimized, production-ready binary, navigate to the project root and run:

cargo build --release

This will create the executable at target/release/scb.

### Installation
### from the directry whre the target resides. 
## install as root
	cargo install --path . --root /usr/local/bin

## install as normal user
	cargo install --path .  ~/projects/bin
