# Contributing

Thank you for your interest in contributing to PolitikTok. This guide covers the development workflow from fork to merged pull request.

## Getting Started

1. **Fork** the repository on GitHub.
2. **Clone** your fork locally:

   ```bash
   git clone https://github.com/YOUR_USERNAME/politiktok.git
   cd politiktok
   ```

3. **Set up** the development environment following the [Installation](../getting-started/installation.md) guide.
4. **Create a branch** for your work:

   ```bash
   git checkout -b feat/your-feature-name
   ```

## Branch Naming

Use prefixed branch names that describe the type of change:

| Prefix | Use Case | Example |
|--------|----------|---------|
| `feat/` | New features | `feat/f03-spike-notifications` |
| `fix/` | Bug fixes | `fix/volunteer-churn-score` |
| `refactor/` | Code refactoring | `refactor/llm-client-retry` |
| `docs/` | Documentation | `docs/add-f07-fundraising` |
| `test/` | Test additions | `test/embedding-chunking` |
| `chore/` | Tooling, CI, dependencies | `chore/update-dioxus-073` |

## Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation changes |
| `refactor` | Code refactoring (no behavior change) |
| `test` | Adding or updating tests |
| `chore` | Tooling, CI, build system changes |
| `perf` | Performance improvements |

### Scope

Use the module name or infrastructure area:

```
feat(volunteer-matching): add skill-weighted scoring
fix(policy-chatbot): handle empty document content
refactor(llm): extract retry logic into helper
docs(architecture): add RAG pipeline diagram
```

### Examples

```
feat(sentiment-monitor): add spike detection with configurable threshold

Implements statistical anomaly detection for sentiment volume spikes.
A spike is flagged when the current window count exceeds the 7-day
historical average by a configurable number of standard deviations.

Closes #42
```

## Code Style

### Rust Formatting

All code must pass `cargo fmt --check`. The project uses default `rustfmt` settings. Format your code before committing:

```bash
cargo fmt
```

### Clippy Lints

The project enforces strict Clippy lints:

```bash
cargo clippy --features server --no-default-features -- -D warnings
cargo clippy --features web --no-default-features -- -D warnings
```

The workspace-level Clippy configuration denies `unwrap_used` and `expect_used`:

```toml
[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
```

Use proper error handling (`?`, `map_err`, `ok_or_else`) instead of `.unwrap()` or `.expect()`.

## Pull Request Process

1. **Push** your branch to your fork.
2. **Open a pull request** against the `main` branch.
3. **Fill in** the PR template with:
   - A description of the changes
   - Which module(s) are affected
   - Testing steps
   - Screenshots (for UI changes)
4. **CI must pass**: format check, Clippy (server + web), and tests.
5. **Code review**: At least one maintainer must approve.
6. **Squash and merge**: PRs are squash-merged to keep the main branch history clean.

## Adding a New Module

If you are adding a new feature module:

1. Create a new directory under `src/modules/your_module/mod.rs`.
2. Register the module in `src/modules/mod.rs`.
3. Add models in `src/models/` if needed.
4. Create page components in `src/pages/your_module/mod.rs`.
5. Add routes to the `Route` enum in `src/app.rs`.
6. Add the sidebar link in `src/components/sidebar.rs`.
7. Write documentation in `docs/src/modules/your-module.md`.
8. Add the entry to `docs/src/SUMMARY.md`.

## Development Tips

- Run `dx serve` for hot-reloading during development.
- Use `tracing::info!`, `tracing::warn!`, etc. for logging (not `println!`).
- Server functions should always authenticate the user via `require_user()` or `require_role()`.
- Database queries use raw SQL via `sqlx::query` -- avoid building SQL strings dynamically.
- LLM prompts should be defined as `const &str` for readability and reuse.
- Keep server functions focused -- one operation per function, delegate complex logic to helper functions.
