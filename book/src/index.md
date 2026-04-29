# Syma Language

Syma is a **symbolic-first** programming language inspired by Wolfram Language, built in Rust.

**Core idea:** Everything is a symbolic expression. Code, data, and math are the same thing.

```syma
(* This is a comment *)
1 + 2               (* arithmetic *)
square[x_] := x^2   (* function definition *)
square[5]           (* => 25 *)
{1, 2, 3}           (* list literal *)
f /@ {1, 2, 3}      (* map over list *)
```

## Key Features

- **Symbolic expressions** — Code is data, data is code
- **Pattern matching** — Define behavior by shape, not type checks
- **Multi-tier execution** — Tree-walk interpreter (default), bytecode VM, Cranelift JIT (future)
- **OOP classes** — Classes, inheritance, mixins, operator overloading
- **REPL + kernel mode** — Interactive exploration + JSON stdio for IDE integration
- **Modules** — Namespaced code organization with selective imports

## Status

Syma is in active development — Phase 1 (tree-walk interpreter with REPL) is functional. See the tutorial chapters to start writing Syma code.
