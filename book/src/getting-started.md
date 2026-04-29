# Getting Started

## Installation

Build and install Syma from source:

```bash
git clone https://github.com/syma-project/syma
cd syma
cargo xtask install --release
```

This installs the `syma` binary to `~/.syma/bin/syma`.

## Running the REPL

Launch the interactive REPL:

```bash
syma
```

You'll see a prompt where you can type expressions:

```syma
In[1]: 1 + 2
Out[1]= 3

In[2]: Sqrt[144]
Out[2]= 12

In[3]: hello[x_] := "Hello, " <> x
In[4]: hello["Syma"]
Out[4]= Hello, Syma
```

## Running Files

Save a `.syma` file and run it:

```bash
syma hello.syma
```

Or evaluate a single expression:

```bash
syma -e "N[Pi, 20]"
```

## Syntax at a Glance

| Feature | Syntax | Example |
|---------|--------|---------|
| Comment | `(* ... *)` | `(* nestable *)` |
| Statement separator | `;` | `a = 1; b = 2` |
| List | `{ ... }` | `{1, 2, 3}` |
| Function def | `f[x_] := body` | `square[x_] := x^2` |
| Rule | `->` | `x_ -> x^2` |
| Replace all | `/.` | `{1,2,3} /. x_ -> x^2` |
| Map | `/@` | `Sqrt /@ {1,4,9}` |
| Apply | `@@` | `Plus @@ {1,2,3}` |
| Postfix | `//` | `25 // Sqrt` |
| Pure function | `#^2 &` | `Map[#^2 &, {1,2,3}]` |
| String concat | `<>` | `"a" <> "b"` |
| If | `If[cond, t, f]` | `If[x > 0, "pos", "neg"]` |
| Module | `module Name { ... }` | `module Math { ... }` |
| Class | `class Name { ... }` | `class Point { ... }` |

## Next Steps

- [Basics](basics.md) — atoms, arithmetic, variables, strings
- [Functions](functions.md) — defining and using functions
- [Lists](lists.md) — working with collections
- [Control Flow](control-flow.md) — conditionals and comparisons
