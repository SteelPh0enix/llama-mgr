# CRUSH.md - Codebase Guidelines

## Build Commands

```bash
# Build the project
cargo build

# Build with release optimizations
cargo build --release

# Run tests
cargo test

# Run tests for specific module
cargo test cmake
```

## Linting

```bash
# Run clippy linting
cargo clippy --all-targets
```

## Code Style Guidelines

### Imports

- Group and order imports:
  1. Standard library
  2. External crates
  3. Local modules
- Use `use` statements for commonly used items
- Avoid wildcard imports

### Formatting

- Use 4 spaces for indentation
- Keep lines under 100 characters
- Use snake_case for function and variable names
- Use PascalCase for struct and enum names
- Use UPPER_SNAKE_CASE for constants

### Error Handling

- Use thiserror crate for error handling
- Return Result<T, CommandError> from functions
- Use exitcode crate for consistent exit codes

### Testing

- Use serial_test for tests that may share resources (like filesystem or network)
- Write tests for all public functions
- Mock external dependencies when possible

### Cursor Rules

```bash
# Cursor rules are not defined in this project
```

### Copilot Instructions

```bash
# Copilot instructions are not defined in this project
```
