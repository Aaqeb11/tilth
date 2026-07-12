# tilth

A CLI that makes running Terraform safer and easier for teams — without requiring anyone to memorize or hardcode variables in advance.

> *tilth (n.): the condition of soil, especially in respect to suitability for planting — well-prepared ground.*

## The Problem

Terraform is powerful, but running it directly comes with friction and risk:

- **Every module needs different variables.** There's no easy way to build a single generic wrapper around Terraform scripts, because each script requires its own set of input variables — and a tool would normally need to know these in advance to be useful.
- **Direct access is risky.** Letting every team member run `terraform apply` / `terraform destroy` directly means anyone can make an untracked, unreviewed, potentially destructive change to shared infrastructure.
- **Onboarding is painful.** New team members (or new contributors to any Terraform project) have to dig through `.tf` files just to figure out what inputs a script even expects before they can run it.

## The Idea

Instead of building a CLI that needs to already know a script's variables, this tool reads them directly from the Terraform configuration itself.

Point the CLI at any Terraform directory, and it will:

- Automatically discover the variables that script needs (by parsing the HCL) — no prior setup or configuration required
- Prompt for each one interactively, with context (description, type, default value)
- Run the underlying Terraform commands on the user's behalf, in a controlled, reviewable way

The goal is a tool that works instantly with *any* Terraform setup — not just ones it's been pre-configured for.

## Current Progress

- [x] **Phase 1: Discovery** - `tilth` can successfully scan a target directory (shallow scan), parse the `.tf` files using `hcl-rs`, and extract variable names, descriptions, types, and default values. Try `tilth inspect <dir>`.
- [ ] **Phase 2: Interactive Prompting** - (Next up) Taking the discovered variables and using a library like `inquire` to build dynamic terminal prompts.
- [ ] **Phase 3: Execution Wrapper** - Passing the collected variables safely to the underlying `terraform` binary (e.g. generating a temporary `.tfvars` file) and passing through arbitrary flags (like `-target`).
- [ ] **Phase 4: Guardrails** - Implementing the specific safety flows for `plan`, `apply`, and `destroy`.

## Proposed Architecture Overview

At a high level, `tilth` acts as an intelligent wrapper around the Terraform binary:
1. **Discovery (HCL Parsing):** When pointed to a directory, it uses an HCL parser (like `hcl-rs`) to read the `.tf` configuration files and extract required and optional variables, their types, and descriptions. It respects Terraform's module boundaries by only performing a shallow scan of the provided Root Module.
2. **Interactive Prompting:** The CLI interactively prompts the user for missing variables, validating inputs against the discovered types.
3. **Command Generation:** It constructs the safe, fully-qualified `terraform` execution command (e.g., injecting variables via temporary `.tfvars` files and passing through targeting flags).
4. **Execution & Guardrails:** It spawns a child process to run the Terraform command while enforcing safety gates (e.g., requiring explicit confirmation before a `destroy` or defaulting to `plan` first).

## Basic Usage (Current & Planned)

```bash
# [IMPLEMENTED] Inspect variables without running
cargo run -- inspect ./infra/vpc

# [PLANNED] Basic usage
tilth apply ./infra/vpc

# [PLANNED] Plan first (safe default)
tilth plan ./modules/eks

# [PLANNED] Destroy with extra confirmation gate
tilth destroy ./infra/vpc

# [PLANNED] Passing native terraform flags (e.g. targeting child modules)
tilth apply ./infra/vpc -- -target=module.my_database
```

---

*This README describes the intent and problem tilth aims to solve. Implementation details and usage instructions will follow as the project develops.*
