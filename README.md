**Current Version**: 1.0.0
**Author**: Trix Cyrus  
**Copyright**: © 2025 Trixsec Org  
**Maintained**: Yes

# rand_agents

`rand_agents` is a Rust library for generating random user agent strings. It can be used in any Rust project to easily create random user agents for web scraping, testing, or security purposes.

## Features
- Generates random user agents for various browsers, operating systems, devices, and webkit versions.
- Supports a variety of user agents including Mozilla, Opera, and others.

## Installation

To use `rand_agents` in your Rust project, follow these steps:

### 1. Add `rand_agents` to `Cargo.toml`

In your Rust project's `Cargo.toml` file, add the following line under `[dependencies]`:

```toml
[dependencies]
rand_agents = "1.0.0" }
```

### 2. Using `rand_agents` in Your Code

In your `main.rs` (or any other Rust file), add the following code to generate a random user agent:

```rust
use rand_agents::user_agent;

fn main() {
    let user_agent = user_agent();
    println!("Random User Agent: {}", user_agent);
}
```

This will generate and print a random user agent string to the console.

### 3. Running Your Project

To run your project, simply use:

```bash
cargo run
```

You should see a randomly generated user agent string.

## Example Output

```bash
Random User Agent: Opera/9.27 (Raspbian; Apple MacBook Pro) AppleWebKit/619.1.22 (KHTML, like Gecko) RockMelt/75.0.406 Safari/619.1.22
```

