# Syma Software Architecture

## From Monolith to Modular System

This document describes how Syma evolves from a single compiled binary into a
large, modular software system that loads functionality on demand — inspired by
how Mathematica organizes its Kernel, SystemFiles, and AddOns.

---

## 1. The Problem

Today, Syma compiles everything into one binary:

```
cargo build → syma (single executable)
  ├── lexer, parser, evaluator, pattern engine   ← always needed
   ├── 200+ builtins across 40 domains            ← always loaded
   ├── RUBI integration engine                    ← planned, not yet implemented
   └── FFI, parallel, IO subsystems               ← always linked
```

As the language grows — more builtins, more rule sets, format converters,
visualization, database access, external library bindings — this approach fails:

- **Binary size** grows linearly with features
- **Startup time** pays for everything even if you only use arithmetic
- **Compile time** increases
- **No user extensibility** — you can't add new system modules without
  recompiling

Mathematica solves this with a layered architecture. The `WolframKernel` binary
is lean; everything else lives under `SystemFiles/` and `AddOns/`, loaded on
first use.

---

## 2. Architecture Overview

Syma's architecture has four concentric layers:

```
┌─────────────────────────────────────────────────┐
│                User Packages                     │
│  ~/.syma/packages/  or  project dependencies     │
├─────────────────────────────────────────────────┤
│              Standard Library                    │
│  $SYMA_HOME/packages/  (LinearAlgebra, Stats…)  │
├─────────────────────────────────────────────────┤
│              System Files                        │
│  $SYMA_HOME/SystemFiles/                        │
│  ├── Kernel/    (builtin modules)               │
│  ├── Data/      (RUBI rules, constants)         │
│  ├── Formats/   (import/export converters)      │
│  └── Links/     (FFI bridges)                   │
├─────────────────────────────────────────────────┤
│              Core Kernel                         │
│  The compiled `syma` binary itself              │
│  (lexer, parser, evaluator, pattern engine,     │
│   env, PackageManager)                          │
└─────────────────────────────────────────────────┘
```

**Principle:** The core kernel knows how to *load* modules but does not contain
their implementations. Each layer can depend on layers below it, never above.

### What Lives Where

| Layer | Contents | Loaded |
|-------|----------|--------|
| **Core Kernel** | Lexer, parser, evaluator, pattern engine, `Env`, `PackageManager` | At startup (compiled in) |
| **SystemFiles/Kernel** | Builtin functions split by domain | On first use of any symbol from that domain |
| **SystemFiles/Data** | RUBI rules, periodic table, physical constants | On first use of the consuming function |
| **SystemFiles/Formats** | Import/Export converters (JSON, CSV, PNG…) | On first `Import["file.ext"]` |
| **SystemFiles/Links** | Python bridge, native library loader | On first `ExternalEvaluate` or `LoadLibrary` |
| **Packages/** | Standard library (LinearAlgebra, Statistics…) | On `Needs["LinearAlgebra"]` |
| **User Packages** | Third-party or user-written packages | On `Needs["MyPkg"]` or `import` |

---

## 3. The Installed Software Layout

When Syma is installed (or built from source), the directory structure looks like
this:

```
$SYMA_HOME/                          # e.g., /usr/local/share/syma/ or ~/.syma/
├── bin/
│   └── syma                         # Core kernel binary
├── SystemFiles/
│   ├── Kernel/
│   │   ├── init.syma                # Boot script: registers lazy providers
│   │   ├── arithmetic.syma          # Plus, Times, Power, Divide, Abs…
│   │   ├── comparison.syma          # Equal, Less, Greater…
│   │   ├── logical.syma             # And, Or, Not
│   │   ├── list.syma                # Length, First, Map, Table, Select…
│   │   ├── string.syma              # StringJoin, StringSplit, ToString…
│   │   ├── math.syma                # Sin, Cos, Log, Exp, Sqrt…
│   │   ├── pattern.syma             # MatchQ, Head, TypeOf, FreeQ
│   │   ├── association.syma         # Keys, Values, Lookup
│   │   ├── io.syma                  # Print, Import, Export, ReadString
│   │   ├── symbolic.syma            # Simplify, Expand, D, Factor, Solve
│   │   ├── random.syma              # RandomInteger, RandomReal
│   │   ├── filesystem.syma          # FileNames, FileExistsQ, FileNameJoin
│   │   ├── parallel.syma            # ParallelMap, ParallelTable
│   │   ├── control.syma             # If, Which, Switch, For, While, Do
│   │   └── ffi.syma                 # LoadLibrary, LoadExtension stubs
│   │
│   ├── Data/
│   │   ├── Rubi/
│   │   │   ├── index.toml           # Rule file index with categories
│   │   │   ├── 1 Algebraic functions/
│   │   │   │   ├── 1.1 Binomial products/
│   │   │   │   │   ├── 1.1.1 Linear.m
│   │   │   │   │   ├── 1.1.2 Quadratic.m
│   │   │   │   │   └── …
│   │   │   │   └── …
│   │   │   ├── 2 Exponentials/
│   │   │   ├── 3 Logarithms/
│   │   │   ├── 4 Trig functions/
│   │   │   └── …
│   │   ├── Chemistry/
│   │   │   └── periodic_table.toml
│   │   └── Physics/
│   │       └── constants.toml
│   │
│   ├── Formats/
│   │   ├── registry.toml            # Extension → format mapping
│   │   ├── json.syma                # JSON import/export
│   │   ├── csv.syma                 # CSV import/export
│   │   └── …
│   │
│   └── Links/
│       ├── Python/
│       │   └── bridge.syma          # Python subprocess bridge
│       └── Native/
│           └── loader.syma          # dlopen/dlsym wrapper
│
├── Packages/                        # Standard library packages
│   ├── LinearAlgebra/
│   │   ├── syma.toml
│   │   └── src/
│   │       └── LinearAlgebra.syma
│   ├── Statistics/
│   │   ├── syma.toml
│   │   └── src/
│   │       └── Statistics.syma
│   ├── Graphics/
│   │   ├── syma.toml
│   │   └── src/
│   │       └── Graphics.syma
│   └── …
│
└── Extensions/                      # Native plugins (Tier 3 FFI)
    ├── syma-linalg/
    │   ├── syma.toml
    │   └── lib/syma_linalg.dylib
    └── …
```

### Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `SYMA_HOME` | `~/.syma` | Root of the installation |
| `SYMA_PATH` | `$SYMA_HOME/Packages` | Additional package search paths |
| `SYMA_SYSTEM` | `$SYMA_HOME/SystemFiles` | System files root |

The kernel discovers `SystemFiles/` relative to its own binary path (like
Mathematica's `$InstallationDirectory`), falling back to `SYMA_HOME`.

---

## 4. Package Format

Every package — system module, standard library package, or user package —
follows the same `syma.toml` manifest format.

### 4.1 Manifest: `syma.toml`

```toml
[package]
name        = "LinearAlgebra"
version     = "1.2.0"
description = "Matrix operations, eigenvalues, decompositions"
authors     = ["Syma Contributors"]
license     = "MIT"
entry       = "src/LinearAlgebra.syma"   # library entry point

[dependencies]
Numerics = "^1.0"

[provides]
# Symbols this package defines. Used by the PackageManager for lazy loading.
# When any of these symbols is first referenced, this package is loaded.
symbols = [
    "MatrixMultiply", "Inverse", "Det", "Eigenvalues",
    "Transpose", "IdentityMatrix", "LinearSolve",
    "Dot", "Cross", "Norm", "Tr", "MatrixPower"
]

[provides.category]
# Optional: associate symbols with categories for documentation and search.
"Matrix Operations" = ["MatrixMultiply", "Inverse", "Det", "Transpose"]
"Decompositions"    = ["Eigenvalues", "LinearSolve"]
```

**Key fields:**

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Package identifier (PascalCase recommended) |
| `version` | Yes | Semver version string |
| `entry` | No | Entry point file (default: `src/main.syma` for binary, `src/lib.syma` for library) |
| `provides.symbols` | No | List of symbols this package defines — used for lazy loading |
| `dependencies` | No | Other packages this depends on |

### 4.2 System Module Manifest

System modules under `SystemFiles/Kernel/` use a lighter-weight manifest
embedded in the `init.toml` index file:

```toml
# SystemFiles/Kernel/init.toml

[[module]]
name = "arithmetic"
file = "arithmetic.syma"
symbols = ["Plus", "Times", "Power", "Divide", "Minus", "Abs"]
attributes = { Plus = ["Flat", "Listable", "NumericFunction", "OneIdentity", "Orderless"] }

[[module]]
name = "list"
file = "list.syma"
symbols = [
    "Length", "First", "Last", "Rest", "Most", "Append", "Prepend",
    "Join", "Flatten", "Sort", "Reverse", "Part", "Range", "Table",
    "Map", "Fold", "Select", "Scan", "Nest", "Take", "Drop",
    "Riffle", "Transpose", "Total", "Sum", "MemberQ", "Count",
    "Position", "Union", "Intersection", "Complement", "Tally",
    "PadLeft", "PadRight"
]

[[module]]
name = "symbolic"
file = "symbolic.syma"
symbols = ["Simplify", "Expand", "D", "Integrate", "Factor", "Solve", "Series"]

[[module]]
name = "rubi"
file = "../Data/Rubi/index.toml"   # Points to the RUBI rule index
symbols = ["Integrate"]            # Overrides the stub in symbolic
lazy = true                        # Load only on first Integrate call

# … more modules …
```

### 4.3 RUBI Rule Index

```toml
# SystemFiles/Data/Rubi/index.toml

[engine]
version = "4.16.1.0"
total_rules = 185

[[category]]
name = "Algebraic functions"
path = "1 Algebraic functions"
subcategories = [
    { name = "Binomial products", path = "1.1 Binomial products" },
    { name = "Trinomial products", path = "1.2 Trinomial products" },
    { name = "Miscellaneous", path = "1.3 Miscellaneous" },
]

[[category]]
name = "Exponentials"
path = "2 Exponentials"

# …
```

---

## 5. Module Loading System

### 5.1 The PackageManager

The core of the modular architecture is the `PackageManager`. It replaces the
current flat `register_builtins()` call with an intelligent loader.

```rust
// Conceptual design — not yet implemented

/// Metadata about a loadable module.
pub struct ModuleInfo {
    /// Module name (e.g., "arithmetic", "rubi")
    pub name: String,
    /// Path to the module file or index
    pub path: PathBuf,
    /// Symbols this module provides
    pub symbols: Vec<String>,
    /// Whether to load eagerly at startup or lazily on first use
    pub lazy: bool,
    /// Module kind — determines how it's loaded
    pub kind: ModuleKind,
}

pub enum ModuleKind {
    /// A .syma source file — parsed and evaluated
    SymaSource,
    /// A RUBI rule index — parsed into RubiEngine
    RubiRules,
    /// A native extension (.dylib/.so/.dll) — loaded via syma_init ABI
    NativeExtension,
    /// A data file (.toml) — loaded into a Value::Assoc
    DataFile,
}

/// Manages module discovery, loading, and caching.
pub struct PackageManager {
    /// All known modules (from init.toml + package discovery)
    modules: Vec<ModuleInfo>,
    /// Symbol → module index mapping (built from modules[].symbols)
    symbol_index: HashMap<String, usize>,
    /// Already-loaded modules (name → Value)
    loaded: HashMap<String, Value>,
    /// Search paths for user packages
    search_paths: Vec<PathBuf>,
}
```

### 5.2 Loading Flow

When the evaluator encounters an undefined symbol:

```
eval(Expr::Symbol("Integrate"))
  │
  ├── env.get("Integrate") → None
  │
  ├── Check lazy_providers (current mechanism)
  │   └── Found? → fire provider, install value, return
  │
  ├── Check PackageManager.symbol_index
  │   ├── Found module "rubi" for symbol "Integrate"
  │   ├── Load module (parse .m rules, init RubiEngine)
  │   ├── Install all symbols from that module into env
  │   └── Return the value for "Integrate"
  │
  └── Not found anywhere → return Value::Symbol("Integrate")
```

The key insight: **loading a module installs ALL its symbols at once**. When
`Integrate` triggers loading of the `rubi` module, all RUBI-related helpers
(`Int`, `Subst`, etc.) become available too. No redundant loads.

### 5.3 Startup Sequence

```
syma binary starts
  │
  ├── 1. Initialize core (lexer, parser, evaluator, env)
  │
  ├── 2. Read SystemFiles/Kernel/init.toml
  │      Build PackageManager with all module metadata
  │      Build symbol_index from all [module].symbols
  │
  ├── 3. Register "always-load" modules (control flow: If, Which, Switch, For…)
  │      These are needed immediately and are small.
  │
  ├── 4. Register lazy providers for everything else
  │      Each symbol → PackageManager::load_module(name)
  │
  └── 5. Ready. First use of any symbol triggers its module load.
```

### 5.4 Builtins Migration Strategy

Today's `builtins/mod.rs` has a single `register_builtins()` that registers 200+
functions. The migration splits this into per-domain loader functions:

```rust
// Before (current):
pub fn register_builtins(env: &Env) {
    register_builtin(env, "Plus", arithmetic::builtin_plus);
    register_builtin(env, "Times", arithmetic::builtin_times);
    // ... 100 more ...
}

// After (modular):
// Each module file is a standalone .syma file that defines its functions.
// The PackageManager loads them on demand.
//
// SystemFiles/Kernel/arithmetic.syma:
//   Plus[a_, b_] := builtin_add(a, b)   (* registered via init.toml *)
//   Times[a_, b_] := builtin_mul(a, b)
//   ...
//
// Or, for performance-critical builtins, the init.toml maps directly:
//   [[module]]
//   name = "arithmetic"
//   builtin = true   # loaded from compiled Rust, not .syma source
//   symbols = ["Plus", "Times", ...]
```

For builtins that must remain as compiled Rust functions (for performance), the
init.toml marks them as `builtin = true`. The PackageManager calls the
corresponding Rust registration function instead of parsing a `.syma` file.

This hybrid approach keeps performance-critical paths (arithmetic, comparison)
as compiled code while moving documentation-heavy modules (symbolic, IO) to
`.syma` source.

### 5.5 Runtime Flow: `Integrate[x^2, x]`

To make the architecture concrete, here is the complete sequence of events when
a user types `Integrate[x^2, x]` in the REPL — from first keystroke to final
result.

#### Step 1: Startup (once, when `syma` launches)

```
$ syma
│
├─ main() → run_repl()
│   │
│   ├─ Env::new()                          // Fresh environment, empty scope
│   │
│   ├─ PackageManager::init()              // NEW: replaces register_builtins()
│   │   │
│   │   ├─ Locate SystemFiles/Kernel/init.toml
│   │   │   (relative to binary path, or $SYMA_HOME/SystemFiles/)
│   │   │
│   │   ├─ Parse init.toml → Vec<ModuleInfo>
│   │   │   e.g., ModuleInfo { name: "arithmetic", symbols: ["Plus","Times",...], lazy: true }
│   │   │        ModuleInfo { name: "rubi", symbols: ["Integrate"], lazy: true }
│   │   │        ...
│   │   │
│   │   ├─ Build symbol_index: HashMap<String, usize>
│   │   │   "Plus"      → 0  (index into modules[])
│   │   │   "Times"     → 0
│   │   │   "Length"    → 1  (list module)
│   │   │   "Integrate" → 7  (rubi module)
│   │   │   ...
│   │   │
│   │   └─ Register eager modules (control flow: If, Which, For, While)
│   │       These are small and always needed for REPL interaction.
│   │       register_control_flow(&env)  // puts If, Which, Switch, For, While, Do
│   │
│   └─ REPL ready. Prompt: In [1]: _
```

At this point, `Integrate` is **not** in the environment. Neither is `Plus`,
`Map`, or any other domain-specific function. Only control flow symbols exist.

#### Step 2: User types `Integrate[x^2, x]`

```
In [1]: Integrate[x^2, x]
│
├─ eval_input("Integrate[x^2, x]", &env)
│   │
│   ├─ lexer::tokenize("Integrate[x^2, x]")
│   │   → [Ident("Integrate"), LBracket, Ident("x"), Caret, Integer(2),
│   │      Comma, Ident("x"), RBracket]
│   │
│   ├─ parser::parse(tokens)
│   │   → Expr::Call {
│   │       head: Expr::Symbol("Integrate"),
│   │       args: [
│   │           Expr::Call { head: Symbol("Power"), args: [Symbol("x"), Integer(2)] },
│   │           Symbol("x")
│   │       ]
│   │     }
│   │
│   └─ eval::eval(&ast, &env)  ← this is where the interesting part happens
```

#### Step 3: Evaluator resolves `Integrate`

```
eval(Call { head: Symbol("Integrate"), args: [Power[x, 2], x] })
│
├─ First: evaluate the arguments
│   │
│   ├─ eval(Symbol("Power"), env)
│   │   ├─ env.get("Power") → None   // not loaded yet
│   │   ├─ PackageManager::resolve("Power")
│   │   │   ├─ symbol_index["Power"] → modules[0] ("arithmetic")
│   │   │   ├─ load_module("arithmetic")
│   │   │   │   ├─ init.toml says: builtin = true
│   │   │   │   ├─ call register_arithmetic(&env)
│   │   │   │   │   ├─ env.set("Plus", Builtin("Plus", builtin_plus))
│   │   │   │   │   ├─ env.set("Times", Builtin("Times", builtin_times))
│   │   │   │   │   ├─ env.set("Power", Builtin("Power", builtin_power))
│   │   │   │   │   ├─ env.set("Divide", Builtin("Divide", builtin_divide))
│   │   │   │   │   └─ env.set("Abs", Builtin("Abs", builtin_abs))
│   │   │   │   └─ mark modules[0] as loaded
│   │   │   └─ return env.get("Power") → Some(Builtin("Power", fn))
│   │   └─ Value::Builtin("Power", builtin_power)
│   │
│   ├─ eval(Symbol("x"), env)
│   │   └─ env.get("x") → None → return Value::Symbol("x")
│   │
│   ├─ eval(Integer(2), env)
│   │   └─ Value::Integer(2)
│   │
│   └─ apply_function(Builtin("Power"), [Symbol("x"), Integer(2)], env)
│       └─ builtin_power(&[Symbol("x"), Integer(2)])
│           → x^2 is symbolic, can't compute numerically
│           → Return Value::Call { head: "Power", args: [Symbol("x"), Integer(2)] }
│
├─ Now evaluate the outer call: Integrate
│   │
│   ├─ eval(Symbol("Integrate"), env)
│   │   ├─ env.get("Integrate") → None   // not loaded yet
│   │   ├─ PackageManager::resolve("Integrate")
│   │   │   ├─ symbol_index["Integrate"] → modules[7] ("rubi")
│   │   │   ├─ load_module("rubi")
│   │   │   │   │
│   │   │   │   │   ┌─────────────────────────────────────────────────┐
│   │   │   │   │   │  LOADING THE RUBI MODULE                       │
│   │   │   │   │   │                                                 │
│   │   │   │   │   │  1. Read SystemFiles/Data/Rubi/index.toml       │
│   │   │   │   │   │     → get list of .m rule file paths            │
│   │   │   │   │   │                                                 │
│   │   │   │   │   │  2. For each .m file:                           │
│   │   │   │   │   │     a. Read file from disk                      │
│   │   │   │   │   │     b. Parse with WLParser                     │
│   │   │   │   │   │        → Vec<IntRule> (pattern, condition,      │
│   │   │   │   │   │          result triples)                        │
│   │   │   │   │   │     c. Append rules to RubiEngine               │
│   │   │   │   │   │                                                 │
│   │   │   │   │   │  3. Store RubiEngine in global OnceLock         │
│   │   │   │   │   │                                                 │
│   │   │   │   │   │  4. Create Integrate builtin:                   │
│   │   │   │   │   │     fn integrate_builtin(args) {                │
│   │   │   │   │   │         engine.lock().integrate(args[0], args[1])│
│   │   │   │   │   │     }                                           │
│   │   │   │   │   │                                                 │
│   │   │   │   │   │  5. env.set("Integrate", Builtin(fn))           │
│   │   │   │   │   │                                                 │
│   │   │   │   │   │  6. Mark modules[7] as loaded                   │
│   │   │   │   │   └─────────────────────────────────────────────────┘
│   │   │   │   │
│   │   │   │   └─ Total time: ~50-200ms (parsing 185 .m files)
│   │   │   └─ return env.get("Integrate") → Some(Builtin("Integrate", fn))
│   │   └─ Value::Builtin("Integrate", integrate_builtin)
│   │
│   └─ apply_function(Builtin("Integrate", fn),
│                      [Call{Power, [x, 2]}, Symbol("x")], env)
│       │
│       └─ integrate_builtin(&[Power[x,2], x])
│           │
│           ├─ engine.lock().integrate(&Power[x,2], "x")
│           │   │
│           │   ├─ Try rule 1: pattern = _Blank, no match on Power[x,2]
│           │   ├─ Try rule 2: pattern = Power[x_, n_Integer], match!
│           │   │   bindings = { "x": Symbol("x"), "n": Integer(2) }
│           │   │   condition: IGtQ[n, 0] → 2 > 0 → true
│           │   │   result: Times[Power[x, Plus[n, 1]], Power[Plus[n, 1], -1]]
│           │   │
│           │   └─ eval_result with bindings:
│           │       → Times[Power[x, Plus[2, 1]], Power[Plus[2, 1], -1]]
│           │       → Times[Power[x, 3], Power[3, -1]]
│           │       → Call{Times, [Call{Power, [x, 3]}, Call{Power, [3, -1]}]}
│           │
│           └─ Return Value::Call {
│                head: "Times",
│                args: [
│                  Call{ head: "Power", args: [Symbol("x"), Integer(3)] },
│                  Call{ head: "Power", args: [Integer(3), Integer(-1)] }
│                ]
│              }
│
└─ eval_input prints result:
    Out[1]: x^3 / 3
    (displayed as Times[Power[x, 3], Power[3, -1]] → simplified to x^3/3)
```

#### Step 4: Subsequent calls are instant

```
In [2]: Integrate[x^3, x]
│
├─ eval(Symbol("Integrate"), env)
│   └─ env.get("Integrate") → Some(Builtin("Integrate", fn))  // already loaded!
│
├─ apply_function → integrate_builtin(&[Power[x,3], x])
│   └─ engine already has all 185 rule files parsed
│   └─ Pattern match → result: x^4/4
│
└─ Out[2]: x^4/4
```

**Key insight:** The first `Integrate` call pays the loading cost (~50-200ms).
Every subsequent call is just a pattern match against already-parsed rules.
The user never sees `Integrate` as undefined — it appears the moment they
need it.

#### Summary Timeline

```
Time ─────────────────────────────────────────────────────────►

│ Startup          │ In[1]: Integrate[x^2, x]              │ In[2]: ...
│                  │                                        │
│ Parse init.toml  │ Lex + Parse                           │ Lex + Parse
│ Build index      │ eval → resolve "Integrate"            │ eval → "Integrate" found!
│ Register control │   ├─ resolve "Power" → load arithmetic│ apply_function
│ flow builtins    │   │  └─ register 5 builtins (~0ms)    │   └─ pattern match (~ms)
│                  │   └─ load rubi module                  │
│                  │       ├─ read 185 .m files from disk   │
│                  │       ├─ parse into IntRules           │
│                  │       ├─ init RubiEngine               │
│                  │       └─ register Integrate builtin    │
│                  │ apply_function                         │
│                  │   └─ pattern match → x^3/3            │
│ ~10ms            │ ~50-200ms (one-time)                   │ ~1ms
```

---

## 7. Standard Library Packages

Standard library packages live under `$SYMA_HOME/Packages/` and follow the
standard package format.

### 7.1 Example: LinearAlgebra

```
$SYMA_HOME/Packages/LinearAlgebra/
├── syma.toml
└── src/
    └── LinearAlgebra.syma
```

```toml
# syma.toml
[package]
name    = "LinearAlgebra"
version = "1.0.0"
entry   = "src/LinearAlgebra.syma"

[provides]
symbols = [
    "MatrixMultiply", "Inverse", "Det", "Eigenvalues",
    "Transpose", "IdentityMatrix", "LinearSolve",
    "Dot", "Cross", "Norm"
]
```

```syma
(* src/LinearAlgebra.syma *)

module LinearAlgebra {
    export MatrixMultiply, Inverse, Det, Eigenvalues,
           Transpose, IdentityMatrix, LinearSolve,
           Dot, Cross, Norm

    (* Matrix multiplication *)
    MatrixMultiply[a_, b_] := ...

    (* Matrix inverse *)
    Inverse[m_] := ...

    (* Determinant via LU decomposition *)
    Det[m_] := ...

    (* ... *)
}
```

### 7.2 Loading a Package

```syma
(* In user code: *)

Needs["LinearAlgebra"]
(* → PackageManager finds LinearAlgebra in Packages/ *)
(* → Loads and evaluates src/LinearAlgebra.syma *)
(* → All exported symbols become available *)

m = {{1, 2}, {3, 4}}
Inverse[m]
(* → Calls LinearAlgebra::Inverse *)
```

Or with explicit import:

```syma
import LinearAlgebra.{Inverse, Det}

Det[{{1, 2}, {3, 4}}]
```

---

## 8. Native Extensions (Tier 3 FFI)

Native extensions follow the existing `syma_init` ABI from `ffi/extension.rs`.
They are discovered by the PackageManager via `Extensions/` directory scanning.

```
$SYMA_HOME/Extensions/syma-linalg/
├── syma.toml
├── lib/
│   ├── syma_linalg.dylib          # macOS
│   ├── syma_linalg.so             # Linux
│   └── syma_linalg.dll            # Windows
└── src/                           # Optional: source for building
    └── lib.rs
```

```toml
# syma.toml
[package]
name    = "syma-linalg"
version = "0.1.0"
type    = "native-extension"

[provides]
symbols = ["FastEigenvalues", "FastSVD"]

[native]
lib_name = "syma_linalg"
```

The extension's `syma_init` function registers its builtins via the C ABI:

```rust
// In the extension crate
#[no_mangle]
pub unsafe extern "C" fn syma_init(ctx: *mut SymaExtensionContext) {
    let register = (*ctx).register_fn;
    register(ctx, c"FastEigenvalues\0".as_ptr(), fast_eigenvalues);
    register(ctx, c"FastSVD\0".as_ptr(), fast_svd);
}
```

---

## 9. Format Converters (Import/Export)

Format converters live in `SystemFiles/Formats/` and are loaded on first
`Import` or `Export` call with a matching file extension.

```toml
# SystemFiles/Formats/registry.toml

[[format]]
extension = "json"
module    = "json.syma"
symbols   = ["ImportJSON", "ExportJSON"]

[[format]]
extension = "csv"
module    = "csv.syma"
symbols   = ["ImportCSV", "ExportCSV"]

[[format]]
extension = ["png", "jpg", "jpeg", "gif"]
module    = "image.syma"
symbols   = ["ImportImage", "ExportImage"]
```

When `Import["data.csv"]` is called:

1. PackageManager checks `Formats/registry.toml` for extension `"csv"`
2. Loads `csv.syma` (which defines `ImportCSV` and `ExportCSV`)
3. Calls `ImportCSV["data.csv"]`
4. Returns the result

---

## 10. Migration Path

Moving from the current monolith to this modular architecture is a gradual
process. Each step is independently shippable.

### Phase 1: PackageManager Skeleton

- Create `src/package_manager.rs` with `ModuleInfo`, `ModuleKind`, `PackageManager`
- Create `SystemFiles/Kernel/init.toml` with the current builtin inventory
- `PackageManager` reads `init.toml` and populates `symbol_index`
- `register_builtins()` still registers everything eagerly (no behavior change)
- **Goal:** Infrastructure exists, tested in isolation

### Phase 2: Lazy Builtin Loading

- Split `register_builtins()` into per-domain functions:
  `register_arithmetic()`, `register_list()`, `register_string()`, etc.
- `PackageManager` registers lazy providers for each domain
- First use of `Map` triggers `register_list()`; first use of `Sin` triggers
  `register_math()`
- Control flow builtins (`If`, `Which`, `For`, `While`) stay eager (always needed)
- **Goal:** Startup time reduced, binary still monolithic

### Phase 3: RUBI Integration

- Implement RubiEngine that reads `.m` rule files from `SystemFiles/Data/Rubi/`
- Wire `Integrate` to call RubiEngine on first use (lazy loading)
- `init.toml` declares the `rubi` module with `lazy = true`
- **Goal:** RUBI rules are an on-disk data dependency loaded at runtime

### Phase 4: Standard Library Packages

- Create `$SYMA_HOME/Packages/` directory structure
- Implement `Needs["PackageName"]` in the evaluator
- Move domain-specific functionality to packages:
  - `LinearAlgebra` — matrix operations
  - `Statistics` — statistical functions
  - `Graphics` — plotting and visualization
- **Goal:** Packages are loadable, core binary is lean

### Phase 5: Native Extensions Discovery

- `PackageManager` scans `$SYMA_HOME/Extensions/` for native libraries
- Auto-registers extensions that declare `provides.symbols`
- Extensions loaded lazily on first symbol reference
- **Goal:** Third-party native code integrates seamlessly

---

## 11. Comparison with Mathematica

| Aspect | Mathematica | Syma (target) |
|--------|------------|---------------|
| Core binary | `WolframKernel` | `syma` |
| System init | `init.m` | `SystemFiles/Kernel/init.toml` |
| Builtin modules | `SystemFiles/Kernel/` `.m` files | `SystemFiles/Kernel/` `.syma` files |
| Rule databases | `SystemFiles/Data/` | `SystemFiles/Data/Rubi/` |
| Packages | `$UserBaseDirectory/Applications/` | `$SYMA_HOME/Packages/` |
| Extensions | WSTP/MathLink | `syma_init` ABI |
| Loading trigger | `Needs[]`, first use of symbol | `Needs[]`, first use of symbol |
| Package format | `PacletInfo.m` | `syma.toml` |

---

## 12. Summary

```
                     ┌─────────────────┐
                     │   User Code     │
                     │  import / Needs │
                     └────────┬────────┘
                              │
              ┌───────────────┼───────────────┐
              │               │               │
     ┌────────▼──┐   ┌───────▼────┐   ┌──────▼──────┐
     │ Packages/ │   │ SystemFiles│   │ Extensions/ │
     │ .syma src │   │  .syma +   │   │  .dylib/.so │
     │ + toml    │   │  .m data   │   │  + toml     │
     └────────┬──┘   └───────┬────┘   └──────┬──────┘
              │               │               │
              └───────────────┼───────────────┘
                              │
                     ┌────────▼────────┐
                     │  PackageManager │
                     │  symbol_index   │
                     │  lazy loading   │
                     └────────┬────────┘
                              │
                     ┌────────▼────────┐
                     │   Core Kernel   │
                     │ lexer, parser,  │
                     │ eval, pattern,  │
                     │ env             │
                     └─────────────────┘
```

The core principle: **the kernel knows how to load, not what to load.** All
domain knowledge — builtins, rules, data, converters — lives in the filesystem
and is loaded on demand. The binary stays small. The system grows by adding
files, not by recompiling.
