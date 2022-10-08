# Development

## Dependencies

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo install cargo-nextest
```


## Running Tests

```bash
cargo nextest run --features "tagged untagged"
cargo nextest run --features "tagged untagged ordered"
cargo nextest run --features "tagged untagged ordered debug"
```


## Coverage

Collect coverage and open `html` report.

```bash
./coverage.sh && cargo coverage_open
```

Collect coverage and output as `lcov`.

```bash
./coverage.sh
```


## Releasing

Update crate versions, then push a tag to the repository. The [`publish`] GitHub workflow will automatically publish the crates to [`crates.io`].

[`publish`]: https://github.com/azriel91/type_reg/actions/workflows/publish.yml
[`crates.io`]:https://crates.io/
