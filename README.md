# Speedfetch
+ This is speedfetch, used for displaying information and hardware in terminal made using Rust for Linux currently.
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
+ Test other distros:
```bash
cargo run -- --distro ubuntu
```