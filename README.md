# muslibger

Your personal music library manager

## Installation
*soon*

## Some instructions

You need cargo to build/test project yourself

### Build:
```bash
cargo build

# or with optimizations
cargo build --release
```
binary will be located under ./target/debug or ./target/release

### Direct run:
```bash
cargo run -- [args]
```

### Run tests:
```bash
 cargo test -- --show-output
```

### Advanced logging:
You can change logging level by specifying RUST_LOG env variable before run or globally in your shell:
```bash
RUST_LOG=level cargo run -- [args]
# or even
RUST_LOG=level muslibger [args]
```
log levels:
* error
* warn
* info
* debug
* trace
* off
