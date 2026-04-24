# Syma

A symbolic-first programming language inspired by Wolfram Language, written in Rust.

Syma is a general-purpose language where everything is a symbolic expression. Functions, data, and code are unified under a single pattern-matching evaluation model — similar to Mathematica/Wolfram Language, but open source and written in Rust.

## Quick Start

```bash
# Clone and build
git clone https://github.com/syma-project/syma-project.git --recursive
cd syma
cargo xtask build

# Run the REPL
cargo run

# Or install to ~/.syma
cargo xtask install
export PATH="$HOME/.syma/bin:$PATH"
syma
```

## Language Overview

### Basics

```mathematica
(* Arithmetic *)
1 + 2 * 3          (* → 7 *)
2^10                (* → 1024 *)

(* Variables *)
x = 42;
x + 8               (* → 50 *)

(* Strings *)
StringJoin["hello", " ", "world"]   (* → "hello world" *)

(* Lists *)
{1, 2, 3}           (* → List[1, 2, 3] *)
Length[{a, b, c}]    (* → 3 *)
```

### Functions and Pattern Matching

```mathematica
(* Function definitions use patterns *)
f[x_] := x^2 + 1;
f[5]                  (* → 26 *)

(* Multiple definitions — tried in order *)
fib[0] := 0;
fib[1] := 1;
fib[n_] := fib[n - 1] + fib[n - 2];
fib[10]               (* → 55 *)

(* Type-constrained patterns *)
g[x_Integer] := x * 2;
g[x_String]  := StringJoin["got: ", x];
```

### Rules and Transformations

```mathematica
(* Rules: -> (immediate) and :> (delayed) *)
{x, y, x} /. x -> 0          (* → {0, y, 0} *)

(* Pattern-based replacement *)
{1, 2, 3, 4} /. n_Integer -> n^2   (* → {1, 4, 9, 16} *)
```

### Functional Programming

```mathematica
(* Map, Fold, Select *)
Map[f, {1, 2, 3}]              (* → {f[1], f[2], f[3]} *)
f /@ {1, 2, 3}                 (* same as above *)
Select[{1, 2, 3, 4, 5}, EvenQ]
Fold[Plus, 0, {1, 2, 3, 4}]    (* → 10 *)

(* Pure functions *)
(#^2 &) /@ {1, 2, 3}           (* → {1, 4, 9} *)
```

### Symbolic Computation

```mathematica
(* Differentiation *)
D[x^3, x]                      (* → 3 x^2 *)

(* Symbolic integration (via Rubi rules) *)
Integrate[x^2, x]              (* → x^3 / 3 *)
Integrate[Sin[x], x]           (* → -Cos[x] *)
```

### Control Flow

```mathematica
If[x > 0, "positive", "non-positive"]

Which[
  x < 0,  "negative",
  x == 0, "zero",
  True,   "positive"
]

For[i = 1, i <= 10, i++, Print[i]]

n = 1; While[n < 100, n = n * 2]
```

### I/O

```mathematica
Print["Hello, world!"]

(* File I/O *)
WriteString["output.txt", "data"]
ReadString["output.txt"]

(* JSON *)
Export["data.json", {1, 2, 3}]
Import["data.json"]
```

## CLI Usage

```bash
syma                       # Interactive REPL
syma <file.syma>           # Run a source file
syma -e "1 + 2"           # Evaluate an expression
syma --check <file>        # Parse-only syntax check
syma --dap <file>          # Debug mode (DAP protocol)
syma --kernel              # Kernel mode (JSON over stdin/stdout)
```

### Package Commands

```bash
syma new myapp             # Create a binary package
syma new --lib mylib       # Create a library package
syma run                   # Run the package entry point
syma build                 # Check syntax of all src/ files
syma test                  # Run all files in tests/
syma add <pkg>[@ver]       # Add a dependency
syma remove <pkg>          # Remove a dependency
syma install               # Install dependencies
```

## Build System

Syma uses [cargo-xtask](https://github.com/matklad/cargo-xtask) for build orchestration.

```bash
cargo xtask build                   # Build (debug)
cargo xtask build --release         # Build (release)
cargo xtask build --features rubi   # Build with Rubi integration engine
cargo xtask install                 # Install to ~/.syma
cargo xtask dist                    # Create distributable archive
cargo xtask test                    # Run all tests
cargo xtask lint                    # fmt + clippy
cargo xtask clean                   # Clean build artifacts
cargo xtask setup-sysfiles          # Create SystemFiles skeleton
```

### Install Layout

`cargo xtask install` creates the following structure in `$SYMA_HOME` (default: `~/.syma`):

```
~/.syma/
├── bin/syma
├── SystemFiles/
│   ├── Kernel/
│   │   └── init.toml
│   ├── Data/
│   │   ├── Rubi/
│   │   ├── Chemistry/
│   │   └── Physics/
│   ├── Formats/
│   └── Links/
├── Packages/
└── Extensions/
```

## Architecture

```
Source → Lexer → Parser → AST → Evaluator → Value
```

| Module | Responsibility |
|--------|---------------|
| `lexer.rs` | Tokenizer with Wolfram-style multi-char operators |
| `parser.rs` | Recursive descent with precedence climbing |
| `ast.rs` | AST node definitions (`Expr` enum) |
| `eval.rs` | Tree-walk evaluator |
| `value.rs` | Runtime value types |
| `env.rs` | Lexical scoping (`Rc<RefCell<Scope>>`) |
| `pattern.rs` | Pattern matching engine |
| `builtins/` | Core library (arithmetic, list, string, math, symbolic, ...) |
| `rubi/` | Rubi rule-based integration engine |

### Design Principles

- **Operators are function calls**: `a + b` desugars to `Plus[a, b]`
- **Pattern-as-value**: Patterns in rules are stored as unevaluated AST
- **Lazy loading**: `Integrate` loads 185 Rubi rule files on first call
- **Everything is symbolic**: Numbers, strings, lists, rules, and functions are all first-class `Value` types

## Examples

See [`syma/examples/`](syma/examples/) for runnable examples:

- **basics/** — Variables, arithmetic, strings, lists, control flow
- **functional/** — Map/Fold/Select, patterns and rules
- **math/** — Trig, series, Newton's method, numerical integration, Monte Carlo
- **advanced/** — Modules, OOP
- **applied/** — Real-world usage

## Project Status

Syma is in active development. Current phase: **tree-walk interpreter with REPL**.

**Working:**
- Full lexer/parser with Wolfram-compatible syntax
- Pattern matching (blanks, sequences, type constraints, alternatives)
- 100+ built-in functions (arithmetic, list, string, math, I/O, filesystem, ...)
- Rule-based symbolic integration (Rubi)
- REPL with history
- Package scaffolding CLI (`syma new`, `syma run`, `syma test`)
- JSON import/export
- Parallel computation (`ParallelMap`, `ParallelTable`)
- Kernel mode for IDE integration
- DAP debug adapter

**Planned:**
- Bytecode compilation and JIT
- Modular `PackageManager` with lazy loading
- Standard library packages (LinearAlgebra, Statistics, Graphics)
- Native extensions via FFI
- Package registry

## License

MIT
