HSAPSO is a hybrid metaheuristic optimization framework that combines novel approaches from the Harmony Search Algorithm (HSA) and Particle Swarm Optimization (PSO) for minimum Covering Array (CA) generation in combinatorial software testing. The project is implemented in Rust and focuses on reducing test suite size while maintaining complete t-way interaction coverage.
<br><br>
More information on the subject:
https://ieeexplore.ieee.org/document/9729719

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
