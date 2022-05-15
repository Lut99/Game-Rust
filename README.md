# Game-Rust
Another Vulkan rasterizer implementation, only this time in Rust.

The main goal of this repository is to create some kind of dope game, possibly in minecraft style (with blocks and junk) but with an own twist to it.


## Installation
To install the project, download the appropriate installer binary (`game-setup`) that is associated with your OS and processor architecture.

Then, you may run the setup by opening a terminal and running:
```bash
# Windows
.\game-setup.exe install

# Unix (Linux, macOS)
./game-setup install
```
You will then be taken through a simple terminal UI to guide you through the installation process.


### Compilation from source
Alternatively, you can also compile the framework from source.

First, install [Rust](https://rust-lang.org) and its toolchain (the easiest is by installing [rustup](https://rustup.rs)).

Next, clone the repository and CD into the project:
```bash
git clone https://github.com/Lut99/Game-Rust.git
cd Game-Rust
```

Then, compile the framework by running:
```bash
cargo build --release
```
Finally, you may install the game assets and appropriate file structure by running:
```bash
# Windows
.\target\release\game-setup.exe install --local

# Unix (Linux, macOS)
./target/release/game-setup install --local
```
and follow the simple terminal UI.


## Running
To start the game after you have installed it, simply run:
```bash
# Windows
.\game.exe

# Unix (Linux, macOS)
./game
```
Depending on where you installed the game, you may have to prefix the command with the appropriate folder.


## Contributing
If you have a suggestion, discover an issue or want to contribute, feel free to create the appropriate issues and pull requests in this repository. We will look at them as soon as we can.

