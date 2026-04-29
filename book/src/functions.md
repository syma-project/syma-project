# Functions

## Defining Functions

Use `:=` (delayed evaluation) to define functions:

```syma
square[x_] := x^2;
square[5]      (* => 25 *)

add[a_, b_] := a + b;
add[3, 4]      (* => 7 *)
```

The `x_` syntax is a **pattern blank** — it matches any value and binds it to `x`.

## Multiple Definitions (Pattern Dispatch)

Define specific cases, then a general case. Syma tries definitions in order:

```syma
fib[0] := 0;
fib[1] := 1;
fib[n_] := fib[n - 1] + fib[n - 2];
fib[10]        (* => 55 *)

fact[0] := 1;
fact[n_] := n * fact[n - 1];
fact[5]        (* => 120 *)
```

## Type-Constrained Patterns

Restrict arguments to specific types:

```syma
f[x_Integer] := x * 2;
f[5]           (* => 10 *)
(* f["hello"] stays unevaluated — no matching definition *)
```

## Pure Functions (Lambdas)

Use `#` for arguments and `&` to mark a pure function:

```syma
#^2 &
```

Apply with `Map` or the `/@` operator:

```syma
Map[#^2 &, {1, 2, 3}]   (* => {1, 4, 9} *)
# + 1 & /@ {1, 2, 3}     (* => {2, 3, 4} *)
```

## Application Syntax

Syma offers several ways to apply functions:

| Syntax | Meaning | Example |
|--------|---------|---------|
| `f[x]` | Standard | `Sin[Pi/2]` |
| `f @ x` | Prefix | `Length @ {1,2,3}` |
| `x // f` | Postfix | `{1,2,3} // Length` |
| `f /@ list` | Map | `Sqrt /@ {1,4,9}` |
| `f @@ list` | Apply | `Plus @@ {1,2,3}` |

```syma
Length @ {1, 2, 3, 4, 5}     (* => 5 *)
{1, 2, 3, 4, 5} // Length    (* => 5 *)
Sqrt /@ {1, 4, 9, 16}        (* => {1, 2, 3, 4} *)
Plus @@ {1, 2, 3, 4, 5}      (* => 15 *)
```
