# PriceMonitor

Runs a Price Monitor on the price of a trading asset based on a Rust engine. 

## How to Run
1. Install the Rust compiler, which can be obtained for your device at <https://www.rust-lang.org/tools/install>.
2. Run the program from the root directory with the command `cargo run -- --currency BTCUSD [--deviation <float>]`

Optionally, the program can be built to an executable by running the command `cargo build`. Then the executable can
also be run with command line arguments as `./target/debug/PriceMonitor --currency BTCUSD [--deviation <float>]`

## WIP
1. More boundary testing/argument validation, runtime testing
2. Response/Error handling from the REST API
3. Implementation of ALL available trading pairs
4. More fun statistics on trading data
5. Unwrap handling

## Approach
1. Look at REST APIs
2. Determine crates for REST, serialization, and command line parsing
3. Build the standard deviation struct, then REST, then JSON serialization, then command line parsing
4. Some simple testing

Took about 2.5 hours of coding, another 0.5 hours of getting Rust and some other tools up/documentation.