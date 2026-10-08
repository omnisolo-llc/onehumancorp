# Native development, testing and build caches

## One-command initialization

```sh
make init                             # Install prerequisites, prompting before sudo/Homebrew changes
make doctor                           # Check readiness without installing dependencies
make lint                             # Complete Rust + Node quality gates
make test                             # Complete workspace, unit, contract and real-stack browser tests
```

Bootstrap inputs: Git, GNU Make and Python, plus permission to install missing host packages. The Python environment used by release/test tools must be 3.11 or newer with venv support (Ubuntu 24.04+/Debian 12+ include a suitable default). On older Linux distributions, install a suitable Python first. macOS requires Homebrew and Xcode command-line tools; `xcode-select --install` is an interactive Apple installation and is not run silently. Native Windows release builds retain their own MSVC/WebView2 requirements; use WSL2 Ubuntu with Docker integration for the full POSIX test environment.

The initializer reads the repository's Node and Rust pins, installs Rust with rustfmt/Clippy without changing the global default, and uses verified official Node archives when the matching Node version is absent. npm/npx and tool links live in ignored `target/dev-tools/`. All four locked npm trees are installed with development dependencies: repository root, Next, CLI and `.github/test-tools` (OpenCode). Installation stamps include both manifests, Node, OS and architecture. Python packages use an isolated venv; Cargo dependencies are fetched with `--locked`. The initializer installs Chromium and its Linux system libraries, then actually launches and closes the browser during readiness checks.

```sh
make init INIT_ARGS=--plan             # Print the plan; no network, installations or writes
make init INIT_ARGS=--yes              # Approve the described host package installations
make init INIT_ARGS=--no-system        # Never invoke