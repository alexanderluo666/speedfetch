# Speedfetch
+ This is speedfetch, used for displaying information and hardware in terminal made using Rust for Linux currently.

## Features

+ Animated RGB gradient logos with motion blur (~30 FPS)
+ **75+ authentic ASCII logos** from [fastfetch](https://github.com/fastfetch-cli/fastfetch) in `src/distro_logos.toml`
+ Per-distro panel themes (logo/label/value colors) for every logo entry
+ Animated gradient presets matched to each distro family (see `src/distro_styles.rs`)
+ Panels: System, Session, Hardware, Display

### Panel fields

| Panel | Fields |
|-------|--------|
| System | OS, Host, Kernel, Arch, Init, Packages |
| Session | User@Host, Shell, Terminal, DE/WM, Uptime, Locale |
| Hardware | CPU, GPU, Memory, Disk |
| Display | Resolution, Font |

Press `Ctrl+C` to exit the animation loop.

## Using the tool

+ Make sure that you are in the speedfetch folder.
+ Make sure that you have cargo and Rust.
+ Run and compile binary from source code:
```bash
cargo run
```
+ Run binary:
```bash
./target/debug/speedfetch
```
+ Clean binary:
```bash
cargo clean
```
+ Build binary from source code:
```bash
cargo build
```
+ Check source code:
```bash
cargo check
```
+ Add as global command:
```bash
echo $PATH
cargo build --release
sudo mv target/release/speedfetch /PATH_IN_ECHO_$PATH
```
+ Test other distros in folder:
```bash
cargo run -- --distro ubuntu
```
+ Test other distros globally:
```bash
speedfetch --distro ubuntu
```

