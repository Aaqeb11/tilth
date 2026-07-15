# tilth 🌱

A CLI that makes running Terraform safer and easier for teams — without requiring anyone to memorize or hardcode variables in advance.

> *tilth (n.): the condition of soil, especially in respect to suitability for planting — well-prepared ground.*

## Features

- **Automatic Variable Discovery:** Scans your Terraform directory, parses `.tf` files natively (HCL), and finds exactly what variables are required.
- **Interactive Prompting:** Dynamically prompts you for missing variables, displaying descriptions and pre-filling default values.
- **Zero Config:** Works instantly with *any* Terraform setup. No custom wrapper scripts, YAML configs, or hardcoded `.tfvars` required.
- **Safe Execution:** Generates temporary, secure `.tfvars.json` files on the fly, runs the underlying `terraform` binary natively, and cleans up automatically.
- **Passthrough Arguments:** Supports passing arbitrary native Terraform flags (like `-target` or `-parallelism`).

## Installation

### Option 1: Pre-compiled Binaries (macOS, Linux, Windows)
Download the latest pre-compiled binary for your operating system from the [GitHub Releases](https://github.com/Aaqeb11/tilth/releases) page.

**macOS / Linux Example:**
```bash
# Download the binary (example for macOS ARM64 / Apple Silicon)
curl -LO https://github.com/Aaqeb11/tilth/releases/download/v0.1.0/tilth-darwin-arm64

# Make it executable
chmod +x tilth-darwin-arm64

# Move it to your path
sudo mv tilth-darwin-arm64 /usr/local/bin/tilth
```
*(Note: Replace `tilth-darwin-arm64` with `tilth-darwin-amd64` for Intel Macs, or `tilth-linux-amd64`/`tilth-linux-arm64` for Linux).*

### Option 2: Using Cargo (Rust toolchain)
If you have Rust installed, you can install directly from the repository:
```bash
cargo install --git https://github.com/Aaqeb11/tilth
```

## Usage

Point `tilth` at any directory containing Terraform `.tf` files.

```bash
# Safely plan infrastructure (prompts for variables first)
tilth plan ./infra/vpc

# Apply infrastructure changes
tilth apply ./infra/vpc

# Destroy infrastructure
tilth destroy ./infra/vpc

# Just inspect the required variables without running Terraform
tilth inspect ./infra/vpc
```

### Passing extra arguments to Terraform
You can pass any native Terraform arguments (like `-target` or `-auto-approve`) by appending them after `--`.

```bash
tilth apply ./infra/vpc -- -target=module.eks_cluster
```

## How it Works Under the Hood

1. **Discovery:** `tilth` does a shallow scan of the provided directory to respect Terraform's strict Root Module boundaries. It parses the HCL AST to extract variable definitions.
2. **Prompting:** Uses an interactive terminal UI to ask the user for values. Skips prompting if no variables are required.
3. **Execution:** Serializes the answers to a temporary `.tfvars.json` file inside the target directory, spawns the `terraform` process with inherited I/O so you see the native output, and automatically deletes the temporary file upon completion (or cancellation).

## License

MIT
