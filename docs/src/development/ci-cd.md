# CI/CD

PolitikTok uses GitHub Actions for continuous integration and documentation deployment.

## CI Pipeline

The CI workflow (`.github/workflows/ci.yml`) runs on every push to any branch and on pull requests targeting `main`.

### Concurrency

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

Concurrent runs on the same branch are cancelled, keeping CI fast and avoiding resource waste.

### Environment

```yaml
env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"
```

All Rust warnings are promoted to errors, ensuring that no warnings are merged into `main`.

### Jobs

#### 1. Format Check (`fmt`)

```bash
cargo fmt --check
```

Verifies that all Rust code is formatted according to the project's `rustfmt` configuration. This job runs independently and quickly, providing early feedback.

#### 2. Clippy Lint (`clippy`)

Runs Clippy separately for each feature flag:

```bash
# Server feature
cargo clippy --features server --no-default-features -- -D warnings

# Web feature
cargo clippy --features web --no-default-features -- -D warnings
```

This catches lint issues in both server-only and client-only code paths. Uses `Swatinem/rust-cache@v2` for caching compiled dependencies.

#### 3. Security Audit (`audit`)

```bash
rustsec/audit-check@v2.0.0
```

Runs only on the `main` branch. Checks all dependencies against the RustSec Advisory Database for known vulnerabilities.

#### 4. Tests (`test`)

```bash
# Server tests
cargo test --features server --no-default-features

# Web tests
cargo test --features web --no-default-features
```

Runs after both `fmt` and `clippy` pass. Tests are executed separately for server and web features to catch feature-gated compilation issues.

### Job Dependency Graph

```
    fmt ──┐
          ├──> test
  clippy ─┘

  audit (main branch only, independent)
```

### Caching

The `Swatinem/rust-cache@v2` action is used in `clippy` and `test` jobs to cache:

- `~/.cargo/registry` (crate downloads)
- `~/.cargo/git` (git dependencies)
- `target/` (compiled artifacts)

This significantly speeds up subsequent CI runs.

## Documentation Deployment

The docs workflow (`.github/workflows/docs.yml`) deploys this documentation site to GitHub Pages.

### Triggers

- Push to `main` branch when files in `docs/` or the workflow file change
- Manual trigger via `workflow_dispatch`

### Build Process

1. Checkout the repository
2. Install mdBook v0.4.43
3. Build the documentation: `mdbook build docs`
4. Upload the `docs/book/` directory as a GitHub Pages artifact

### Deployment

The deploy job uses `actions/deploy-pages@v4` to publish to the `github-pages` environment. The concurrency group ensures only one deployment runs at a time (no cancellation of in-progress deployments).

### Permissions

```yaml
permissions:
  contents: read
  pages: write
  id-token: write
```

## Running CI Locally

You can replicate the CI checks locally before pushing:

```bash
# Format check
cargo fmt --check

# Clippy (server)
cargo clippy --features server --no-default-features -- -D warnings

# Clippy (web)
cargo clippy --features web --no-default-features -- -D warnings

# Tests (server)
cargo test --features server --no-default-features

# Tests (web)
cargo test --features web --no-default-features

# Security audit (requires cargo-audit)
cargo install cargo-audit
cargo audit
```

## Adding CI Steps

When adding new CI checks:

1. Add the job to `.github/workflows/ci.yml`.
2. Consider whether it should block merging (`needs` dependency) or run independently.
3. Use `Swatinem/rust-cache@v2` for any job that compiles Rust code.
4. Use the same feature flag separation (`--features server` / `--features web`) to match the project's build targets.
