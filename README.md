Generating minimum CA by combining novel approaches to Harmony Search Algorithm and PSO implemented in Rust.<br>
More info on the subject matter: https://ieeexplore.ieee.org/document/9729719
### How to Compile & Run:
- Install rust (Linux or macOS):<br>
```$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh```
<br><i>or visit this page for other platforms: https://www.rust-lang.org/tools/install</i>
- Run command:<br>
```cargo run --release```

### How to Profile:
- Add this to `Cargo.toml` for more detailed profiling:<br>
```
[profile.release] 
debug = true
```
- Run command:<br>
```cargo install flamegraph```
- Start the program with this command:<br>
```cargo flamegraph```
- check this file in the root directory:<br>
```
flamegraph.svg
```
