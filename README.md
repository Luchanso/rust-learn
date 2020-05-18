
# install

For working this project you need install nightly rust
```sh
# setup nightly
rustup toolchain install nightly --force
rustup self update
# if you want use always unstable nightly features
rustup default nightly
```

# run

Run application:
```sh
cargo run
```

Run tests:
```sh
cargo test
```

Run benchmarks:
```sh
cargo bench
```

# benchmark result
see in benchmark.snap

# WIP
- [] automation
    - [] CI/CD
        - [] set nightly Rust
        - [] linting
        - [] building
        - [] testing
        - [] benchmarks
    - [] scripts
        - [] build, test, in single command
        - [] benchmarks
- [] algorithm
    - [] Sieve of Atkin
    - [] Sieve of Eratosthenes
- [] tests
    - [] coverage
