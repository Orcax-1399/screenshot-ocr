# Repository Guidelines

## Project Structure

- `src/main.rs`: CLI entry point; reads image bytes from stdin (typically `grim ... - | screenshot-ocr`).
- `src/config.rs`: Loads config from `~/.config/screenshot-ocr/config.toml` and env.
- `src/clipboard.rs`: Clipboard integration via `wl-copy`.
- `src/notifier.rs`: Desktop notifications (D-Bus via `notify-rust`).
- `src/provider/`: OCR backends (`glm.rs`) behind the `OcrProvider` trait (`mod.rs`).
- `target/`: Cargo build output (gitignored).

## Build, Test, and Development Commands

- `cargo build`: Build a debug binary.
- `cargo build --release`: Build an optimized binary (release profile uses LTO/strip).
- `cargo run -- < image.png`: Run locally by piping an image file to stdin.
- `cargo test`: Run unit/integration tests (add tests as the project grows).
- `cargo fmt`: Format code with rustfmt.
- `cargo clippy -- -D warnings`: Lint and fail on warnings.

## Coding Style & Naming Conventions

- Follow standard Rust 2021 conventions; let `cargo fmt` be the source of truth.
- Use `snake_case` for modules/functions, `UpperCamelCase` for types, and keep modules small and single-purpose.
- Prefer `anyhow::Result<T>` for fallible flows at binary boundaries; keep provider-specific errors informative.

## Testing Guidelines

- Unit tests: place in the same file under `#[cfg(test)] mod tests { ... }`.
- Integration tests (if needed): create `tests/*.rs` and exercise the CLI/provider boundary.
- When adding new providers, include at least one test that validates request/response parsing with a fixed fixture.

## Commit & Pull Request Guidelines

- History is currently a single initial commit; adopt **Conventional Commits** going forward (e.g., `feat: add tesseract provider`, `fix: handle empty stdin`).
- PRs should include: what changed, how to run it (`grim`/`slurp`/`wl-copy` pipeline), and any config updates.

## Security & Configuration Tips

- Never commit API keys. Use `GLM_API_KEY` or `~/.config/screenshot-ocr/config.toml` with `chmod 600`.
- This tool targets Arch + Wayland (niri). Keep shell commands and examples Wayland-first (`wl-copy`, `grim`, `slurp`).
