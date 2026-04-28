# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Syma** is a symbolic-first programming language inspired by Wolfram Language, written in Rust (edition 2024). Multi-tier execution: tree-walk interpreter → bytecode VM → JIT via Cranelift. Includes REPL, kernel mode (JSON stdio), and DAP debugger. The full language specification is in `syma/syma-lang.md` (1200+ lines, includes EBNF grammar) — that file is the source of truth for syntax and semantics.

## Workspace Structure

Cargo workspace with two crates and a build system:

- **`syma/`** — The language: lexer, parser, evaluator, builtins, pattern engine, REPL, CLI
- **`xtask/`** — Build system (cargo-xtask pattern). Orchestrates build, install, dist, test, lint, clean, and SystemFiles setup.
- **`docs/software-architecture.md`** — Target modular architecture (SystemFiles, PackageManager, etc.)

## Build System (cargo-xtask)

```bash
# Primary build commands
cargo xtask build                          # Build syma (debug)
cargo xtask build --release                # Build syma (release)

# Install to $SYMA_HOME (default: ~/.syma)
cargo xtask install                        # Build + install binary + create SystemFiles layout
cargo xtask install --release              # Release build + install

# Distribution
cargo xtask dist                           # Create target/dist/syma-0.1.0.tar.gz

# Development (conduct a whole test if expensive and slow, test specific modules with cargo test)
cargo xtask test                           # Run all tests (cargo test --locked --workspace)
cargo xtask lint                           # fmt --check + clippy -- -D warnings
cargo xtask clean                          # Clean target/

# SystemFiles management
cargo xtask setup-sysfiles                 # Create ~/.syma/SystemFiles skeleton (no build)
```

### Install Layout ($SYMA_HOME, default ~/.syma)

```
~/.syma/
├── bin/syma                    # Installed binary
├── SystemFiles/
│   ├── Kernel/init.toml        # Module registry (skeleton)
│   ├── Data/{Rubi,Chemistry,Physics}/
│   ├── Formats/
│   └── Links/
├── Packages/                   # Standard library packages (future)
└── Extensions/                 # Native plugins (future)
```

## Direct Cargo Commands

```bash
# Quick development cycle
cargo check                              # Fast type-check
cargo run                                # Launch REPL
cargo run -- -e "1 + 2"                 # Evaluate expression
cargo run -- <file.syma>                # Run source file
cargo test lexer                         # Run tests for specific module
cargo test --test cli                    # Integration tests only
```

## Architecture

**Pipeline (3 tiers):** `Source → Lexer → Parser → AST → Evaluator → Value` (tree-walk, default). Hot functions auto-promote to `AST → Bytecode → VM → Value`. Optional JIT: `Bytecode → Cranelift → Native → Value`.

### Key Modules (in `syma/src/`)

- **`lexer.rs`** — Tokenizer. Handles Wolfram-style multi-char operators with maximal munch (`//.` before `//` before `/`). `(* *)` comments support nesting. Pattern blanks like `x_Integer` are lexed as single identifiers.
- **`ast.rs`** — AST node definitions. Everything is an `Expr` enum variant. Operators are desugared to `Call` nodes (e.g., `a + b` → `Plus[a, b]`).
- **`parser.rs`** — Recursive descent with precedence climbing. Expression precedence (low→high): pipe (`//`) → at/apply (`@`/`@@`) → rule (`->`/`:>`) → or → and → comparison → add → mul → power → unary → postfix → primary.
- **`value.rs`** — Runtime value types: atoms, `List`, `Call`, `Assoc` (hash map), `Function` (user-defined with pattern defs), `Builtin`, `PureFunction`, `Object`, `RuleSet`, `Pattern` (wraps unevaluated `Expr`).
- **`eval.rs`** — Tree-walk evaluator. `eval()` dispatches on `Expr` variants. `apply_function()` dispatches on `Value` types. Function definitions accumulate — multiple `f[x_] := ...` defs coexist and are tried in order.
- **`env.rs`** — Lexical scoping via `Rc<RefCell<Scope>>` chains. `child()` creates a new scope inheriting the parent.
- **`pattern.rs`** — Pattern matching engine. Supports blanks (`_`, `x_`, `_Integer`), sequences (`__`, `___`), list destructuring, call patterns, alternatives (`|`), and guards (`/;`).
- **`builtins/`** — Core library split by domain: `arithmetic`, `comparison`, `logical`, `list`, `string`, `math`, `pattern`, `association`, `symbolic`, `random`, `io`, `error`, `parallel`, `ffi`, `filesystem`.
- **`kernel.rs`** — JSON-over-stdin/stdout kernel mode for IDE integration.
- **`debug.rs`** — DAP (Debug Adapter Protocol) support.
- **`bytecode/`** — Bytecode compiler (`compiler.rs`) compiles hot functions to register-based bytecode. VM (`vm.rs`) executes it. Sits between tree-walk and JIT.
- **`jit/`** — Cranelift JIT backend. Compiles bytecode to native machine code. Behind `"jit"` feature flag.
- **`cli.rs`** — Package scaffolding (`syma new`), build, run, dependency management commands.

### Key Design Decisions

- **Operators → Calls**: All operators are desugared into `Call` nodes with PascalCase heads. `a - b` → `Plus[a, Times[-1, b]]`. Unary `-x` → `Times[-1, x]`.
- **Pattern-as-Value**: Patterns in rules/function defs are stored as `Value::Pattern(Expr)` — unevaluated AST wrapped in a value.
- **`x_` in lexer**: Pattern blanks like `x_Integer` are single `Ident` tokens. The parser's `convert_pattern()` method splits them by `_`.
- **Lazy Rubi loading**: `Integrate` is registered as a lazy provider. On first call, `syma-rubi` parses all 185 `.m` rule files and initializes a global `RubiEngine` via `OnceLock`.

### What's Not Yet Implemented

- `@transform` class member type (lexer/parser/AST ready, evaluator skips)
- `Simplify` is basic; advanced symbolic manipulation (e.g., trig identities) is limited
- The modular `PackageManager` architecture described in `docs/software-architecture.md` (currently everything is eagerly registered via `register_builtins()`)

## CI

GitHub Actions (`.github/workflows/ci.yml`) with three jobs:
1. **check** — `cargo check --locked` (fast type-check, runs first)
2. **test** — `cargo test --locked` (all tests, depends on check)
3. **lint** — `cargo fmt --check` + `cargo clippy --locked -- -D warnings` (depends on check)

The `rug` crate requires `libgmp-dev` and `clang` on Ubuntu. `Cargo.lock` is checked in; all jobs use `--locked`.

## Syntax Quick Reference

```
(* comment *)                  # Nestable comments
;                              # Statement separator; last expression is the result
f[x_] := body                  # Delayed function definition
x = val                        # Assignment
{1, 2, 3}                      # List literal (= List[1, 2, 3])
"a" <> "b"                    # String concatenation
a -> b                         # Rule (immediate)
a :> b                         # Rule (delayed)
expr /. rules                  # Replace all
expr //. rules                 # Replace repeated
expr // f                      # Postfix pipe
f /@ expr                      # Map (= Map[f, expr])
f @@ expr                      # Apply (= Apply[f, expr])
```
