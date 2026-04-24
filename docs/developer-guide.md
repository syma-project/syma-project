# Syma Developer Guide

This guide helps you understand the Syma codebase, add new features, and avoid
common pitfalls. It assumes you're comfortable with Rust (edition 2024) and have
some familiarity with Wolfram Language concepts.

---

## 1. Codebase Map

```
syma/                         # Language crate
├── src/
│   ├── main.rs               # CLI entry point, REPL loop
│   ├── lib.rs                # Crate root, re-exports
│   ├── lexer.rs              # Tokenizer
│   ├── parser.rs             # Recursive-descent precedence parser
│   ├── ast.rs                # Expr enum — the universal AST node (40+ variants)
│   ├── eval/
│   │   ├── mod.rs            # Core eval dispatch, apply_function, try_match_params
│   │   ├── rules.rs          # Rule application (ReplaceAll, ReplaceRepeated)
│   │   ├── numeric.rs        # N[] — high-precision numeric evaluation
│   │   ├── table.rs          # Table, Sum, Do, ParallelTable iterators
│   │   └── plot.rs           # Plot — function sampling and Graphics construction
│   ├── value.rs              # Value enum — runtime types
│   ├── pattern.rs            # Pattern matching engine (blanks, sequences, guards)
│   ├── env.rs                # Lexical scoping via Rc<RefCell<Scope>> chains
│   ├── builtins/
│   │   ├── mod.rs            # register_builtins() orchestrator, help system
│   │   ├── arithmetic.rs     # Plus, Times, Power, Divide, Minus, Abs
│   │   ├── comparison.rs     # Equal, Unequal, Less, Greater, etc.
│   │   ├── logical.rs        # And, Or, Not
│   │   ├── list.rs           # Length, Map, Fold, Select, Table, etc.
│   │   ├── string.rs         # StringJoin, StringSplit, StringReplace, etc.
│   │   ├── math.rs           # Sin, Cos, Log, Exp, Sqrt, Floor, GCD, etc.
│   │   ├── pattern.rs        # MatchQ, Head, TypeOf, FreeQ
│   │   ├── association.rs    # Keys, Values, Lookup, KeyExistsQ
│   │   ├── symbolic.rs       # Simplify, Expand, D (differentiation), Integrate, etc.
│   │   ├── random.rs         # RandomInteger, RandomReal, RandomChoice
│   │   ├── graphics.rs       # SVG renderer, GraphicsStyle directives, ListPlot
│   │   ├── io/
│   │   │   ├── mod.rs        # Print, Input, Write, ReadString, WriteString, PrintF
│   │   │   ├── export.rs     # Export — dispatches by extension (.svg, .json, …)
│   │   │   ├── import.rs     # Import — dispatches by extension (.json, .nb, .m, …)
│   │   │   └── nb.rs         # .nb notebook parser (WL expression → code extraction)
│   │   ├── filesystem.rs     # FileNames, FileExistsQ, FileNameJoin, DirectoryQ, etc.
│   │   ├── ffi.rs            # ExternalEvaluate, LoadExtension
│   │   ├── parallel.rs       # ParallelMap, ParallelTable
│   │   ├── error.rs          # Throw, Error
│   │   ├── linalg.rs         # Matrix operations
│   │   ├── statistics.rs     # Mean, Median, Variance, StandardDeviation
│   │   ├── localsymbol.rs    # LocalSymbol — persistent cross-session storage
│   │   ├── format.rs         # InputForm/OutputForm string formatting
│   │   └── random.rs         # Random primitives
│   ├── ffi/
│   │   ├── mod.rs            # FFI module root
│   │   ├── extension.rs      # Native extension loader (dlopen/dlsym)
│   │   ├── loader.rs         # Dynamic library loading helpers
│   │   ├── marshal.rs        # JSON ↔ Value marshalling
│   │   └── python.rs         # Python bridge (subprocess)
│   ├── cli.rs                # Package scaffolding commands
│   ├── debug.rs              # DAP (Debug Adapter Protocol)
│   ├── kernel.rs             # JSON-over-stdin/stdout kernel mode
│   ├── format.rs             # Terminal formatting
│   └── manifest.rs           # syma.toml manifest parser
├── tests/
│   └── cli.rs                # Integration tests (runs .syma example files)
├── examples/                 # Runnable example .syma files
└── CLAUDE.md                 # Project conventions for Claude Code

xtask/                        # Build system (cargo-xtask)
├── src/
│   ├── main.rs               # Subcommands: build, install, dist, test, lint, clean
│   └── setup.rs              # SystemFiles skeleton creation

docs/
├── software-architecture.md  # Target modular architecture (future state)
└── developer-guide.md        # This file
```

### What Each Pipeline Stage Does

```
Source (text) ──► Lexer ──► Tokens ──► Parser ──► AST (Expr) ──► Evaluator ──► Value
```

| Stage | File | Input | Output |
|-------|------|-------|--------|
| **Lexer** | `lexer.rs` | Source text | `Vec<SpannedToken>` |
| **Parser** | `parser.rs` | Tokens | `Vec<Expr>` (or single `Expr`) |
| **Evaluator** | `eval/mod.rs` | `&Expr` + `&Env` | `Result<Value, EvalError>` |

---

## 2. Core Data Types

### `Expr` (AST — `ast.rs`)

The universal compile-time representation. Every Syma expression — from a simple
integer to a complex function call — is an `Expr` variant.

```rust
pub enum Expr {
    // Atoms
    Integer(rug::Integer),
    Real(rug::Float),
    Complex { re: f64, im: f64 },
    Str(String),
    Bool(bool),
    Symbol(String),
    Null,

    // Compound
    Call { head: Box<Expr>, args: Vec<Expr> },  // f[arg1, arg2, ...]
    List(Vec<Expr>),                              // {a, b, c}
    Assoc(Vec<(String, Expr)>),                   // <| key -> val |>
    Rule { lhs: Box<Expr>, rhs: Box<Expr> },      // a -> b
    RuleDelayed { lhs: Box<Expr>, rhs: Box<Expr> }, // a :> b

    // Function constructs
    Slot(Option<usize>),                           // #, #1, #2
    Function { params: Vec<Symbol>, body: Box<Expr> },

    // Patterns (parsed but stored as AST)
    Blank { type_constraint: Option<Symbol> },     // _, _Integer
    NamedBlank { name: Symbol, type_constraint: Option<Symbol> },  // x_, x_Integer
    BlankSequence { .. },  // __, x__
    BlankNullSequence { .. },  // ___, x___
    PatternGuard { pattern: Box<Expr>, condition: Box<Expr> },  // /;
    OptionalBlank { .. }, OptionalNamedBlank { .. },  // _., x_.

    // Special forms
    ReplaceAll { .. }, ReplaceRepeated { .. },  // /., //.
    Map { .. }, Apply { .. },  // /@, @@
    Pipe { .. }, Prefix { .. },  // //, @
    If { .. }, Which { .. }, Switch { .. }, For { .. }, While { .. }, Do { .. },

    // Definitions
    FuncDef { name, params, body, delayed },  // f[x_] := ...
    Assign { lhs, rhs },  // x = val
    DestructAssign { .. },  // {a, b} = val
    RuleDef { name, rules },  // rule name = { ... }

    // Module system (parsed, eval is minimal)
    ClassDef { .. }, ModuleDef { .. },
    Import { .. }, Export { .. },

    // Sequences and holds
    Sequence(Vec<Expr>),
    Hold(Box<Expr>), HoldComplete(Box<Expr>), ReleaseHold(Box<Expr>),
    Information(Box<Expr>),  // ?expr
}
```

### `Value` (Runtime — `value.rs`)

The runtime representation. Most `Expr` nodes evaluate to `Value` nodes of the
same shape, but there are important differences:

```rust
pub enum Value {
    // Atoms (same as Expr)
    Integer(rug::Integer), Real(rug::Float), Complex { re: f64, im: f64 },
    Str(String), Bool(bool), Symbol(String), Null,

    // Containers (same shape as Expr)
    List(Vec<Value>),
    Call { head: String, args: Vec<Value> },  // head is String, not Expr
    Assoc(HashMap<String, Value>),

    // Callable values (do NOT exist in Expr)
    Function { params: Vec<PatternExpr>, body: Expr, env: Env },
    Builtin(String, Arc<BuiltinFn>),
    PureFunction { body: Expr, env: Env },

    // Other runtime-only values
    Rule { lhs: Box<Value>, rhs: Box<Value> },
    RuleDelayed { lhs: Box<Value>, rhs: Box<Value> },
    Object { class: String, fields: HashMap<String, Value> },
    RuleSet(Vec<Value>),
    Pattern(Expr),
    Sequence(Vec<Value>),  // auto-splats in lists/calls
    Slice { list: Box<Value>, indices: Vec<Value> },
}
```

Key distinction between `Expr` and `Value`:

| Feature | `Expr` | `Value` |
|---------|--------|---------|
| `Call` head | `Box<Expr>` (any expression) | `String` (always a symbol name) |
| Function defs | `FuncDef` variant | `Function { params, body, env }` |
| Builtins | N/A | `Builtin(name, Arc<BuiltinFn>)` |
| Patterns | `Blank`, `NamedBlank`, etc. | `Pattern(Expr)` wrapping raw AST |
| `Sequence` | Semi-colons in source | Auto-splats in lists/function args |
| Evaluation | Compile-time representation | Post-evaluation result |

### `Env` (Environment — `env.rs`)

Lexical scoping via `Rc<RefCell<Scope>>` chains:

```rust
pub struct Scope {
    pub bindings: HashMap<String, Value>,  // All definitions live here
    pub parent: Option<Env>,
    pub depth: usize,
}
pub struct Env(pub Rc<RefCell<Scope>>);
```

- `env.get("x")` — walks up the chain
- `env.set("x", val)` — sets in the current (innermost) scope
- `env.child()` — creates a new scope inheriting parent
- `env.set_root(key, val)` — sets at the outermost (global) scope

---

## 3. The Evaluation Pipeline (In Detail)

### 3.1 Source to Evaluator

```rust
// In lib.rs or cli.rs:
fn evaluate(source: &str, env: &Env) -> Result<Value, EvalError> {
    let tokens = lexer::tokenize(source)?;          // Vec<SpannedToken>
    let exprs = parser::parse(tokens)?;             // Vec<Expr>
    let mut result = Value::Null;
    for expr in exprs {
        result = eval::eval(&expr, env)?;            // Value
    }
    Ok(result)
}
```

### 3.2 eval() Dispatch

`eval()` in `eval/mod.rs` dispatches on `Expr` variants. The main cases:

```rust
pub fn eval(expr: &Expr, env: &Env) -> Result<Value, EvalError> {
    match expr {
        // Atoms pass through
        Expr::Integer(n) => Ok(Value::Integer(n.clone())),
        Expr::Symbol(s) => eval_symbol(s, env),  // lookup in env

        // Call is the most complex — see §3.3
        Expr::Call { head, args } => eval_call(head, args, env),

        // Special forms with special evaluation rules
        Expr::If { condition, then_branch, else_branch } => eval_if(...),
        Expr::FuncDef { name, params, body, delayed } => eval_funcdef(...),
        Expr::Assign { lhs, rhs } => eval_assign(...),
        Expr::ReplaceAll { expr, rules } => eval_replace_all(...),

        // Lists evaluate each element
        Expr::List(items) => {
            let vals: Result<Vec<_>, _> = items.iter().map(|i| eval(i, env)).collect();
            Ok(Value::List(vals?))
        },
        // ...
    }
}
```

### 3.3 eval_call() — Function Application

The heart of the evaluator:

```rust
fn eval_call(head: &Expr, args: &[Expr], env: &Env) -> Result<Value, EvalError> {
    // 1. Evaluate the head to get a callable value
    let head_val = eval(head, env)?;

    // 2. Evaluate arguments (unless the function holds them)
    let evaled_args: Vec<Value> = args.iter().map(|a| eval(a, env)).collect::<Result<_, _>>()?;

    // 3. Apply the function
    apply_function(&head_val, &evaled_args, env)
}
```

### 3.4 apply_function() — Function Dispatch

Dispatches on the callable value type:

```rust
pub fn apply_function(func: &Value, args: &[Value], env: &Env) -> Result<Value, EvalError> {
    match func {
        Value::Builtin(name, builtin_fn) => builtin_fn(args),  // direct Rust call
        Value::Function { params, body, env: def_env } => {
            // Pattern-match args against params
            try_match_params(params, args, body, def_env)
        }
        Value::PureFunction { body, env: def_env } => {
            // Bind slots (#, #1, #2) and evaluate body
            apply_pure_function(body, args, def_env)
        }
        Value::Symbol(name) => {
            // Undefined symbol → return Call { head: name, args }
            Ok(Value::Call { head: name.clone(), args: args.to_vec() })
        }
        // ... pattern, rule, ruleset, etc.
    }
}
```

### 3.5 Function Definitions Accumulate

```syma
f[x_] := x + 1;     // adds a Function to f's value
f[x_] := x * 2;     // adds ANOTHER Function to f's value

f[5]  // tries x+1 first; if it doesn't match OR returns $Failed, tries x*2
```

When a symbol is used as a function head and its value is a `Value::RuleSet`
(which wraps multiple `Function` values), the evaluator tries each definition
in order. This enables:

- **Default cases**: `f[0] := 1; f[n_] := n * f[n-1]`
- **Type dispatch**: `g[x_Integer] := ...; g[x_String] := ...`
- **Overriding**: later definitions take precedence (tried first)

---

## 4. Pattern Matching (`pattern.rs`)

The pattern engine handles Wolfram-compatible pattern matching.

### Supported Patterns

| Syntax | AST | Match Behavior |
|--------|-----|----------------|
| `_` | `Blank { type_constraint: None }` | Matches any single value |
| `_Integer` | `Blank { type_constraint: Some("Integer") }` | Matches any Integer |
| `x_` | `NamedBlank { name: "x", type_constraint: None }` | Matches any, binds to `x` |
| `x_Integer` | `NamedBlank { name: "x", type_constraint: Some("Integer") }` | Matches Integer, binds to `x` |
| `__` | `BlankSequence { name: None, .. }` | Matches 1+ values, splats as Sequence |
| `___` | `BlankNullSequence { .. }` | Matches 0+ values |
| `x_ .` | `OptionalNamedBlank { .. }` | Matches with Null default |
| `a | b` | `Alternatives(a, b)` | Matches a OR b |
| `pat /; cond` | `PatternGuard { pattern, condition }` | Matches `pat` if `cond` is true |

### How Matching Works

The `match_value` function takes a pattern expression, a value, and optional
existing bindings:

```rust
fn match_value(
    pattern: &Expr,       // the pattern (e.g. Expr::NamedBlank { name: "x", .. })
    value: &Value,        // the value to match against (e.g. Value::Integer(5))
    bindings: &mut HashMap<String, Value>,  // accumulated bindings
) -> bool
```

**NullSequence matching** is handled specially: `___` can match zero values, and
`match_value` returns `Some(vec![])` to indicate a zero-length match that the
caller must handle (e.g., adjust argument list position).

### Rule Application (`eval/rules.rs`)

`ReplaceAll` (`/.`) and `ReplaceRepeated` (`//.`) are handled in `eval/rules.rs`:

```rust
pub fn replace_all(value: &Value, rules: &[Value], env: &Env) -> Result<Value, EvalError>
pub fn replace_repeated(value: &Value, rules: &[Value], env: &Env) -> Result<Value, EvalError>
```

The `Replacer` struct walks the value tree bottom-up, applying rules at each
node. For each rule (Rule or RuleSet), it calls `match_value` to check if the
pattern matches the current sub-expression, then substitutes the result.

**Limitation**: Guard conditions (`/;`) in patterns are parsed but the condition
is not yet evaluated during matching.

---

## 5. How To Add a Builtin

### Step 1: Create the function

Find or create the appropriate file in `src/builtins/`. The function signature
is always:

```rust
pub fn builtin_magic(args: &[Value]) -> Result<Value, EvalError> {
    // Validate args
    // Compute result
    // Return Ok(value) or Err(...)
}
```

Argument validation patterns:

```rust
fn builtin_example(args: &[Value]) -> Result<Value, EvalError> {
    // Fixed arity
    if args.len() != 2 {
        return Err(EvalError::Error("Example requires 2 arguments".into()));
    }

    // Type-checking
    let s = match &args[0] {
        Value::Str(s) => s.clone(),
        other => return Err(EvalError::TypeError {
            expected: "String".to_string(),
            got: other.type_name().to_string(),
        }),
    };

    // Multiple type handling
    let n = if let Value::Integer(i) = &args[1] {
        i.to_f64()
    } else if let Value::Real(r) = &args[1] {
        r.to_f64()
    } else {
        return Err(EvalError::TypeError { ... });
    };

    // ...
    Ok(result)
}
```

### Step 2: Register it

In `src/builtins/mod.rs`, add to `register_builtins()`:

```rust
pub fn register_builtins(env: &Env) {
    // ... existing registrations ...
    register_builtin(env, "Example", builtin_example);
}
```

The `register_builtin` helper creates a `Value::Builtin` wrapper and stores
it in the environment.

### Step 3: Add help text

Also in `mod.rs`, add to the help system:

```rust
"Example" => "Example[x, y] — does something amazing. (Experimental.)",
```

### Step 4: Add tests

Tests go inline at the bottom of the builtin file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_builtin_example_basic() {
        let result = builtin_example(&[
            Value::Str("hello".into()),
            Value::Integer(rug::Integer::from(42)),
        ]);
        assert!(result.is_ok());
        // assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_builtin_example_errors() {
        assert!(builtin_example(&[]).is_err());
        assert!(builtin_example(&[Value::Str("only".into())]).is_err());
    }
}
```

### Step 5: Run tests

```bash
cargo test builtin_example          # Just your new tests
cargo test --locked                  # Full suite
cargo clippy -- -D warnings         # No warnings
```

---

## 6. How To Add a Language Feature

Adding a new language feature touches four files in sequence:

### 6.1 Lexer (`lexer.rs`)

1. Add a new `Token` variant if you need a new operator or keyword:

```rust
// In the Token enum
pub enum Token {
    // ...
    MyKeyword,  // new keyword
    MyOp,       // new operator
}
```

2. Add the token to `Display` for error messages:

```rust
Token::MyKeyword => write!(f, "MyKeyword"),
Token::MyOp => write!(f, "~>"),  // or whatever the syntax is
```

3. Add recognition in the lexer's main loop:

```rust
// For multi-char operators: add before single-char checks
// Follow the maximal-munch pattern (longer matches first)
if self.remaining() >= 3 && self.src[self.pos..].starts_with("~~>") {
    self.pos += 3;
    push!(Token::MyOp);  // actually "~~>" is probably not what you want
}
```

### 6.2 AST (`ast.rs`)

Add a new `Expr` variant:

```rust
pub enum Expr {
    // ... existing variants ...
    MyFeature {
        target: Box<Expr>,
        body: Box<Expr>,
    },
}
```

Then implement `PartialEq` and `Display` for the new variant in the existing
match blocks at the bottom of the file.

### 6.3 Parser (`parser.rs`)

Add parsing logic. Where to add it depends on the syntax:

- **New operator**: Add precedence and parsing in the precedence climber section
- **New keyword form**: Add a parse method and call it from the primary expression parser
- **New sugar**: Can be desugared in the parser (e.g., `a - b` → `Plus[a, Times[-1, b]]`)

Example — adding a keyword form:

```rust
// In the primary expression parser:
fn parse_primary(&mut self) -> Result<Expr, ParseError> {
    match self.peek() {
        Token::MyKeyword => self.parse_my_feature(),
        // ...
    }
}

fn parse_my_feature(&mut self) -> Result<Expr, ParseError> {
    self.advance(); // consume the keyword
    let target = self.parse_expr()?;
    // parse the body...
    Ok(Expr::MyFeature { target: Box::new(target), body: Box::new(body) })
}
```

### 6.4 Evaluator (`eval/mod.rs`)

Add a case in `eval()`:

```rust
Expr::MyFeature { target, body } => {
    let target_val = eval(target, env)?;
    let body_val = eval(body, env)?;
    // ... do the feature's thing ...
    Ok(result)
}
```

### 6.5 Test

Add parser tests in `parser.rs` and evaluator tests in `eval/mod.rs` or inline.

---

## 7. How To Add an Import/Export Format

### Import (parsing a file into a Value)

In `src/builtins/io/import.rs`, add an extension match arm:

```rust
} else if path.ends_with(".csv") {
    let parsed = csv_to_value(&contents)
        .map_err(|e| EvalError::Error(format!("Import CSV error: {}", e)))?;
    Ok(parsed)
```

Then implement the converter function (either inline or in a new module).

### Export (serializing a Value to a file)

In `src/builtins/io/export.rs`, add an extension match arm before the default
text fallback.

### Example: adding TOML support

```rust
// In import.rs:
} else if path.ends_with(".toml") {
    let parsed = toml_to_value(&contents)
        .map_err(|e| EvalError::Error(format!("Import TOML error: {}", e)))?;
    Ok(parsed)
}
```

---

## 8. Project Idioms and Patterns

### EvalError

Two forms:

```rust
EvalError::Error(String)             // Generic error message
EvalError::TypeError { expected: String, got: String }  // Type mismatch
```

### Type Checking via type_name()

Every `Value` has a `type_name()` method — use it in error messages:

```rust
return Err(EvalError::TypeError {
    expected: "String".to_string(),
    got: args[0].type_name().to_string(),
});
```

### Integer Operations

Use `rug::Integer` (not `i64`):

```rust
use rug::Integer;
Value::Integer(Integer::from(42))
```

### Floats

Use `rug::Float` with `DEFAULT_PRECISION`:

```rust
use crate::value::DEFAULT_PRECISION;
use rug::Float;
Value::Real(Float::with_val(DEFAULT_PRECISION, 3.14))
```

### Display for Values

`Value` implements `Display` using the input form format. Use `format!("{}", val)`
for user-facing output.

### Sequence Auto-Splatting

`Value::Sequence(...)` auto-splats into lists and function calls. This is how
`BlankSequence` bindings work — a pattern like `x__` matches 1+ values and
binds to `Value::Sequence`. The evaluator splats it into `List` and `Call`
construction via `flatten_sequence_calls()` and `flatten_sequence_items()`.

### Pattern Storage

Patterns in rules and function definitions are stored as `Value::Pattern(Expr)`.
This wraps the raw AST so it can be pattern-matched at runtime. The pattern
engine's `match_value()` method takes `&Expr` (the pattern) and `&Value`
(the runtime value to match against).

### The `struct_eq` method on Value

`Value` has custom `PartialEq` that treats `Integer(1)` and `Real(1.0)` as
different. For structural equality (same shape regardless of numeric type),
use `value.struct_eq(other)` which does a deeper comparison.

---

## 9. Debugging Tips

### 9.1 Check the AST

```bash
cargo run -- -e "1 + 2 * 3"     # just evaluates
cargo run -- --check "a + b"    # parse only, show errors
```

### 9.2 Enable Trace Logging

Set `RUST_LOG=debug` to see evaluation traces if logging is wired in:

```bash
RUST_LOG=debug cargo run -- -e "1 + 2"
```

### 9.3 DAP Debug Mode

```bash
cargo run -- --dap <file.syma>
```

Connects to a DAP-compatible debugger. See `src/debug.rs`.

### 9.4 Kernel Mode

```bash
cargo run -- --kernel
```

Read-eval-print loop over JSON lines on stdin/stdout. Used by IDE integration.

### 9.5 Inspect Pattern Matching

If a pattern isn't matching, the issue is usually in `pattern.rs`'s `match_value`
function. Add `eprintln!` debugging or write a focused test case:

```rust
#[test]
fn debug_pattern() {
    use crate::pattern::match_value;
    let pattern = Expr::NamedBlank {
        name: "x".into(),
        type_constraint: Some("Integer".into()),
    };
    let value = Value::Integer(Integer::from(5));
    let mut bindings = HashMap::new();
    assert!(match_value(&pattern, &value, &mut bindings));
    assert_eq!(bindings["x"], Value::Integer(Integer::from(5)));
}
```

---

## 10. Testing Conventions

| Test Type | Location | Run Command |
|-----------|----------|-------------|
| Unit tests | `#[cfg(test)] mod tests` at bottom of each `.rs` file | `cargo test <module_name>` |
| Integration tests | `syma/tests/cli.rs` | `cargo test --test cli` |
| Example files | `syma/examples/*.syma` (run by integration tests) | `cargo test --test cli` |

### Writing Integration Tests

Integration tests in `tests/cli.rs` run `.syma` files and check output:

```rust
#[test]
fn test_run_basics_example() {
    let output = run_syma_file("examples/basics/hello.syma");
    assert!(output.status.success());
    assert!(stdout_contains(&output, "Hello"));
}
```

### Testing Conventions

- **Unit tests first**: Test the function directly in its module
- **Error paths**: Always test error cases (wrong arg count, wrong types)
- **Round-trips**: For serialization, test write → read → compare
- **Pattern matching**: Test both match and non-match cases
- **Edge cases**: Empty lists, null values, nested structures

---

## 11. Common Pitfalls

### 11.1 `PartialEq` on Expr

`Expr`'s `PartialEq` implementation at the bottom of `ast.rs` is hand-written
(macro-free). If you add a new `Expr` variant, you **must** add cases to:
- The `PartialEq` impl (missing it = compiler error)
- The `Display` impl (missing it = match warning or panic in tests)

### 11.2 Pattern Matching Guards

Guard conditions (`/;`) are parsed, stored in the AST as `PatternGuard`, but
the guard expression is NOT yet evaluated during matching. If you add a
pattern guard:
- The pattern part will match correctly
- The guard condition will be ignored during matching

### 11.3 Sequence Handling

`Value::Sequence` has special auto-splatting behavior:
- **Lists**: `List[Sequence[1, 2], 3]` → `{1, 2, 3}`
- **Calls**: `f[Sequence[1, 2], 3]` → `f[1, 2, 3]`

This means if you construct a `List` or `Call` manually in a builtin, you need
to handle `Sequence` values by calling `flatten_sequence_items()` or
`flatten_sequence_calls()`.

### 11.4 `rug::Integer` vs Built-in Integers

All integer arithmetic uses `rug::Integer` (GMP-backed arbitrary precision).
You cannot use `i32` or `i64` as `Value::Integer`. Use `Integer::from(n)` to
wrap.

### 11.5 Undefined Symbols Return Themselves

If a symbol isn't defined, `eval_symbol` returns `Value::Symbol(name)` rather
than an error. This means `undefinedFunction[1, 2]` returns as a symbolic
`Call` — it doesn't error. This is by design (Wolfram-compatible symbolic
evaluation).

### 11.6 Function Definition Accumulation

Multiple `f[x_] := ...` statements create a `RuleSet` with multiple entries.
The evaluator tries **each** definition in order until one matches. This is
different from simple assignment, where later `x = val` replaces the earlier
value.

### 11.7 `Register_builtins` Order

In `builtins/mod.rs`, the order of `register_builtin` calls matters —
functions registered later override earlier ones for the same name. Currently
no overrides exist, but this matters if you add fallback or conditional logic.

### 11.8 String Concat Operator

`<>` is `StringJoinOp` in the lexer. `"a" <> "b"` desugars to
`StringJoin["a", "b"]`. The `StringJoin` builtin handles both explicit calls
and the infix form.

### 11.9 Subtractor Design

`a - b` parses as `Plus[a, Times[-1, b]]`, not as a separate `Minus` node.
Subtraction is syntactic sugar. Unary `-x` is `Times[-1, x]`.

---

## 12. Performance Notes

- **Tree-walk interpreter**: Currently Phase 1 — no bytecode or JIT. Performance
  is O(expression depth) per evaluation.
- **Pattern matching is recursive**: Deeply nested patterns can cause stack
  recursion. Be mindful of very large expressions.
- **`rug` operations**: GMP integers are fast for large numbers but the
  boxing/unboxing overhead for small integers is non-trivial.
- **`Clone` is used freely**: `Value` and `Expr` both derive `Clone` and it's
  used throughout. Large expressions cloned repeatedly will be slow.

---

## 13. Reading Order

If you're new to the codebase, read the files in this order:

1. **`value.rs`** — Understand the runtime types (the "output" of the pipeline)
2. **`ast.rs`** — Understand the AST nodes (the "intermediate representation")
3. **`lexer.rs`** — How source text becomes tokens
4. **`parser.rs`** — How tokens become AST
5. **`eval/mod.rs`** — How AST becomes Value (the heart of the system)
6. **`env.rs`** — Variable scoping
7. **`pattern.rs`** — Pattern matching engine
8. **`builtins/mod.rs`** — Builtin registration orchestration
9. **`eval/rules.rs`** — Rule application (the other side of patterns)
10. **`builtins/arithmetic.rs`** — A simple builtin module
11. **`builtins/symbolic.rs`** — Complex builtins (differentiation, integration)
12. **`cli.rs`** — CLI and REPL entry point
