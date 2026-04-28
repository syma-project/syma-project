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
1 + 2 * 3          (* -> 7 *)
2^10                (* -> 1024 *)

(* Variables *)
x = 42;
x + 8               (* -> 50 *)

(* Strings *)
StringJoin["hello", " ", "world"]   (* -> "hello world" *)

(* Lists *)
{1, 2, 3}           (* -> List[1, 2, 3] *)
Length[{a, b, c}]    (* -> 3 *)
```

### Functions and Pattern Matching

```mathematica
(* Function definitions use patterns *)
f[x_] := x^2 + 1;
f[5]                  (* -> 26 *)

(* Multiple definitions — tried in order *)
fib[0] := 0;
fib[1] := 1;
fib[n_] := fib[n - 1] + fib[n - 2];
fib[10]               (* -> 55 *)

(* Type-constrained patterns *)
g[x_Integer] := x * 2;
g[x_String]  := StringJoin["got: ", x];
```

### Rules and Transformations

```mathematica
(* Rules: -> (immediate) and :> (delayed) *)
{x, y, x} /. x -> 0          (* -> {0, y, 0} *)

(* Pattern-based replacement *)
{1, 2, 3, 4} /. n_Integer -> n^2   (* -> {1, 4, 9, 16} *)
```

### Functional Programming

```mathematica
(* Map, Fold, Select *)
Map[f, {1, 2, 3}]              (* -> {f[1], f[2], f[3]} *)
f /@ {1, 2, 3}                 (* same as above *)
Select[{1, 2, 3, 4, 5}, EvenQ]
Fold[Plus, 0, {1, 2, 3, 4}]    (* -> 10 *)

(* Pure functions *)
(#^2 &) /@ {1, 2, 3}           (* -> {1, 4, 9} *)
```

### Symbolic Computation

```mathematica
(* Differentiation *)
D[x^3, x]                      (* -> 3 x^2 *)

(* Symbolic integration (via Rubi rules) *)
Integrate[x^2, x]              (* -> x^3 / 3 *)
Integrate[Sin[x], x]           (* -> -Cos[x] *)

(* Expand, Simplify, Factor *)
Expand[(x + 1)^3]              (* -> 1 + 3 x + 3 x^2 + x^3 *)
Factor[1 - 2 x + x^2]          (* -> (-1 + x)^2 *)

(* Series expansion *)
Series[Sin[x], {x, 0, 5}]
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

(* JSON import/export *)
Export["data.json", {1, 2, 3}]
Import["data.json"]

(* Syma Notebook format *)
Export["notebook.nb", expr, "NB"]
```

### Linear Algebra

```mathematica
(* Matrix operations *)
m = {{1, 2}, {3, 4}};
Det[m]              (* -> -2 *)
Inverse[m]
Transpose[m]
Eigenvalues[m]
Eigenvectors[m]

(* LinearSolve, Dot, Cross *)
LinearSolve[m, {a, b}]
```

### Number Theory

```mathematica
FactorInteger[84]   (* -> {{2, 2}, {3, 1}, {7, 1}} *)
GCD[12, 18]         (* -> 6 *)
Mod[17, 5]          (* -> 2 *)
PrimeQ[7]           (* -> True *)
Prime[100]          (* -> 541 *)
Divisible[15, 3]    (* -> True *)
```

### Statistics

```mathematica
Mean[{1, 2, 3, 4, 5}]          (* -> 3 *)
Median[{1, 2, 3, 4, 5}]        (* -> 3 *)
Variance[{1, 2, 3, 4, 5}]
StandardDeviation[{1, 2, 3, 4, 5}]
RandomReal[{0, 1}]              (* random float in [0, 1) *)
RandomInteger[{1, 10}]          (* random integer in [1, 10] *)
```

### Parallel Computation

```mathematica
ParallelMap[F, {1, 2, 3, 4, 5}]
ParallelTable[i^2, {i, 10}]
```

### FFI / Python Interop

```mathematica
(* Load native extension *)
LoadExtension["mylib"]

(* Python bridge *)
PythonEvaluate["import numpy as np"]
PythonEvaluate["np.array([1, 2, 3])"]
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
Source -> Lexer -> Parser -> AST -> Evaluator -> Value
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
| `bytecode/` | Bytecode compiler and VM |
| `jit/` | JIT compilation (experimental) |
| `ffi/` | Native extension loader and Python bridge |
| `profiler.rs` | Runtime profiler |

### Builtin Modules

| Module | Functions |
|--------|-----------|
| `arithmetic` | Plus, Times, Power, Minus, Divide, Abs |
| `list` | Length, Map, Fold, Select, Sort, Flatten, Join, Part, Table, Range, Nest — 60+ list ops |
| `string` | StringJoin, StringSplit, StringReplace, ToExpression, Characters — 30+ string ops |
| `math` | Sin, Cos, Tan, Log, Exp, Sqrt, ArcSin, ArcCos, ArcTan, Mod, Factorial, GCD, LCM — 65+ |
| `symbolic` | Simplify, Expand, Factor, D (differentiation), Integrate, Series, Solve |
| `number_theory` | Prime, PrimeQ, FactorInteger, DivisorSigma, EulerPhi, MoebiusMu, NextPrime — 25+ |
| `statistics` | Mean, Median, Variance, StandardDeviation, Quantile, Histogram — 35+ |
| `linalg` | Det, Inverse, Transpose, Eigenvalues, Eigenvectors, LinearSolve, Dot, Cross — 27+ |
| `association` | Association, Keys, Values, Lookup, KeyExistsQ, Merge, GroupBy — 23+ |
| `parallel` | ParallelMap, ParallelTable, ParallelCombine, ParallelEvaluate — 13+ |
| `filesystem` | FileExistsQ, ReadFile, WriteFile, CopyFile, DeleteFile, DirectoryName — 13+ |
| `image` | Image, ImageDimensions, ImageData, ImageTake, ImageRotate, ImageFilter — 12+ |
| `format` | InputForm, FullForm, TreeForm, TeXForm, MathMLForm — 11+ |
| `logical` | And, Or, Not, TrueQ, BooleanQ, BooleanConvert, BooleanTable — 11+ |
| `io` | Print, ReadString, WriteString, Import, Export, NB notebook support |
| `graphics` | Plot, ListPlot, Graphics, ColorData — 5+ |
| `dataset` | Dataset, Query, GroupBy, SortBy |
| `discrete` | DiscretePlot, Sum, Product, Table |
| `random` | RandomInteger, RandomReal, RandomChoice |
| `developer` | Developer utils, Packing, MachineIntegerQ |
| `names` | Names, ? (symbol info), Usage |
| `localsymbol` | LocalSymbol (scoped unique symbols) |
| `clearing` | Clear, ClearAll, Unset, Remove |
| `comparison` | Equal, Unequal, Less, Greater, LessEqual, GreaterEqual |
| `pattern` | MatchQ, Head, TypeOf, FreeQ, Replace |
| `error` | Throw, Catch, Error |

### Design Principles

- **Operators are function calls**: `a + b` desugars to `Plus[a, b]`
- **Implicit multiplication**: `x y` parses as `Times[x, y]`, `x x` as `Power[x, 2]`
- **Newlines as statement separators**: `\n` separates statements like `;`
- **Pattern-as-value**: Patterns in rules are stored as unevaluated AST
- **Lazy Rubi loading**: `Integrate` loads 185+ Rubi rule files on first call via `OnceLock`
- **Everything is symbolic**: Numbers, strings, lists, rules, and functions are all first-class `Value` types

## Examples

See [`syma/examples/`](syma/examples/) for runnable examples:

- **basics/** — Variables, arithmetic, strings, lists, control flow
- **functional/** — Map/Fold/Select, patterns and rules
- **math/** — Trig, series, Newton's method, numerical integration, Monte Carlo
- **advanced/** — Modules, OOP, custom data types
- **applied/** — Real-world usage, data analysis

## Project Status

Syma is in active development. Current phase: **tree-walk interpreter with REPL + bytecode VM**.

**Working:**
- Full lexer/parser with Wolfram-compatible syntax
- Pattern matching (blanks, sequences, type constraints, alternatives, guards)
- 390+ built-in functions across 28 modules
- Rule-based symbolic integration via Rubi (185+ rule files)
- Pattern-based symbolic simplification, expansion, and factoring
- Symbolic differentiation and series expansion
- REPL with history, multi-line input, syntax highlighting
- Package scaffolding CLI (`syma new`, `syma run`, `syma test`)
- Dependency management (`syma add`, `syma install`)
- JSON and native notebook format import/export
- Linear algebra (determinant, inverse, eigenvalues, solving systems)
- Number theory (primes, factorization, divisor functions)
- Statistics (descriptive stats, distributions)
- Parallel computation on thread pool
- Bytecode compiler with stack-based VM
- JIT compilation (experimental)
- Native extension loading via FFI
- Python bridge (eval Python, convert types)
- Image processing (I/O, filtering, transforms)
- Data visualization (plotting, graphics)
- Kernel mode (JSON protocol for IDE integration)
- DAP debug adapter protocol support
- Profiler with call-count and timing

**Planned:**
- Complete bytecode compilation for all language features
- Advanced JIT optimization
- Modular `PackageManager` with lazy loading
- Standard library packages (advanced linear algebra, advanced statistics)
- Package registry
- WASM target

## Test Suite

```bash
cargo xtask test  # 1000+ tests
```

## License

MIT
