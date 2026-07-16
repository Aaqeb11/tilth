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

## Working with Modules
 
Because `tilth` performs a **shallow scan** of only the root module directory (see [How it Works](#how-it-works-under-the-hood) above), it only discovers variables declared in the root's own `variables.tf` / `main.tf` — it does **not** recurse into child modules to find their variables.
 
### Why shallow scan?
 
`tilth` explicitly performs a *shallow scan* of the target directory to extract variables. It intentionally does not recursively scan subdirectories (child modules). This perfectly mirrors Terraform's own strict "Root Module" boundaries. In Terraform, a root module must supply all required variables to any child modules it calls. If `tilth` prompted you for a child module's internal variables directly, it would bypass the root module's configuration and cause Terraform to crash (often due to missing provider contexts).
 
This means: if your root `main.tf` calls a child module but doesn't pass a particular variable through, `tilth` won't know that variable exists, and it won't prompt for it or include it in the generated `.tfvars.json`.
 
**The fix is standard Terraform practice anyway** — declare a root-level variable for anything a child module needs, and pass it through explicitly:
 
```
infra/
├── main.tf          # root module — calls child modules
├── variables.tf      # root-level variable declarations
└── modules/
    ├── vpc/
    │   ├── main.tf
    │   └── variables.tf   # tilth does NOT scan this directly
    └── eks/
        ├── main.tf
        └── variables.tf   # tilth does NOT scan this directly
```
 
**`modules/vpc/variables.tf`** (child module):
```hcl
variable "cidr_block" {
  description = "CIDR block for the VPC"
  type        = string
}
```
 
**`variables.tf`** (root — this is what `tilth` actually reads):
```hcl
variable "vpc_cidr_block" {
  description = "CIDR block for the VPC"
  type        = string
  default     = "10.0.0.0/16"
}
```
 
**`main.tf`** (root — wires the root variable into the module):
```hcl
module "vpc" {
  source     = "./modules/vpc"
  cidr_block = var.vpc_cidr_block
}
 
module "eks" {
  source = "./modules/eks"
  # ...same pattern for eks-specific variables
}
```
 
With this structure, `tilth inspect ./infra` picks up `vpc_cidr_block` (and every other root-declared variable across all your modules), prompts for it once, and writes it into a single `.tfvars.json` that satisfies every module underneath — not just one.
 
### Provisioning a single module
 
Since the generated `.tfvars.json` contains variables for *all* modules under the root, you can still scope an operation to just one module using Terraform's native `-target` flag, passed through after `--`:
 
```bash
# Only apply the vpc module, even though variables for eks etc. were also prompted for
tilth apply ./infra -- -target=module.vpc
```
 
This is useful when you want the convenience of one prompt/one `.tfvars.json` for the whole root config, but need to stage changes module-by-module (e.g. bring up networking before compute).
 
> **Note:** this pattern is specifically a consequence of `tilth`'s shallow-scan design — if you restructure your modules or add new ones, remember to add matching root-level variables and wiring, or `tilth` won't discover the new inputs.

## Contributing

Issues and pull requests are welcome. If you run into a bug or have a feature request, please [open an issue](https://github.com/Aaqeb11/tilth/issues).

## License

MIT
