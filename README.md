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

- Automatically discover the variables that script needs — no prior setup or configuration required
- Prompt for each one interactively, with context (description, type, whether it's required)
- Run the underlying Terraform commands on the user's behalf, in a controlled, reviewable way

The goal is a tool that works instantly with *any* Terraform setup — not just ones it's been pre-configured for.

## The Intent

This isn't meant to just be a shortcut for typing `terraform apply` faster. The real purpose is:

1. **Generalize across any Terraform project** — one CLI that adapts to whatever script it's pointed at, instead of needing custom wrapper code per project.
2. **Act as a guardrail** — teams can standardize on running infrastructure changes through this CLI instead of raw Terraform commands, making runs more consistent, reviewable, and safer.
3. **Lower the barrier to entry** — anyone can clone a Terraform project, point the CLI at it, and immediately know what's needed to run it — no digging through `.tf` files first.

## Who This Is For

- Teams who want a safer, more consistent way for engineers to run shared Terraform infrastructure
- Anyone who's tired of writing one-off wrapper scripts per Terraform project just to handle variable input
- Open-source Terraform projects that want a friendlier on-ramp for new contributors

## Proposed Architecture Overview

At a high level, `tilth` acts as an intelligent wrapper around the Terraform binary:
1. **Discovery (HCL Parsing):** When pointed to a directory, it uses an HCL parser (like `hcl-rs`) to read the `.tf` configuration files and extract required and optional variables, their types, and descriptions.
2. **Interactive Prompting:** The CLI interactively prompts the user for missing variables, validating inputs against the discovered types.
3. **Command Generation:** It constructs the safe, fully-qualified `terraform` execution command (e.g., injecting variables via `-var` flags or temporary `.tfvars` files).
4. **Execution & Guardrails:** It spawns a child process to run the Terraform command while enforcing safety gates (e.g., requiring explicit confirmation before a `destroy` or defaulting to `plan` first).

## Basic Usage

```bash
# Basic usage
tilth apply ./infra/vpc

# Plan first (safe default)
tilth plan ./modules/eks

# Destroy with extra confirmation gate
tilth destroy ./infra/vpc

# Inspect variables without running
tilth inspect ./infra/vpc
```

---

*This README describes the intent and problem tilth aims to solve. Implementation details and usage instructions will follow as the project develops.*
