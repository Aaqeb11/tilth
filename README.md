# tilth

A CLI that makes running Terraform safer and easier for teams — without requiring anyone to memorize or hardcode variables in advance.

> *tilth (n.): the condition of soil, especially in respect to suitability for planting — well-prepared ground.*

## Features

- **Automatic Variable Discovery:** Scans your Terraform directory, parses `.tf` files natively (HCL), and finds exactly what variables are required.
- **Interactive Prompting:** Dynamically prompts you for missing variables, displaying descriptions and pre-filling default values.
- **Zero Config:** Works instantly with *any* Terraform setup. No custom wrapper scripts, YAML configs, or hardcoded `.tfvars` required.
- **Safe Execution:** Generates temporary, secure `.tfvars.json` files on the fly, runs the underlying `terraform` binary natively, and cleans up automatically.
<!-- TODO: confirm this bullet matches actual behavior before publishing -->
- **Guardrails:** Requires explicit confirmation before running `destroy`, so a single mistyped command can't take down shared infrastructure.
- **Passthrough Arguments:** Supports passing arbitrary native Terraform flags (like `-target` or `-parallelism`).

## Prerequisites

`tilth` is a wrapper around the native `terraform` binary — it doesn't bundle or replace it. You'll need:

- [Terraform](https://developer.hashicorp.com/terraform/install) installed and available on your `PATH`
<!-- TODO: fill in actual minimum version you've tested against -->
- Terraform `>= 1.x` (tested against `1.x`)

## Installation

### Option 1: Pre-compiled Binaries (macOS, Linux, Windows)

Download the latest pre-compiled binary for your operating system from the [GitHub Releases](https://github.com/Aaqeb11/tilth/releases) page.

**macOS / Linux:**
```bash
# Download the binary (example for macOS ARM64 / Apple Silicon)
curl -LO https://github.com/Aaqeb11/tilth/releases/download/v0.1.1/tilth-darwin-arm64

# Make it executable
chmod +x tilth-darwin-arm64

# Move it to your path
sudo mv tilth-darwin-arm64 /usr/local/bin/tilth
```
*(Replace `tilth-darwin-arm64` with `tilth-darwin-amd64` for Intel Macs, or `tilth-linux-amd64` / `tilth-linux-arm64` for Linux.)*

**Windows:**
1. Download `tilth-windows-amd64.exe` from the [Releases](https://github.com/Aaqeb11/tilth/releases) page.
2. (Optional) Rename it to `tilth.exe` and move it into a folder on your `PATH` (e.g. `C:\Windows\System32` or a custom tools directory) so you can run `tilth` from anywhere.
3. Verify it works:
```powershell
.\tilth.exe --help
```

### Option 2: Using Cargo (Rust toolchain)

If you have Rust installed, you can install directly from crates.io
```bash
cargo install tilth-tf
```

## Usage

Point `tilth` at any directory containing Terraform `.tf` files.

| Command | Description |
|---|---|
| `tilth inspect <dir>` | Discover and display required variables without running Terraform |
| `tilth plan <dir>` | Prompt for variables, then run `terraform plan` |
| `tilth apply <dir>` | Prompt for variables, then run `terraform apply` |
| `tilth destroy <dir>` | Prompt for variables and an extra confirmation, then run `terraform destroy` |

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

### Example

```
$ tilth plan ./infra/vpc
? region (string, default: "us-east-1") › us-east-1
? instance_count (number) › 3
? enable_nat_gateway (bool, default: true) › true

Running: terraform plan -var-file=/tmp/tilth-8f2a1c.tfvars.json

Terraform will perform the following actions:
  ...

Plan: 12 to add, 0 to change, 0 to destroy.
```

## How it Works Under the Hood

1. **Discovery (Shallow Scanning):** `tilth` explicitly performs a *shallow scan* of the target directory to extract variables. It intentionally does not recursively scan subdirectories (child modules). This perfectly mirrors Terraform's own strict "Root Module" boundaries. In Terraform, a root module must supply all required variables to any child modules it calls. If `tilth` prompted you for a child module's internal variables directly, it would bypass the root module's configuration and cause Terraform to crash (often due to missing provider contexts).
2. **Prompting:** Uses an interactive terminal UI to ask the user for values. Skips prompting if no variables are required.
3. **Execution:** Serializes the answers to a temporary `.tfvars.json` file inside the target directory, spawns the `terraform` process with inherited I/O so you see the native output, and automatically deletes the temporary file upon completion (or cancellation).

## Contributing

Issues and pull requests are welcome. If you run into a bug or have a feature request, please [open an issue](https://github.com/Aaqeb11/tilth/issues).

## License

MIT
