# Code Style

PolitikTok follows strict Rust code style conventions. See [AGENTS.md](https://github.com/mighty840/politiktok/blob/main/AGENTS.md) for the complete coding guidelines.

## Formatting

All code must be formatted with `rustfmt`:

```bash
cargo fmt
cargo fmt --check  # CI check
```

## Linting

All code must pass Clippy without warnings:

```bash
cargo clippy -- -D warnings
```

## Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Functions/variables | `snake_case` | `list_volunteers` |
| Types/traits | `PascalCase` | `VolunteerSummary` |
| Constants | `SCREAMING_SNAKE_CASE` | `SYSTEM_PROMPT` |
| Modules | `snake_case` | `policy_chatbot` |

## Project Patterns

### Server Functions

```rust
#[server(endpoint = "endpoint-name")]
async fn my_function(param: Type) -> Result<ReturnType, ServerFnError> {
    // ...
}
```

### Component Pattern

```rust
#[component]
pub fn MyComponent(
    required_prop: String,
    #[props(default = false)] optional_prop: bool,
) -> Element {
    rsx! {
        // ...
    }
}
```

### State Pattern

```rust
// Newtype wrapper with Arc inner
pub struct MyState(Arc<MyStateInner>);

impl Deref for MyState {
    type Target = MyStateInner;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}
```

## Documentation

All public items must have doc comments. See AGENTS.md for the complete doc comment format.

## Before Committing

- `cargo fmt` — format code
- `cargo clippy -- -D warnings` — lint
- `cargo check --features server` — server build
- `cargo check --features web` — web build
- No `.unwrap()` in production code
- No commented-out code or `dbg!` macros
