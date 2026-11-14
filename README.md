# Usage

## Setup

```bash
rustup toolchain install
```

## Formatting

```bash
cargo fmt --all --check
```

## Linting

```bash
cargo clippy --locked -- -D warnings
```

## Testing

```bash
cargo test --locked
```

## Build

```bash
cargo build --locked --release
```

## Verification

```bash
 gh attestation verify [PATH_TO_DOWNLOADED_BUNDLE] --repo mczul/rust-playground
```

## Installation

### MacOS

Due to security policies downloaded binaries are marked as downloaded and the OS refuses to execute them if they are not signed by Apple. To remove this "downloaded" flag, simply execute 

```bash
xattr -d com.apple.quarantine [PATH_TO_BINARY]
```

