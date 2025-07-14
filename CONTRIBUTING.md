# Contributing to Squad

## Getting Started

1. **Prerequisites**
   - Rust 1.75+ 
   - Podman or Docker

2. **Setup**
   ```bash
   git clone https://github.com/cargodog/squad.git
   cd squad
   ./install-hooks.sh  # Install git hooks
   cargo build          # Verify everything builds
   ```

## Development Workflow

### Code Standards

- **Formatting**: `cargo fmt` (enforced by pre-commit hook)
- **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Building**: `cargo build` must pass
- **Testing**: `cargo test` for all changes

### Git Workflow

- Make small, atomic commits
- All git hooks run automatically (never use `--no-verify`)
- Pre-commit hook checks formatting, linting, builds, and security
- Commit messages should be clear and concise

### Testing

Run tests with:
```bash
cargo test                    # Unit and integration tests
cargo test --test <name>      # Specific test file
```

### Security

- No hardcoded secrets/keys in code
- Minimal debug prints in production code
- Pre-commit hook performs basic security checks

## Project Structure

```
squad/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli/                 # CLI command implementations
│   │   ├── mod.rs           # Module exports
│   │   ├── create.rs        # Create agents
│   │   ├── create_test.rs   # Create command tests
│   │   ├── list.rs          # List agents
│   │   ├── list_test.rs     # List command tests
│   │   ├── rm.rs            # Remove agents
│   │   ├── rm_test.rs       # Remove command tests
│   │   ├── ps.rs            # Show agent status
│   │   ├── ps_test.rs       # PS command tests
│   │   └── utils.rs         # CLI utilities
│   ├── container/           # Container runtime abstraction
│   │   ├── mod.rs           # Module exports
│   │   ├── agent.rs         # Agent management
│   │   ├── agent_tests.rs   # Agent tests
│   │   ├── runtime.rs       # Runtime detection & execution
│   │   ├── runtime_tests.rs # Runtime tests
│   │   ├── parser.rs        # Container output parsing
│   │   └── security_test.rs # Security tests
├── tests/                   # Integration tests
│   ├── integration_test.rs  # Main integration tests
│   └── mock_runtime_test.rs # Mock runtime tests
├── hooks/                   # Git hooks
│   ├── commit-msg           # Commit message validation
│   ├── pre-commit           # Pre-commit checks
│   └── pre-push             # Pre-push validation
├── Cargo.toml               # Dependencies
├── build.sh                 # Build script
├── install-hooks.sh         # Git hooks installer
└── CLAUDE.md                # Project instructions
```

## Submitting Changes

1. **Before Committing**:
   ```bash
   cargo fmt              # Format code
   cargo clippy --all-targets --all-features -- -D warnings  # Lint
   cargo test             # Run tests
   cargo build            # Verify build
   ```

2. **Commit Process**:
   - Ensure all hooks pass: `git commit` (hooks run automatically)
   - Hooks will automatically run formatting, linting, build, and security checks
   - Never use `--no-verify` to skip hooks

3. **Testing**:
   ```bash
   cargo test                    # Unit and integration tests
   ./build.sh                    # Build containers (if applicable)
   ```

4. **Pull Request Guidelines**:
   - Create small, focused PRs
   - Include tests for new functionality
   - Update documentation if needed
   - Ensure CI passes before requesting review

## Code Review Process

- All changes require review before merging
- PRs should be kept small and focused
- Review feedback should be addressed promptly
- Maintainers will merge once approved and CI passes

## Release Process

1. Version bumps follow [Semantic Versioning](https://semver.org/)
2. Update `Cargo.toml` version
3. Update `CHANGELOG.md` (if present)
4. Tag release: `git tag v0.x.y`
5. Push tag: `git push origin v0.x.y`

## Getting Help

- Check existing [issues](https://github.com/cargodog/squad/issues)
- Create new issue for bugs or feature requests
- Join discussions for questions or ideas
