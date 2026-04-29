# Language Specification

This chapter summarizes the complete Syma language. For the authoritative specification, see [`syma/syma-lang.md`](https://github.com/syma-project/syma/blob/main/syma/syma-lang.md) in the repository.

## Lexical Structure

### Comments
```
(* nestable comment *)
```

### Atoms
```
42           (* integer *)
3.14         (* real number *)
"hello"      (* string *)
True         (* boolean *)
False
Null         (* null value *)
Pi           (* constant π *)
E            (* constant e *)
```

### Identifiers
Identifiers are alphanumeric: `[a-zA-Z_][a-zA-Z0-9_]*`. Pattern blanks like `x_` and `x_Integer` are lexed as single tokens.

## Operators (by precedence, low to high)

| Precedence | Operators | Assoc | Description |
|-----------|-----------|-------|-------------|
| 1 | `//` | left | Postfix pipe |
| 2 | `//.` | left | Replace repeated |
| 3 | `/.` | right | Replace all |
| 4 | `@` | right | Prefix application |
| 5 | `@@` | right | Apply |
| 6 | `/@` | right | Map |
| 7 | `->` `:>` | right | Rule (immediate/delayed) |
| 8 | `\|\|` | left | Logical or |
| 9 | `&&` | left | Logical and |
| 10 | `==` `!=` `<` `>` `<=` `>=` | left | Comparison |
| 11 | `+` `-` `<>` | left | Addition, string concat |
| 12 | `*` `/` | left | Multiplication |
| 13 | `^` | right | Power |
| 14 | `!` `-` | right | Unary not/negate |
| 15 | `(` `)` `[` `]` `{` `}` | — | Primary |

Operators are desugared to `Call` nodes: `a + b` → `Plus[a, b]`.

## Patterns

```
_              (* blank — matches anything *)
x_             (* named blank — matches and binds to x *)
_Integer       (* type-constrained blank *)
x_Integer      (* named + type-constrained *)
__             (* one or more elements *)
___            (* zero or more elements *)
pattern | pattern  (* alternatives *)
pattern /; condition  (* guard *)
```

## Function Definition

```
(* Delayed — RHS evaluated on each call *)
f[x_] := x^2

(* Immediate — RHS evaluated at definition time *)
f[x_] = x^2

(* C-style def *)
def f(x) := x^2

(* Pure function (lambda) *)
#^2 &
```

Multiple definitions coexist and are tried in order. More specific patterns match first.

## Control Flow

```
If[condition, then, else]
Which[cond1, val1, cond2, val2, ..., True, default]
```

## Assignment

```
x = value              (* immediate assignment *)
x := expression        (* delayed assignment *)
x += n                 (* increment *)
x -= n                 (* decrement *)
```

## Data Structures

```
{1, 2, 3}              (* List — core data structure *)
<|"a" -> 1, "b" -> 2|> (* Association — hash map *)
```

## Modules

```
module Name {
    export sym1, sym2
    (* private definitions *)
}

import Name                    (* import all exports *)
import Name.{sym1}             (* selective import *)
import Name as Alias           (* aliased import *)
```

## OOP Classes

```
class Name {
    field name: Type          (* required field *)
    field name: Type = default  (* field with default *)

    constructor[arg_, ...] {
        this.field = arg
    }

    method name[args_] := body
}

class Child extends Parent {
    (* inheritance *)
}

class Name with Mixin1, Mixin2 {
    (* mixins *)
}
```

## Standard Library

### Arithmetic
`Plus`, `Times`, `Power`, `Divide`, `Minus`, `Abs`

### Comparison
`Equal`, `Unequal`, `Less`, `Greater`, `LessEqual`, `GreaterEqual`

### Logical
`And`, `Or`, `Not`

### List Operations
`Length`, `First`, `Last`, `Rest`, `Most`, `Append`, `Prepend`, `Join`, `Reverse`, `Sort`, `Flatten`, `Part`, `Range`, `Table`, `Map`, `Fold`, `Select`, `Scan`, `Nest`, `Take`, `Drop`, `Total`, `MemberQ`, `Count`, `Position`, `Union`, `Intersection`, `Complement`, `Tally`, `PadLeft`, `PadRight`, `Riffle`, `Transpose`

### String
`StringJoin`, `StringLength`, `ToString`, `ToExpression`, `StringSplit`, `StringReplace`, `StringTake`, `StringDrop`, `StringContainsQ`, `StringReverse`, `ToUpperCase`, `ToLowerCase`, `Characters`, `StringMatchQ`, `StringPadLeft`, `StringPadRight`, `StringTrim`, `StringStartsQ`, `StringEndsQ`

### Mathematics
`Sin`, `Cos`, `Tan`, `ArcSin`, `ArcCos`, `ArcTan`, `Log`, `Log2`, `Log10`, `Exp`, `Sqrt`, `Floor`, `Ceiling`, `Round`, `Max`, `Min`, `Mod`, `GCD`, `LCM`, `Factorial`, `N`

### Symbolic
`Expand`, `Factor`, `D` (differentiation), `Integrate`, `Simplify`, `Solve`, `Series`

### Pattern
`MatchQ`, `Head`, `TypeOf`, `FreeQ`

### Association
`Keys`, `Values`, `Lookup`, `KeyExistsQ`

### I/O
`Print`, `Input`, `Write`, `WriteLine`, `Import`, `Export`, `ReadString`, `WriteString`

### Random
`RandomInteger`, `RandomReal`, `RandomChoice`
