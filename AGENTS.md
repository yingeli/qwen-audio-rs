# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust library crate entry (`lib.rs`). Add modules as `src/<module>.rs` or `src/<module>/mod.rs`.
- `audio/`: Sample audio assets for quick experiments.
- `models/`: Python utilities/demos for Qwen2-Audio + TensorRT-LLM (see `models/README.md`).
- `.devcontainer/`: Container setup based on NVIDIA TensorRT-LLM with Rust toolchain.
- Tests live alongside modules using `#[cfg(test)]` or in `tests/` for integration.

## Build, Test, and Development Commands
- Build library: `cargo build --release` (use `--verbose` while iterating).
- Format code: `cargo fmt --all`.
- Lint & static checks: `cargo clippy --all-targets -- -D warnings`.
- Run tests: `cargo test` (unit + integration).
- Python demos: `python3 models/run.py ...` or `python3 models/run_chat.py ...` after following `models/README.md` (TensorRT-LLM required).

## Coding Style & Naming Conventions
- Rust: rustfmt defaults (4-space indent). Use `snake_case` for functions/vars, `PascalCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts. Keep modules focused and small.
- Errors: prefer `anyhow::Result<T>` at boundaries; map/annotate with context.
- Public API: minimal, documented with `///` rustdoc. Example module file: `src/audio_io.rs` with `mod audio_io;` in `lib.rs`.
- Python (models): follow PEP 8 where editing; keep scripts self-contained.

## Testing Guidelines
- Unit tests in the same file under `#[cfg(test)] mod tests { ... }`.
- Integration tests in `tests/` as `tests/<feature>_test.rs`.
- Name tests descriptively, e.g., `test_decodes_wav_header()`.
- Keep tests deterministic; avoid network or GPU dependencies in Rust tests.

## Commit & Pull Request Guidelines
- Commits: use concise, imperative subject lines. Prefer Conventional Commits when possible: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`.
- PRs: include problem statement, approach, and validation steps. Link issues. Add terminal output or screenshots when helpful.
- Keep PRs small and focused. Include `cargo fmt` and `cargo clippy` clean runs.

## Security & Configuration Tips
- Do not commit model checkpoints or large binaries; use Git LFS where needed (audio samples are OK).
- GPU/TensorRT work belongs under `models/`; the Rust crate should remain CPU/tooling-agnostic unless stated otherwise.
- Use the devcontainer for a consistent environment; otherwise install Rust via rustup and up-to-date toolchains.
