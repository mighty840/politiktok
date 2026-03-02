# Pull Requests

Guidelines for submitting pull requests to PolitikTok.

## Branch Naming

Use conventional prefixes:

```
feat/volunteer-churn-prediction
fix/sidebar-mobile-toggle
docs/self-hosting-guide
refactor/llm-client-retry
```

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add volunteer churn prediction model
fix: sidebar not closing on mobile navigation
docs: add self-hosting reverse proxy guide
refactor: extract LLM client retry logic
test: add sentiment monitor unit tests
```

## PR Template

```markdown
## Summary

Brief description of what this PR does.

## Changes

- List of specific changes

## Test Plan

- How to test the changes
- Any edge cases considered

## Screenshots

(If UI changes)
```

## Review Checklist

Before requesting review, ensure:

- [ ] `cargo fmt` — code is formatted
- [ ] `cargo clippy -- -D warnings` — no lint warnings
- [ ] `cargo check --features server` — server build passes
- [ ] `cargo check --features web` — web build passes
- [ ] No `.unwrap()` in production code paths
- [ ] Public items have doc comments
- [ ] No credentials or secrets in code

## Adding a New Module

When adding a new feature module:

1. Create `src/modules/<module_name>/mod.rs` with server functions
2. Create `src/pages/<module_name>/mod.rs` with UI components
3. Add models in `src/models/<module_name>.rs`
4. Register the route in `src/app.rs`
5. Add the sidebar entry in `src/components/sidebar.rs`
6. Add module documentation in `docs/modules/<module_name>.md`
7. Add database migrations if needed
