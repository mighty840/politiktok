# Testing

PolitikTok uses Rust's built-in test framework with feature-gated compilation for server and web targets.

## Running Tests

```bash
# Test server-side code
cargo test --features server --no-default-features

# Test web/WASM code
cargo test --features web --no-default-features

# Test with output
cargo test --features server --no-default-features -- --nocapture
```

## Test Organization

Tests are organized alongside the code they test using `#[cfg(test)]` modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_score() {
        let score = compute_match_score(&["rust", "python"], &["rust", "javascript"]);
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
}
```

## Feature-Gated Tests

Since the project has separate `server` and `web` feature sets, tests must be written with the correct feature gates:

```rust
#[cfg(all(test, feature = "server"))]
mod server_tests {
    // Tests that use server-only dependencies (sqlx, reqwest, etc.)
}
```

## CI Integration

The CI pipeline runs both server and web test suites. See the [CI/CD](/deployment/ci-cd) page for details on the GitHub Actions workflow.

## Writing Good Tests

- Test public interfaces, not implementation details
- Use `pretty_assertions` (available in dev-dependencies) for readable diff output
- Mock external services (LLM, database) in unit tests
- Keep tests fast — avoid network calls in unit tests
