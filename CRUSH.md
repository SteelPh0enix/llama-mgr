# CRUSH.md

## Build Commands

- `cargo build` - Build the project

## Test Commands

- `cargo test` - Run all tests
- `cargo test --lib` - Run library tests

## Lint Commands

- `cargo clippy` - Run Clippy linter
- `rustfmt --check` - Check code formatting
- `markdownlint` - Run markdown linting on all markdown files

## Code Style Guidelines

- **Imports**: Use `use` statements for crates. Avoid nested imports.
- **Naming**: Variables: snake_case, Functions: snake_case, Structs: PascalCase.
- **Error Handling**: Prefer `Result<T, E>` with descriptive errors.
- **Formatting**: Use `rustfmt` to auto-format code.

## Testing

- Ensure tests cover all public functions.
- Add `///` comments for public functions.

## Commit Messages

- Use imperative mood (e.g., "Fix login error").
